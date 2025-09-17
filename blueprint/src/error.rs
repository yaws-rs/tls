//! Tls Errors
// TODO: re-oragnize Rustls errors to separate group or remove pub API exposure here

use rustls::unbuffered::EncodeError as RustlsEncodeError;
use rustls::unbuffered::EncryptError as RustlsEncryptError;
use rustls::Error as RustlsError;
use rustls_pki_types::InvalidDnsNameError as RustlsInvalidDnsNameError;

/// .
#[derive(Debug)]
pub enum TlsError {
    /// Rustls backend reports invalid configuration
    RustlsConfig(RustlsError),
    /// Rustls baclend rejects DNS name
    RustlsDns(RustlsInvalidDnsNameError),
    /// Rustls Encode
    RustlsEncode(RustlsEncodeError),
    /// Rustls Handle Records
    RustlsHandleRecords(RustlsError),
    /// Rustls Encrypt discard usize with EncryptError
    RustlsEncrypt(usize, RustlsEncryptError),
    /// Rustls Decrypt discard usize with RustlsError
    RustlsDecrypt(usize, RustlsError),
}
