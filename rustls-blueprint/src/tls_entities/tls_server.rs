//! TLS Entities

mod tls_server_config;
pub use tls_server_config::TlsServerConfig;

use crate::Arc;
use crate::TlsError;
use crate::TlsPosition;

use crate::rustls::CtxRustls;

use rustls::server::ServerConfig as RustlsServerConfig;
use rustls::server::UnbufferedServerConnection as RustlsServerConnection;

use rustls::unbuffered::ConnectionState as RustlsConnectionState;
use rustls::unbuffered::UnbufferedStatus as RustlsUnbufferedStatus;

use blueprint::{Left, Right};

/// .
pub struct TlsServer {
    pub(crate) config: TlsServerConfig,
    pub(crate) rustls_config: RustlsServerConfig,
    pub(crate) rustls_server: RustlsServerConnection,
}

impl core::fmt::Debug for TlsServer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "TlsServer")
    }
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
    /// Advance the state machine
    #[inline]
    pub fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<TlsPosition, TlsError> {
        let mut ctx = CtxRustls::with_server(self);
        ctx.advance_with(_u, l, r)
    }
}
