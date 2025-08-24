//! Certs utilities

use crate::TlsError;
use rustls::RootCertStore as RustlsRootCertStore;
use rustls_pki_types::PrivatePkcs8KeyDer;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};

#[cfg(feature = "std")]
use std::{fs::File, io::Read, path::Path};

/// Read rustls compatible CertificateDer from a file
#[cfg(feature = "std")]
pub fn load_cert_der_file(ca_path: &Path) -> Result<CertificateDer<'static>, TlsError> {
    let mut ca_pkcs10_file = File::open(ca_path).unwrap();
    let mut ca_pkcs10_data: Vec<u8> = vec![];
    ca_pkcs10_file.read_to_end(&mut ca_pkcs10_data).unwrap();
    load_cert_der_bytes(&ca_pkcs10_data)
}

/// Read rustls compatible CertificateDer from bytes
pub fn load_cert_der_bytes(ca_pkcs10_data: &[u8]) -> Result<CertificateDer<'static>, TlsError> {
    let (ca_type_label, ca_data) = pem_rfc7468::decode_vec(&ca_pkcs10_data).unwrap();
    assert_eq!(ca_type_label, "CERTIFICATE");
    Ok(ca_data.into())
}

/// Read rustls compatible PrivatekeyDer from a file
#[cfg(feature = "std")]
pub fn load_private_key_der_file(key_path: &Path) -> Result<PrivateKeyDer<'static>, TlsError> {
    let mut key_file = File::open(key_path).unwrap();
    let mut key_data: Vec<u8> = vec![];
    key_file.read_to_end(&mut key_data).unwrap();
    load_private_key_der_bytes(&key_data)
}

/// Read rustls compatible PrivatekeyDer from bytes
pub fn load_private_key_der_bytes(key_data_in: &[u8]) -> Result<PrivateKeyDer<'static>, TlsError> {
    let (key_label, key_data) = pem_rfc7468::decode_vec(&key_data_in).unwrap();
    assert_eq!(key_label, "PRIVATE KEY");
    let rustls_pkcs8der: PrivatePkcs8KeyDer = key_data.into();
    Ok(rustls_pkcs8der.into())
}

/// provide rustls roots with pinned CA cert
pub fn roots(ca_pinned: CertificateDer) -> RustlsRootCertStore {
    let mut roots = RustlsRootCertStore::empty();
    roots.add(ca_pinned).unwrap();
    roots
}
