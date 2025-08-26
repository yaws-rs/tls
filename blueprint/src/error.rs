//! Tls Errors

use rustls::unbuffered::EncodeError as RustlsEncodeError;
use rustls::Error as RustlsError;
use rustls_pki_types::InvalidDnsNameError as RustlsInvalidDnsNameError;

/// .
#[derive(Debug)]
pub enum TlsError {
    /// Rustls backend reports invalid configuration
    RustlsConfig(RustlsError),
    /// Rustls baclend rejects DNS name
    RustlsDns(RustlsInvalidDnsNameError),
    /// Rustls Encode Error
    RustlsEncode(RustlsEncodeError),
    /// Rustls Handle Records error
    RustlsHandleRecords(RustlsError),
}
