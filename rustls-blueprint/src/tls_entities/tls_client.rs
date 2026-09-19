//! TLS Entities

use crate::Arc;

mod tls_client_config;
pub use tls_client_config::TlsClientConfig;

use crate::TlsError;
use crate::TlsPosition;

use crate::rustls::CtxRustls;

use rustls::client::ClientConfig as RustlsClientConfig;
use rustls::client::UnbufferedClientConnection as RustlsClientConnection;

use rustls::unbuffered::ConnectionState as RustlsConnectionState;
use rustls::unbuffered::UnbufferedStatus as RustlsUnbufferedStatus;

use crate::tls_entities::FakeServerCertVerifier;

//TODO: no_std time wiring?
//#[cfg(not(feature = "std"))]
//use crate::tls_entities::FakeTime;

use crate::tls_entities::TlsServerIdentifier;
use rustls_pki_types::DnsName as RustlsDnsName;

use blueprint::{Left, Right};

/// Rustls Client
pub struct TlsClient {
    /// .
    #[allow(dead_code)]
    pub(crate) config: TlsClientConfig,
    /// .
    #[allow(dead_code)]
    pub(crate) rustls_config: RustlsClientConfig,
    /// .
    pub(crate) rustls_client: RustlsClientConnection,
}

impl core::fmt::Debug for TlsClient {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "TlsClient")
    }
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
    /// Advance the state machine
    #[inline]
    pub fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<TlsPosition, TlsError> {
        let mut ctx = CtxRustls::with_client(self);
        ctx.advance_with(_u, l, r)
    }
}
