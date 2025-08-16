//! TLS Entities

use crate::Arc;

mod tls_client_config;
pub use tls_client_config::TlsClientConfig;

use crate::TlsError;

use rustls::client::ClientConfig as RustlsClientConfig;
use rustls::client::UnbufferedClientConnection as RustlsClientConnection;

use crate::tls_entities::{FakeServerCertVerifier, FakeTime};

use crate::tls_entities::TlsServerIdentifier;
use rustls_pki_types::DnsName as RustlsDnsName;

/// .
pub struct TlsClient {
    /// .
    config: TlsClientConfig,
    /// .
    rustls_config: RustlsClientConfig,
    /// .
    rustls_client: RustlsClientConnection,
}

use rustls_pki_types::ServerName as RustlsServerName;

impl TlsClient {
    /// Construct new
    pub fn with_config(config: TlsClientConfig) -> Result<Self, TlsError> {
        let rustls_config: RustlsClientConfig = config.clone().try_into()?;

        let rustls_client = RustlsClientConnection::new(
            Arc::new(rustls_config.clone()),
            config.server_identifier.clone().try_into()?,
        )
        .map_err(TlsError::RustlsConfig)?;

        Ok(Self {
            config,
            rustls_config,
            rustls_client,
        })
    }
}
