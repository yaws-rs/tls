//! TLS Entities

use crate::TlsError;

/// .
pub struct TlsClientConfig {
}

/// .
pub struct TlsClient {
}

impl TlsClient {
    /// Construct new
    pub fn with_config(c: TlsClientConfig) -> Result<Self, TlsError> {
//        rustls::client::UnbufferedClientConnection::new(c.into());
        todo!()
    }
}
