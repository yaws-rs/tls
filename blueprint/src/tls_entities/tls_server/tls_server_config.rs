//! Tls Server Config

use crate::TlsServerIdentifier;

///
#[derive(Clone, Debug)]
pub struct TlsServerConfig<'c> {
    server_identifier: TlsServerIdentifier<'c>,
}

impl<'c> TlsServerConfig<'c> {
}
