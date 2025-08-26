//! TLS Entities

use crate::Arc;

mod tls_client_config;
pub use tls_client_config::TlsClientConfig;

use crate::TlsError;
use crate::TlsPosition;

use rustls::client::ClientConfig as RustlsClientConfig;
use rustls::client::UnbufferedClientConnection as RustlsClientConnection;

use rustls::unbuffered::ConnectionState as RustlsConnectionState;
use rustls::unbuffered::UnbufferedStatus as RustlsUnbufferedStatus;

use crate::tls_entities::{FakeServerCertVerifier, FakeTime};

use crate::tls_entities::TlsServerIdentifier;
use rustls_pki_types::DnsName as RustlsDnsName;

use blueprint::{Left, Right};

/// .
pub struct TlsClient {
    /// .
    config: TlsClientConfig,
    /// .
    rustls_config: RustlsClientConfig,
    /// .
    rustls_client: RustlsClientConnection,
}

use rustls_pki_types::ServerName as RustlsServerName;

impl TlsClient {
    /// Construct new
    pub fn with_config(config: TlsClientConfig) -> Result<Self, TlsError> {
        let rustls_config: RustlsClientConfig = config.clone().try_into()?;

        let rustls_client = RustlsClientConnection::new(
            Arc::new(rustls_config.clone()),
            config.server_identifier.clone().try_into()?,
        )
        .map_err(TlsError::RustlsConfig)?;

        Ok(Self {
            config,
            rustls_config,
            rustls_client,
        })
    }
    /// Advance the state machine
    pub fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<TlsPosition, TlsError> {
        println!(
            "Client state wants_write<{:?}> is_handshaking<{:?}> wants_read<{:?}>",
            self.rustls_client.wants_write(),
            self.rustls_client.is_handshaking(),
            self.rustls_client.wants_read()
        );

        let (in_b, out_b) = l.bufs();

        let status = self.rustls_client.process_tls_records(in_b);
        let in_discard = status.discard;

        let rustls_state = match status.state {
            Err(e) => return Err(TlsError::RustlsHandleRecords(e)),
            Ok(s) => s,
        };

        match rustls_state {
            RustlsConnectionState::EncodeTlsData(mut e) => {
                let encoded_size = e.encode(out_b).map_err(TlsError::RustlsEncode)?;

                Ok(TlsPosition {
                    in_discard,
                    out_encoded: 0,
                    out_send: encoded_size,
                })
            }
            RustlsConnectionState::TransmitTlsData(mut t) => {
                match t.may_encrypt_app_data() {
                    Some(w) => println!("Server can encrypt."),
                    None => println!("Server cannot encrypt yet."),
                }
                t.done();
                Ok(TlsPosition {
                    in_discard,
                    out_send: 0,
                    out_encoded: 0,
                })
            }
            _ => {
                dbg!(rustls_state);
                todo!()
            }
        }
    }
}
