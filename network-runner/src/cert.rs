extern crate askama;
extern crate openssl;
extern crate chrono;

use std::collections::HashMap;
use openssl::x509::{ X509, X509Name };
use openssl::pkey::{ PKey, Private };
use openssl::hash::MessageDigest;
use openssl::rsa::Rsa;
use openssl::nid::Nid;
use openssl::asn1::Asn1Time;
use askama::Template;

use crate::node::*;
use crate::consts::*;

pub fn issue_credentials(node_name: &String) -> (Rsa<Private>, X509) {

    // issue private key
    let privkey = Rsa::generate(KEY_SIZE).unwrap();

    // extract a key-pair from private key
    let pkey = PKey::from_rsa(privkey.clone()).unwrap();

    // prepare parameters
    let mut name = X509Name::builder().unwrap();
    let cname = format!("{}.com", node_name);
    name.append_entry_by_nid( Nid::COMMONNAME, &cname ).unwrap();
    let name = name.build();
    let from = Asn1Time::days_from_now(0).unwrap();
    let to = Asn1Time::days_from_now(CERT_EXPIRATION).unwrap();

    // set parameters and build
    let mut builder = X509::builder().unwrap();
    builder.set_version(2).unwrap();
    builder.set_subject_name(&name).unwrap();
    builder.set_not_before(&from).unwrap();
    builder.set_not_after(&to).unwrap();
    builder.set_pubkey(&pkey).unwrap();
    builder.sign(&pkey, MessageDigest::sha256()).unwrap();

    (privkey, builder.build())
}

#[derive(Clone, Debug)]
struct RouterRecord {
    router_id :String,
    certificate :String,
    public_key :String,
    expiration :String,
}
impl RouterRecord {
    pub fn new(router_id: String, certificate: String, public_key: String, expiration: String) -> RouterRecord {
        RouterRecord {
            router_id:  router_id,
            certificate: certificate,
            public_key: public_key,
            expiration: expiration,
        }
    }
}
#[derive(Clone, Debug)]
struct PrefixRecord {
    router_id :String,
    prefix :String,
} 
impl PrefixRecord {
    pub fn new(router_id: String, prefix: String) -> PrefixRecord {
        PrefixRecord {
            router_id:  router_id,
            prefix: prefix,
        }
    }
}
#[derive(Template, Clone, Debug)]
#[template(path="insert_seed", print="none")]
struct InsertData {
    routers :Vec<RouterRecord>,
    prefixes :Vec<PrefixRecord>,
}

pub fn issue_seed_sql(cert_map: &HashMap<Node, X509>) -> String {
    // initialize
    let mut data = InsertData {
        routers: Vec::with_capacity(cert_map.len()),
        prefixes: Vec::with_capacity(cert_map.len()),
    };
    for (node, cert) in cert_map {

        // reformat Asn1Time to SQLite3 datetime
        let expiration = chrono::DateTime::parse_from_str(
            &cert.not_after().to_string().replace("GMT", "+0000"),
            ASN1TIME_FORMAT
        ).unwrap();

        // router record
        let router_record = RouterRecord::new(
            node.router_id().to_string(),
            String::from_utf8(cert.to_pem().unwrap()).unwrap(),
            String::from_utf8(cert.public_key().unwrap().public_key_to_pem().unwrap()).unwrap(),
            expiration.format(SQLITE_DATETIME_FORMAT).to_string()
        );
        data.routers.push(router_record);

        // all prefixes
        for prefix in node.enabled_prefixes() {
            let prefix_record = PrefixRecord::new(
                node.router_id().to_string(),
                prefix.to_string()
            );
            data.prefixes.push(prefix_record);
        }
    }

    data.render().unwrap()
}
