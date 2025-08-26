//! TLS Entities

mod tls_server_config;
pub use tls_server_config::TlsServerConfig;

use crate::Arc;
use crate::TlsError;
use crate::TlsPosition;

use rustls::server::ServerConfig as RustlsServerConfig;
use rustls::server::UnbufferedServerConnection as RustlsServerConnection;

use rustls::unbuffered::ConnectionState as RustlsConnectionState;
use rustls::unbuffered::UnbufferedStatus as RustlsUnbufferedStatus;

use blueprint::{Left, Right};

/// .
pub struct TlsServer {
    config: TlsServerConfig,
    rustls_config: RustlsServerConfig,
    rustls_server: RustlsServerConnection,
}

impl TlsServer {
    /// Construct new
    pub fn with_config(config: TlsServerConfig) -> Result<Self, TlsError> {
        let rustls_config: RustlsServerConfig = config.clone().try_into()?;

        let rustls_server = RustlsServerConnection::new(Arc::new(rustls_config.clone()))
            .map_err(TlsError::RustlsConfig)?;

        Ok(Self {
            config,
            rustls_config,
            rustls_server,
        })
    }
    /// Advance the state machine
    pub fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<TlsPosition, TlsError> {
        let (in_b, out_b) = l.bufs();
        println!(
            "Server state wants_write<{:?}> is_handshaking<{:?}> wants_read<{:?}>",
            self.rustls_server.wants_write(),
            self.rustls_server.is_handshaking(),
            self.rustls_server.wants_read()
        );

        let status = self.rustls_server.process_tls_records(in_b);
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
                    Some(w) => println!("Client can encrypt."),
                    None => println!("Client cannot encrypt yet."),
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
