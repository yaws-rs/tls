//! Common TLS Entities

use crate::TlsError;

use core::net::IpAddr;

use rustls_pki_types::ServerName as RustlsServerName;

/// .
#[derive(Clone, Debug, PartialEq)]
pub enum TlsServerIdentifier {
    /// .
    DnsName(&'static str),
    ///
    IpAddr(IpAddr),
}

impl TryFrom<TlsServerIdentifier> for RustlsServerName<'_> {
    type Error = TlsError;
    fn try_from(c: TlsServerIdentifier) -> Result<Self, Self::Error> {
        match c {
            TlsServerIdentifier::DnsName(s) => {
                Ok(Self::DnsName(s.try_into().map_err(TlsError::RustlsDns)?))
            }
            TlsServerIdentifier::IpAddr(i) => todo!(), //
        }
    }
}
