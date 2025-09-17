//! Rusls Unbuffered Context

use rustls::unbuffered::ConnectionState as RustlsConnectionState;

use rustls::client::ClientConnectionData as RustlsClientConnectionData;
use rustls::server::ServerConnectionData as RustlsServerConnectionData;

use crate::TlsError;
use crate::TlsPosition;

use crate::tls_entities::{TlsClient, TlsServer};

use blueprint::{Left, Right};

pub(crate) enum CtxRustls<'ctx> {
    Client(&'ctx mut TlsClient),
    Server(&'ctx mut TlsServer),
}

pub(crate) enum TlsConnectionState<'a, 'b> {
    Server(RustlsConnectionState<'a, 'b, RustlsServerConnectionData>),
    Client(RustlsConnectionState<'a, 'b, RustlsClientConnectionData>),
}

impl<'ctx> CtxRustls<'ctx> {
    #[inline]
    pub(crate) fn with_server(s: &'ctx mut TlsServer) -> Self {
        Self::Server(s)
    }
    #[inline]
    pub(crate) fn with_client(c: &'ctx mut TlsClient) -> Self {
        Self::Client(c)
    }
    #[inline]
    fn wants_write(&self) -> bool {
        match self {
            Self::Server(s) => s.rustls_server.wants_write(),
            Self::Client(c) => c.rustls_client.wants_write(),
        }
    }
    #[inline]
    fn wants_read(&self) -> bool {
        match self {
            Self::Server(s) => s.rustls_server.wants_read(),
            Self::Client(c) => c.rustls_client.wants_read(),
        }
    }
    #[inline]
    fn is_handshaking(&self) -> bool {
        match self {
            Self::Server(s) => s.rustls_server.is_handshaking(),
            Self::Client(c) => c.rustls_client.is_handshaking(),
        }
    }
}

impl<'ctx> CtxRustls<'ctx> {
    /// Advance the state machine
    #[inline]
    pub(crate) fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<TlsPosition, TlsError> {
        println!(
            "CtxRustls Wants_write<{:?}> is_handshaking<{:?}> wants_read<{:?}>",
            self.wants_write(),
            self.is_handshaking(),
            self.wants_read()
        );

        let (left_in_b, left_out_b) = l.left_bufs_mut();

        let (mut in_discard, rustls_state) = match self {
            Self::Server(ref mut s) => {
                let s_status = s.rustls_server.process_tls_records(left_in_b);
                (
                    s_status.discard,
                    TlsConnectionState::Server(
                        s_status.state.map_err(TlsError::RustlsHandleRecords)?,
                    ),
                )
            }
            Self::Client(ref mut c) => {
                let c_status = c.rustls_client.process_tls_records(left_in_b);
                (
                    c_status.discard,
                    TlsConnectionState::Client(
                        c_status.state.map_err(TlsError::RustlsHandleRecords)?,
                    ),
                )
            }
        };

        let pos = match rustls_state {
            TlsConnectionState::Server(RustlsConnectionState::EncodeTlsData(mut e)) => {
                let encoded_size = e.encode(left_out_b).map_err(TlsError::RustlsEncode)?;
                TlsPosition::with_send(in_discard, encoded_size)
            }
            TlsConnectionState::Client(RustlsConnectionState::EncodeTlsData(mut e)) => {
                let encoded_size = e.encode(left_out_b).map_err(TlsError::RustlsEncode)?;
                TlsPosition::with_send(in_discard, encoded_size)
            }
            TlsConnectionState::Server(RustlsConnectionState::TransmitTlsData(mut t)) => {
                /* match t.may_encrypt_app_data() {
                    Some(w) => // send data
                } */
                t.done();
                TlsPosition::with_discard_only(in_discard)
            }
            TlsConnectionState::Client(RustlsConnectionState::TransmitTlsData(mut t)) => {
                /* match t.may_encrypt_app_data() {
                    Some(w) => // send data
                } */
                t.done();
                TlsPosition::with_discard_only(in_discard)
            }
            TlsConnectionState::Client(RustlsConnectionState::WriteTraffic(mut wt)) => {
                if r.out_len() > 0 {
                    let right_out_b = r.buf_right_out();
                    let sent_b = wt
                        .encrypt(right_out_b, left_out_b)
                        .map_err(|e| TlsError::RustlsEncrypt(in_discard, e))?;
                    r.all_sent_right_out();
                    TlsPosition::with_send(in_discard, sent_b)
                } else {
                    r.set_wants_right_next_in(true);
                    TlsPosition::want_write_right(in_discard)
                }
            }
            TlsConnectionState::Server(RustlsConnectionState::WriteTraffic(mut wt)) => {
                if r.out_len() > 0 {
                    let right_out_b = r.buf_right_out();
                    let sent_b = wt
                        .encrypt(right_out_b, left_out_b)
                        .map_err(|e| TlsError::RustlsEncrypt(in_discard, e))?;
                    r.all_sent_right_out();
                    TlsPosition::with_send(in_discard, sent_b)
                } else {
                    r.set_wants_right_next_in(true);
                    TlsPosition::want_write_right(in_discard)
                }
            }
            TlsConnectionState::Server(RustlsConnectionState::ReadTraffic(mut rt)) => {
                let mut recs_decrypted = 0;
                while let Some(rec_res) = rt.next_record() {
                    match rec_res {
                        Ok(decrypted_rec) => {
                            in_discard += decrypted_rec.discard;
                            r.add_right_in(decrypted_rec.payload);
                        }
                        Err(e) => {
                            return Err(TlsError::RustlsDecrypt(in_discard, e));
                        }
                    }
                }
                TlsPosition::with_discard_only(in_discard)
            }
            TlsConnectionState::Client(RustlsConnectionState::ReadTraffic(mut rt)) => {
                let mut recs_decrypted = 0;
                while let Some(rec_res) = rt.next_record() {
                    match rec_res {
                        Ok(decrypted_rec) => {
                            in_discard += decrypted_rec.discard;
                            r.add_right_in(decrypted_rec.payload);
                        }
                        Err(e) => {
                            return Err(TlsError::RustlsDecrypt(in_discard, e));
                        }
                    }
                }
                TlsPosition::with_discard_only(in_discard)
            }
            TlsConnectionState::Client(mut c) => {
                todo!("Client uninmplemented: {:?}", c)
            }
            TlsConnectionState::Server(mut s) => {
                todo!("Server unimplmented: {:?}", s)
            }
        };
        Ok(pos)
    }
}
