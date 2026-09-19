//! A/B rustls

use crate::*;
use blueprint::BluePrint;
use blueprint::Orbit;

use blueprint_rustls::{TlsServer as RustlsServer, TlsServerConfig as RustlsServerConfig};

use std::path::Path;

fn load_config() -> RustlsServerConfig {
    RustlsServerConfig::with_certs_and_key_file(Path::new(CA), Path::new(CERT), Path::new(KEY))
        .unwrap()
}

pub fn init_server() -> impl Orbit {
    let tls_config_server = load_config();
    RustlsServer::with_configuration(tls_config_server).unwrap()
}
