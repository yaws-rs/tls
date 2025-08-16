//! TLS Entities

mod tls_server_config;
pub use tls_server_config::TlsServerConfig;

use crate::Arc;
use crate::TlsError;

use rustls::server::ServerConfig as RustlsServerConfig;
use rustls::server::UnbufferedServerConnection as RustlsServerConnection;

/// .
pub struct TlsServer {
    config: TlsServerConfig,
    rustls_config: RustlsServerConfig,
    rustls_server: RustlsServerConnection,
}

impl TlsServer {
    /// Construct new
    pub fn with_config(config: TlsServerConfig) -> Result<Self, TlsError> {
        let rustls_config: RustlsServerConfig = config.clone().try_into()?;

        let rustls_server = RustlsServerConnection::new(Arc::new(rustls_config.clone()))
            .map_err(TlsError::RustlsConfig)?;

        Ok(Self {
            config,
            rustls_config,
            rustls_server,
        })
    }
}
