//! Tls Client Config

use rustls::client::ClientConfig as RustlsClientConfig;
use rustls_pki_types::ServerName as RustlsServerName;

use crate::TlsError;

use crate::TlsServerIdentifier;

/// .
#[derive(Clone, Debug)]
pub struct TlsClientConfig<'c> {
    server_identifier: TlsServerIdentifier<'c>,
}

impl<'c> TlsClientConfig<'c> {
    pub fn rustls_server_name(&self) -> RustlsServerName<'c> {
        self.server_identifier.into()
    }
}

impl TryFrom<TlsClientConfig<'_>> for RustlsClientConfig {
    type Error = TlsError;

    fn try_from(_: TlsClientConfig<'_>) -> Result<Self, Self::Error> {
        todo!()
    }    
}
