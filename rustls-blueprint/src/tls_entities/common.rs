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

impl TlsServerIdentifier {
    /// Build with hostname
    pub fn with_hostname(host: &'static str) -> Result<Self, TlsError> {
        Ok(Self::DnsName(host.into()))
    }
    /// Build with [`IpAddr`]
    pub fn with_ipaddr(addr: IpAddr) -> Result<Self, TlsError> {
        Ok(Self::IpAddr(addr))
    }
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
