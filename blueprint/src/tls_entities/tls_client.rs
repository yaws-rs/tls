//! TLS Entities

use crate::Arc;

mod tls_client_config;
pub use tls_client_config::TlsClientConfig;

use crate::TlsError;

use rustls::client::ClientConfig as RustlsClientConfig;
use rustls::client::UnbufferedClientConnection as RustlsClientConnection;

/// .
pub struct TlsClient<'c> {
    config: TlsClientConfig<'c>,
    rustls_ubuffered_connection: RustlsClientConnection,
}

impl<'c> TlsClient<'c> {
    /// Construct new
    pub fn with_config(config: TlsClientConfig<'c>) -> Result<Self, TlsError> {
        let arc_config:  Arc<RustlsClientConfig> = Arc::new(config.clone().try_into()?);
        let rustls_ubuffered_connection =
            rustls::client::UnbufferedClientConnection::new(arc_config, config.rustls_server_name())
            .map_err(TlsError::RustlsConfig)?;
        Ok(Self { config, rustls_ubuffered_connection })
    }
}
