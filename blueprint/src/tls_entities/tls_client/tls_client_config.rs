//! Tls Client Config

use rustls::client::ClientConfig as RustlsClientConfig;
use rustls_pki_types::ServerName as RustlsServerName;

use crate::Arc;
use crate::TlsError;

use crate::TlsServerIdentifier;

use crate::tls_entities::{FakeServerCertVerifier, FakeTime};

/// .
#[derive(Clone, Debug)]
pub struct TlsClientConfig {
    ///
    pub server_identifier: TlsServerIdentifier,
}

impl TlsClientConfig {}

impl TryFrom<TlsClientConfig> for RustlsClientConfig {
    type Error = TlsError;

    fn try_from(_: TlsClientConfig) -> Result<Self, Self::Error> {
        let fake_time = FakeTime {};
        let fake_server_cert_verifier = FakeServerCertVerifier {};

        #[cfg(feature = "std")]
        let rustls_config = rustls::client::ClientConfig::builder_with_provider(Arc::new(
            rustls_rustcrypto::provider(),
        ));

        #[cfg(not(feature = "std"))]
        let rustls_config = rustls::client::ClientConfig::builder_with_details(
            Arc::new(rustls_rustcrypto::provider()),
            Arc::new(fake_time),
        );

        let rustls_config = rustls_config
            .with_safe_default_protocol_versions()
            .map_err(TlsError::RustlsConfig)?;

        let rustls_config = rustls_config
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(fake_server_cert_verifier));

        Ok(rustls_config.with_no_client_auth())
    }
}
