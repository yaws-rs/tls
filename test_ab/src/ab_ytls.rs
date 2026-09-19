//! A/B yTLS

use crate::*;
use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint_ytls::{CryptoConfig, CryptoRng};
use blueprint_ytls::{TlsServer, TlsServerConfig, TlsServerCtxConfig};

fn load_config() -> TlsServerConfig {
    let ca_vec = load_pem_vec(CA);
    let cert_vec = load_pem_vec(CERT);
    let key_vec = load_pem_vec(KEY);

    let (_cert_type_label, cert_data) = pem_rfc7468::decode_vec(&cert_vec).unwrap();
    let (_key_type_label, key_data_der) = pem_rfc7468::decode_vec(&key_vec).unwrap();
    use sec1::EcPrivateKey;
    let key_info = EcPrivateKey::try_from(key_data_der.as_ref()).unwrap();
    let key_data = key_info.private_key.to_vec();
    let (_ca_type_label, ca_data) = pem_rfc7468::decode_vec(&ca_vec).unwrap();

    TlsServerConfig::with_ca_cert_key(&ca_data, &cert_data, &key_data).unwrap()
}

pub fn init_server() -> impl Orbit {
    let tls_config_server = load_config();
    TlsServer::with_configuration(tls_config_server).unwrap()
}

//pub struct YtlsServer;

/*
impl AbServerCtx for YtlsServer {
    fn init_server() -> Self {
        TlsServer::with_configuration(tls_config_server).unwrap()
    }
}
*/
