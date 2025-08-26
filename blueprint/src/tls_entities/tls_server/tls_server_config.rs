//! Tls Server Config

use crate::Arc;
use crate::TlsError;
use crate::TlsServerIdentifier;
use crate::Vec;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};

#[cfg(feature = "std")]
use rustls_pki_types::pem::PemObject;

use crate::tls_entities::tls_server::RustlsServerConfig;

use crate::FakeTime;

#[cfg(feature = "std")]
use std::path::Path;

///
#[derive(Debug)]
pub struct TlsServerConfig {
    rustls_cert_chain: Vec<CertificateDer<'static>>,
    // TODO: don't stick to this being here. just testing for now.
    rustls_key_der: PrivateKeyDer<'static>,
}

impl TlsServerConfig {
    /// Construct new with cert chain and key
    // TODO: generalise cert & key instead of exposing rustls types here
    pub fn with_certs_and_key(
        chain: Vec<CertificateDer<'static>>,
        rustls_key_der: PrivateKeyDer<'static>,
    ) -> Result<Self, TlsError> {
        Ok(Self {
            rustls_cert_chain: chain,
            rustls_key_der,
        })
    }
    /// Construct new from PEM files of ca, signed cert and key
    #[cfg(all(feature = "std", feature = "util"))]
    pub fn with_certs_and_key_file(
        ca_file: &Path,
        cert_file: &Path,
        key_file: &Path,
    ) -> Result<Self, TlsError> {
        let ca_der = crate::util::load_cert_der_file(ca_file)?;
        let cert_der = crate::util::load_cert_der_file(cert_file)?;
        let private_key_der = crate::util::load_private_key_der_file(key_file)?;

        Ok(Self {
            rustls_cert_chain: vec![ca_der, cert_der],
            rustls_key_der: private_key_der,
        })
    }
}

impl Clone for TlsServerConfig {
    fn clone(&self) -> Self {
        Self {
            rustls_cert_chain: self.rustls_cert_chain.clone(),
            rustls_key_der: self.rustls_key_der.clone_key(),
        }
    }
}

impl TlsServerConfig {}

impl TryFrom<TlsServerConfig> for RustlsServerConfig {
    type Error = TlsError;

    fn try_from(c: TlsServerConfig) -> Result<Self, Self::Error> {
        // TODO: time in no_std
        let fake_time = FakeTime {};

        #[cfg(feature = "std")]
        let rustls_config =
            RustlsServerConfig::builder_with_provider(Arc::new(rustls_rustcrypto::provider()));

        #[cfg(not(feature = "std"))]
        let rustls_config = RustlsServerConfig::builder_with_details(
            Arc::new(rustls_rustcrypto::provider()),
            Arc::new(fake_time),
        );

        let rustls_config = rustls_config
            .with_safe_default_protocol_versions()
            .map_err(TlsError::RustlsConfig)?;

        let rustls_config = rustls_config.with_no_client_auth();

        /*        let certs_res: Vec<_> = CertificateDer::pem_file_iter("certs/cert-chain.pem")
            .unwrap()
            .collect();
        let certs: Vec<_> = certs_res.into_iter().map(|res| res.unwrap()).collect();
        let pkcs8 = PrivateKeyDer::from_pem_file("certs/rustcryp.to.rsa4096.key").unwrap();
        let rustls_config = rustls_config
            .with_single_cert(certs, pkcs8.into())
            .map_err(TlsError::RustlsConfig)?; */

        let rustls_config = rustls_config
            .with_single_cert(c.rustls_cert_chain.clone(), c.rustls_key_der.clone_key())
            .map_err(TlsError::RustlsConfig)?;

        Ok(rustls_config)
    }
}
