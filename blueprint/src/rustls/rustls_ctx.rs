//! Rusls Unbuffered Context

use rustls::unbuffered::ConnectionState as RustlsConnectionState;

use rustls::client::ClientConnectionData as RustlsClientConnectionData;
use rustls::server::ServerConnectionData as RustlsServerConnectionData;

use crate::TlsError;
use crate::TlsPosition;

use crate::tls_entities::{TlsClient, TlsServer};

use blueprint::{Left, Right, InBuffer};

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


        let mut total_in_discard = 0;
        
        loop {

        let mut go_again = false;
            
        l.set_left_in_blocked(false);
        let (left_in_len, left_out_len) = l.left_lens();
        let (left_inputs, mut left_out_b) = l.left_bufs_mut();

            // rustls state machine wrapers overwrite so ensure it only sees
            // the remaining outbuf
            left_out_b.split_off_mut(..left_out_len);
            
        let mut buf_tmp: Vec<u8> = vec![];
        
        let left_in_b = match left_inputs {
            InBuffer::Single(buf) => buf,
            InBuffer::Double(buf1, buf2) => {
                let total_len = buf1.len() + buf2.len();
                buf_tmp = Vec::with_capacity(total_len);
                buf_tmp.extend_from_slice(buf1);
                buf_tmp.extend_from_slice(buf2);
                buf_tmp.as_mut_slice()
            },
        };

            
            let (mut in_discard, rustls_state) = match self {
                Self::Server(ref mut s) => {
                    let s_status = s.rustls_server.process_tls_records(left_in_b);
                    let ok_state = match s_status.state {
                        Ok(s) => s,
                        Err(e) => {
                            // KTODO: discard
                            return Ok(TlsPosition::with_discard_only(s_status.discard));
                        },
                    };
                    
                    (
                        s_status.discard,
                        TlsConnectionState::Server(ok_state),
                    )
                }
                Self::Client(ref mut c) => {
                    let c_status = c.rustls_client.process_tls_records(left_in_b);
                    //println!("c_status = {:?}", c_status);
                    
                    let ok_state = match c_status.state {
                        Ok(s) => s,
                        Err(e) => {
                            // KTODO: discard
                            return Ok(TlsPosition::with_discard_only(c_status.discard));
                        },
                    };
                    
                    (
                        c_status.discard,
                        TlsConnectionState::Client(ok_state),
                    )
                }
            };
            
            
            let pos = match rustls_state {
                TlsConnectionState::Server(RustlsConnectionState::EncodeTlsData(mut e)) => {
                    let encoded_size = e.encode(left_out_b).map_err(TlsError::RustlsEncode)?;
                    // KTODO: fix this. provide only limited outbuf so it doesn't overwrite ?
                    //go_again = true;
                    TlsPosition::with_send(in_discard, encoded_size)
                }
                TlsConnectionState::Client(RustlsConnectionState::EncodeTlsData(mut e)) => {
                    let encoded_size = e.encode(left_out_b).map_err(TlsError::RustlsEncode)?;
                    //go_again = true;
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
                    let (r_in_len, r_out_len) = r.right_lens();
                    if r_out_len > 0 {
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
                    let (r_in_len, r_out_len) = r.right_lens();
                    if r_out_len > 0 {
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
                TlsConnectionState::Server(RustlsConnectionState::BlockedHandshake) => {
                    l.set_left_want_read(true);
                    l.set_left_in_blocked(true);                
                    TlsPosition::with_blocked_handshake(in_discard)
                },
                TlsConnectionState::Client(RustlsConnectionState::BlockedHandshake)	=> {
                    l.set_left_want_read(true);
                    l.set_left_in_blocked(true);
		            TlsPosition::with_blocked_handshake(in_discard)
                },
                TlsConnectionState::Client(mut c) => {
                    todo!("Client uninmplemented: {:?}", c)
                }
                TlsConnectionState::Server(mut s) => {
                    todo!("Server unimplmented: {:?}", s)
                }
            };

            let new_in_len = left_in_len - pos.in_discard;
            let new_out_len = left_out_len + pos.out_send;
            
            let handshaking = self.is_handshaking();
            
            l.left_set_lens(new_in_len, new_out_len);
            
            l.set_ready(!self.is_handshaking());


            total_in_discard += in_discard;
            
            if !go_again {
                break;
            }
        }
            
        Ok(TlsPosition::with_discard_only(total_in_discard))
    }
}
