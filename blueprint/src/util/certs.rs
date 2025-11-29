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

    let mut pkcs10_file = File::open(ca_path).unwrap();
    let mut pkcs10_data: Vec<u8> = vec![];
    pkcs10_file.read_to_end(&mut pkcs10_data).unwrap();
    
    Ok(CertificateDer::from_pem_slice(&pkcs10_data).unwrap())
    
}

/*
/// Read rustls compatible CertificateDer from bytes
pub fn load_cert_der_bytes(ca_pkcs10_data: &[u8]) -> Result<CertificateDer<'static>, TlsError> {
    let (ca_type_label, ca_data) = pem_rfc7468::decode_vec(&ca_pkcs10_data).unwrap();
    assert_eq!(ca_type_label, "CERTIFICATE");
    Ok(ca_data.into())
} */

use rustls_pki_types::pem::PemObject;

/// Read rustls compatible PrivatekeyDer from a file
#[cfg(feature = "std")]
pub fn load_private_key_der_file(key_path: &Path) -> Result<PrivateKeyDer<'static>, TlsError> {
    /*
    let mut key_file = File::open(key_path).unwrap();
    let mut key_data: Vec<u8> = vec![];
    key_file.read_to_end(&mut key_data).unwrap();
    load_private_key_der_bytes(key_data)*/
    //let pk = PrivatePkcs8KeyDer::from_pem_file(key_path).unwrap();
    //Ok(PrivateKeyDer::Pkcs8(pk))
    let pk = PrivateKeyDer::from_pem_file(key_path).unwrap();
    Ok(pk)
}

/*
/// Read rustls compatible PrivatekeyDer from bytes
pub fn load_private_key_der_bytes(key_data_in: Vec<u8>) -> Result<PrivateKeyDer<'static>, TlsError> {
    //let (key_label, key_data) = pem_rfc7468::decode_vec(&key_data_in).unwrap();
    //assert_eq!(key_label, "EC PRIVATE KEY");
    //let pk_ref = pkcs8::PrivateKeyInfo::try_from(key_data_in.as_slice()).unwrap();
    //let key_data = pk_ref.private_key.as_bytes();
    //PrivatePkcs8KeyDer::from_pem_file("tests/data/nistp256key.pkcs8.pem").unwrap();
    //let rustls_pkcs8der: PrivatePkcs8KeyDer = key_data_in.into();
    Ok(rustls_pkcs8der.into())
} */

/// provide rustls roots with pinned CA cert
pub fn roots(ca_pinned: CertificateDer) -> RustlsRootCertStore {
    let mut roots = RustlsRootCertStore::empty();
    roots.add(ca_pinned).unwrap();
    roots
}
