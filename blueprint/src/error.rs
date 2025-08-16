//! Tls Errors

use rustls::Error as RustlsError;
use rustls_pki_types::InvalidDnsNameError as RustlsInvalidDnsNameError;

/// .
pub enum TlsError {
    /// Rustls backend reports invalid configuration
    RustlsConfig(RustlsError),
    /// Rustls baclend rejects DNS name
    RustlsDns(RustlsInvalidDnsNameError),
}
