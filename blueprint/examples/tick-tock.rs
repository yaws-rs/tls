//! Tick Tock simulator

use blueprint::{BluePrint, Orbit};
use blueprint::{Left, Right};
use blueprint_tls::TlsPosition;
use blueprint_tls::{TlsClient, TlsServer};
use blueprint_tls::{TlsClientConfig, TlsServerConfig};

const CA: &'static str = "certs/ca.rsa4096.crt";
const CERT: &'static str = "certs/rustcryp.to.rsa4096.ca_signed.crt";
const KEY: &'static str = "certs/rustcryp.to.rsa4096.key";

use std::path::Path;

#[derive(Default)]
struct PreTls {
    buf_in: Vec<u8>,
    buf_in_len: usize,
    buf_out: Vec<u8>,
    buf_out_len: usize,
    is_ready: bool,
}

impl Left for PreTls {
    fn left_lens(&self) -> (usize, usize) {
        (self.buf_in_len, self.buf_out_len)
    }
    fn left_set_lens(&mut self, len_in: usize, len_out: usize) -> () {
        self.buf_in_len = len_in;
        self.buf_out_len = len_out;
    }
    fn left_bufs_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        (&mut self.buf_in[0..self.buf_in_len], &mut self.buf_out)
    }
    fn is_ready(&self) -> bool {
        self.is_ready
    }
    fn set_ready(&mut self, s: bool) -> bool {
        self.is_ready = s;
        s
    }
}

#[derive(Default)]
struct PostTls {
    buf_in: Vec<u8>,
    buf_in_len: usize,
    buf_out: Vec<u8>,
    buf_out_len: usize,
    wants_right_next_in: bool,
}

impl Right for PostTls {
    fn out_len(&self) -> usize {
        self.buf_out_len
    }
    fn buf_right_out(&self) -> &[u8] {
        &self.buf_out[0..self.buf_out_len]
    }
    fn wants_right_next_in(&self) -> bool {
        self.wants_right_next_in
    }
    fn set_wants_right_next_in(&mut self, s: bool) -> () {
        self.wants_right_next_in = s
    }
    fn all_sent_right_out(&mut self) -> () {
        self.buf_out_len = 0;
        self.buf_out = vec![0u8; 8192];
    }
    // TODO: what about fragmentation?
    fn add_right_out(&mut self, bs: &[u8]) -> () {
        let start_pos = self.buf_out_len;
        let end_pos = self.buf_out_len + bs.len();
        self.buf_out[start_pos..end_pos].copy_from_slice(bs);
        self.buf_out_len += bs.len();
    }
    // TODO: what about fragmentation?
    fn add_right_in(&mut self, bs: &[u8]) -> () {
        let start_pos = self.buf_in_len;
        let end_pos = self.buf_in_len + bs.len();
        self.buf_in[start_pos..end_pos].copy_from_slice(bs);
        self.buf_in_len += bs.len();
    }
}

const ZERO_BUF: [u8; 8192] = [0u8; 8192];

// Pretend the bytes travel over the network
fn aft_emulate_network<'a>(tls_pos: &TlsPosition, tx: &mut PreTls, rx: &mut PreTls) {
    if tls_pos.out_send > 0 {
        let out_bytes = tls_pos.out_send;
        let out_end = rx.buf_in_len + out_bytes;
        rx.buf_in[rx.buf_in_len..out_end].copy_from_slice(&tx.buf_out[0..out_bytes]);
        rx.buf_in_len += out_bytes;
        tx.buf_out = vec![0u8; 8192];
    }

    if tls_pos.in_discard > 0 {
        let discard = tls_pos.in_discard;
        let copy_count = 8192 - discard;
        tx.buf_in.rotate_left(discard);
        tx.buf_in[discard..8192].copy_from_slice(&ZERO_BUF[discard..8192]);
        tx.buf_in_len -= discard;
    }
}

fn main() {
    let config_server =
        TlsServerConfig::with_certs_and_key_file(Path::new(CA), Path::new(CERT), Path::new(KEY))
            .unwrap();
    let config_client = TlsClientConfig::with_hostname("localhost").unwrap();

    let mut server = TlsServer::with_config(config_server).unwrap();
    let mut client = TlsClient::with_config(config_client).unwrap();

    // Client Ciphertext (Left) side
    let mut client_left = PreTls {
        buf_in: vec![0u8; 8192],
        buf_out: vec![0u8; 8192],
        ..Default::default()
    };
    // Client Cleartext (Right) side
    let mut client_right = PostTls {
        buf_in: vec![0u8; 8192],
        buf_out: vec![0u8; 8192],
        ..Default::default()
    };
    // Serer Ciphertext (Left) side
    let mut server_left = PreTls {
        buf_in: vec![0u8; 8192],
        buf_out: vec![0u8; 8192],
        ..Default::default()
    };
    // Server Cleartext (Right) side
    let mut server_right = PostTls {
        buf_in: vec![0u8; 8192],
        buf_out: vec![0u8; 8192],
        ..Default::default()
    };

    struct Empty;
    let mut u = Empty;

    client_right.buf_out[0..4].copy_from_slice("TICK".as_bytes());
    client_right.buf_out_len = 4;

    loop {
        //----- Client side --------
        let tls_client_pos = client
            .advance_with(&mut u, &mut client_left, &mut client_right)
            .unwrap();
        // Assume this was sent over network Client -> Server
        aft_emulate_network(&tls_client_pos, &mut client_left, &mut server_left);

        //----- Server side --------
        let tls_server_pos = server
            .advance_with(&mut u, &mut server_left, &mut server_right)
            .unwrap();
        // Assume this was sent over network Server -> Client
        aft_emulate_network(&tls_server_pos, &mut server_left, &mut client_left);
    }
}
