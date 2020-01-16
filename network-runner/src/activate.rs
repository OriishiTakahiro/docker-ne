extern crate askama;
extern crate openssl;

use std::fs;
use std::process::Command;
use std::collections::HashMap;
use std::path::Path;

use openssl::x509::X509;
use askama::Template;

use clap::{ ArgMatches };

// use original modules
use crate::node::*;
use crate::consts::*;
use crate::utils::*;
use crate::cert::*;

pub fn activate_network(nodes: Nodes, args: &ArgMatches) -> Result<Nodes, String>{

    let fluent_host = args.value_of("fhost").unwrap_or(DEFAULT_FLUENTD_HOST);
    let fluent_port = args.value_of("fport").unwrap_or(DEFAULT_FLUENTD_PORT);
    let network_name = Path::new( args.value_of("file").unwrap_or(DEFAULT_NETWORK_NAME) ).file_stem().unwrap().to_str().unwrap();

    // remove all conf file
    match fs::remove_dir_all( TMP_DIR ) {
        Ok(_) => (),
        Err(_) => (),
    };

    // store CA's information
    //let mut ca_node = Node::new();
    let mut ca_cert = String::from("");

    // store all certs
    let mut cert_store :HashMap<Node, X509> = HashMap::with_capacity( nodes.len() );

    for n in &nodes {

        // create directories for mout point
        fs::create_dir_all( get_filepath(n.name(), FileType::Conf, "") ).unwrap();
        fs::create_dir_all( get_filepath(n.name(), FileType::Cert, "") ).unwrap();
        fs::create_dir_all( get_filepath(n.name(), FileType::SQL, "") ).unwrap();
        fs::create_dir_all( get_filepath(n.name(), FileType::ExecBin, "") ).unwrap();

        // create config files for each node
        let conf_str = match n.render() {
            Ok(result) => result,
            Err(msg) => return Err(msg.to_string()),
        };

        write_file(
            get_filepath(n.name(), FileType::Conf, OSPF6D_CONF),
            conf_str
        );

        write_file( 
            get_filepath(n.name(), FileType::Conf, ZEBRA_CONF),
            format!(
                "hostname {}\npassword zebra\nenable password zebra\nlog stdout",
                n.name()
           )
        );

        // create private key and certificate for each node
        let (privkey, cert) = issue_credentials(&n.name());

        // save credentials to each mout point
        fs::write(
            &get_filepath(n.name(), FileType::Cert, OWN_PKEY),
            &privkey.private_key_to_pem().unwrap()
        ).unwrap();
        fs::write(
            &get_filepath(n.name(), FileType::Cert, OWN_CERT),
            &cert.to_pem().unwrap()
        ).unwrap();

        if n.is_registered() {
            cert_store.insert(n.clone(), cert);
        }

        // store CA's data
        if n.is_ca() {
            ca_cert = get_filepath(&n.name(), FileType::Cert, OWN_CERT);
            //ca_node = n.clone();
        };

    };

    // save CA's certificate to all nodes
    for n in &nodes {
        fs::copy(
            &ca_cert,
            &get_filepath(&n.name(), FileType::Cert, CA_CERT)
        ).unwrap();

        // issue SQL for inserting seed data
        write_file(
            get_filepath(&n.name(), FileType::SQL, SEED_DATA),
            issue_seed_sql(&cert_store)
        );
    };


    // create all switches
    for s in switch_set(&nodes) {
        let output = Command::new("ovs-vsctl").args(&[ "add-br", &s ]).output();
        let disp_msg = match output {
            Ok(ovs_out) => if ovs_out.status.success() {
                    format!("{} is activated!", s)
                } else {
                    format!("{} is failed to activated! ({})", s, String::from_utf8(ovs_out.stderr).unwrap())
                },
            Err(msg) => format!("{} is failed to activated! ({:?})", s, msg),
        };
        println!("{}", disp_msg);
    }

    // activate all nodes
    for n in &nodes {
        // run container
        let output = Command::new("docker")
            .args(&[
                "run",
                "-d",
                "--name",
                n.name(),
                "--net=none",
                "--sysctl",
                "net.ipv6.conf.all.disable_ipv6=0 ",
                "--sysctl",
                "net.ipv6.conf.all.forwarding=1",
                "--privileged",
                "-v",
                &format!("{}:{}:rw", get_filepath(n.name(), FileType::Conf, ""), MNT_QUAGGA),
                "-v",
                &format!("{}:{}:rw", get_filepath(n.name(), FileType::Cert, ""), MNT_CERT),
                "-v",
                &format!("{}:{}:rw", get_filepath(n.name(), FileType::SQL, ""), MNT_SQL),
                "-v",
                &format!("{}:{}:rw", get_filepath(n.name(), FileType::ExecBin, ""), MNT_BIN),
                "-e",
                &format!("OSPF6D_OPTS={}", if n.is_ca() {"-c"} else {"''"}),
                "--log-driver=fluentd",
                "--log-opt",
                &format!("fluentd-address={}:{}", fluent_host, fluent_port),
                "--log-opt", 
                &format!("tag=docker.{}.{}", network_name, "{{.Name}}"),
                n.image(),
            ]).output();

        println!("{}:{}", get_filepath(n.name(), FileType::SQL, ""), MNT_SQL);

        let disp_msg = match output {
            Ok(docker_out) => if docker_out.status.success() {
                    format!("{} is activated! id: {}", n.name(), String::from_utf8(docker_out.stdout).unwrap())
                } else {
                    format!("{} is failed to activated! ({:?})", n.name(), String::from_utf8(docker_out.stderr).unwrap())
                },
            Err(msg) => format!("{} is failed to activated! ({:?})", n.name(), msg),
        };
        println!("{}", disp_msg);

        // conntect containers and switches
        for iface in n.interfaces() {
            let output = Command::new("ovs-docker")
                .args(&[
                      "add-port",
                      iface.switch(),
                      iface.name(),
                      n.name(),
                      &format!("--ipaddress6={}", iface.prefix()),
                ]).output();

            let disp_msg = match output {
                Ok(ovs_out) => if ovs_out.status.success() {
                        format!("{} is connected with {}!", iface.switch(), n.name())
                    } else {
                        format!("{} is failed to connection! ({:?})", iface.switch(), String::from_utf8(ovs_out.stderr).unwrap())
                    },
                Err(msg) => format!("{} is failed to connection! ({:?})", iface.switch(), msg),
            };
            println!("{}", disp_msg);
        };
    }

    Ok(nodes)

}
