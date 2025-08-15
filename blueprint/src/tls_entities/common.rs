//! Common TLS Entities

use crate::TlsError;

use core::net::IpAddr;

use rustls_pki_types::ServerName as RustlsServerName;

#[derive(Clone, Debug, PartialEq)]
pub enum DnsName<'c> {
    Valid(&'c str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TlsServerIdentifier<'c> {
    DnsName(DnsName<'c>),
    IpAddress(IpAddr),    
}


impl TryFrom<TlsServerIdentifier<'_>> for RustlsServerName<'_> {
    type Error = TlsError;
    fn try_from(_: TlsServerIdentifier<'_>) -> RustlsServerName<'_> {
        todo!()
    }
}
