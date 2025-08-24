//! Tick Tock simulator

use blueprint::{BluePrint, Orbit};
use yaws_tls_blueprint::{TlsClient, TlsServer};
use yaws_tls_blueprint::{TlsClientConfig, TlsServerConfig};

const CA: &'static str = "certs/ca.rsa4096.crt";
const CERT: &'static str = "certs/rustcryp.to.rsa4096.ca_signed.crt";
const KEY: &'static str = "certs/rustcryp.to.rsa4096.key";

use std::path::Path;

fn main() {
    let config_server =
        TlsServerConfig::with_certs_and_key_file(Path::new(CA), Path::new(CERT), Path::new(KEY))
            .unwrap();
    let config_client = TlsClientConfig::with_hostname("localhost").unwrap();

    let server = TlsServer::with_config(config_server);
    let client = TlsClient::with_config(config_client);
}
