//! Tls Errors

use rustls::Error as RustlsError;

/// .
pub enum TlsError {
    /// Rustls backend reports invalid configuration
    RustlsConfig(RustlsError),
}
