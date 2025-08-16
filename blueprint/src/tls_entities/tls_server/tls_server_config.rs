//! Tls Server Config

use crate::Arc;
use crate::TlsError;
use crate::TlsServerIdentifier;
use crate::Vec;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};

use crate::tls_entities::tls_server::RustlsServerConfig;

use crate::FakeTime;

///
#[derive(Debug)]
pub struct TlsServerConfig {
    server_identifier: TlsServerIdentifier,
    rustls_cert_chain: Vec<CertificateDer<'static>>,
    // TODO: don't stick to this being here. just testing for now.
    rustls_key_der: PrivateKeyDer<'static>,
}

impl Clone for TlsServerConfig {
    fn clone(&self) -> Self {
        Self {
            server_identifier: self.server_identifier.clone(),
            rustls_cert_chain: self.rustls_cert_chain.clone(),
            rustls_key_der: self.rustls_key_der.clone_key(),
        }
    }
}

impl TlsServerConfig {}

impl TryFrom<TlsServerConfig> for RustlsServerConfig {
    type Error = TlsError;

    fn try_from(c: TlsServerConfig) -> Result<Self, Self::Error> {
        // TODO: time in no_std
        let fake_time = FakeTime {};

        #[cfg(feature = "std")]
        let rustls_config =
            RustlsServerConfig::builder_with_provider(Arc::new(rustls_rustcrypto::provider()));

        #[cfg(not(feature = "std"))]
        let rustls_config = RustlsServerConfig::builder_with_details(
            Arc::new(rustls_rustcrypto::provider()),
            Arc::new(fake_time),
        );

        let rustls_config = rustls_config
            .with_safe_default_protocol_versions()
            .map_err(TlsError::RustlsConfig)?;

        let rustls_config = rustls_config.with_no_client_auth();

        let rustls_config = rustls_config
            .with_single_cert(c.rustls_cert_chain.clone(), c.rustls_key_der.clone_key())
            .map_err(TlsError::RustlsConfig)?;

        Ok(rustls_config)
    }
}
