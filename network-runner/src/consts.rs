pub const DEFAULT_FLUENTD_HOST: &str    = "localhost";
pub const DEFAULT_FLUENTD_PORT: &str    = "24224";
pub const DEFAULT_NETWORK_NAME: &str    = "default";

// ------------- For temporary files --------------- //

pub const NETWORK_STATE_FILE: &str = "/tmp/network_runner/network_state.yml";

// --- For quagga configurations and mount points ---//

// Directory for mout to containers
pub const TMP_DIR: &'static str = "/tmp/network_runner";

// Mountpoint in Container
pub const MNT_QUAGGA: &'static str = "/usr/local/etc/";
pub const MNT_CERT: &'static str = "/var/quagga/cert/";
pub const MNT_SQL: &'static str = "/var/quagga/sql/";
pub const MNT_BIN: &'static str = "/var/quagga/bin/";

// Configuration files
pub const OSPF6D_CONF: &'static str = "ospf6d.conf";
pub const ZEBRA_CONF: &'static str = "zebra.conf";

// Private key and Certificate
pub const OWN_PKEY: &'static str = "privkey.pem";
pub const OWN_CERT: &'static str = "cert.pem";
pub const CA_CERT: &'static str = "ca_cert.pem";

// Private key and Certificate
pub const SEED_DATA: &'static str = "insert_seed.sql";

// parameters for credentials
pub const KEY_SIZE: u32 = 2048;
pub const CERT_EXPIRATION: u32 = 3650;

// ---------------- For SQLite3 -------------------- //

// Asn1Time format
pub const ASN1TIME_FORMAT: &'static str = "%b  %e %T %Y %z";
pub const SQLITE_DATETIME_FORMAT: &'static str = "%Y-%m-%d %T";

// ------------------------------------------------- //

// Directory Prefix
pub enum FileType {
    Conf,
    Cert,
    SQL,
    ExecBin,
}
