//! TLS Entities

mod tls_server_config;
pub use tls_server_config::TlsServerConfig;

use crate::TlsError;

use core::marker::PhantomData;

/// .
pub struct TlsServer<'c> {
    config: TlsServerConfig<'c>,
}

impl<'c> TlsServer<'c> {
    /// Construct new
    pub fn with_config(c: TlsServerConfig<'c>) -> Result<Self, TlsError> {
//        rustls::client::UnbufferedClientConnection::new(c.into());
        todo!()
    }
}
