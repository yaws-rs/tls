//! TLS Entities

use crate::TlsError;

/// .
pub struct TlsServerConfig {
}

/// .
pub struct TlsServer {
}

impl TlsServer {
    /// Construct new
    pub fn with_config(c: TlsServerConfig) -> Result<Self, TlsError> {
//        rustls::client::UnbufferedClientConnection::new(c.into());
        todo!()
    }
}
