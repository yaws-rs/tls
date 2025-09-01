//! Tick Tock simulator

use blueprint::{BluePrint, Orbit};
use blueprint::{Left, Right};
use blueprint_tls::{TlsClient, TlsServer};
use blueprint_tls::{TlsClientConfig, TlsServerConfig};
use blueprint_tls::TlsPosition;

const CA: &'static str = "certs/ca.rsa4096.crt";
const CERT: &'static str = "certs/rustcryp.to.rsa4096.ca_signed.crt";
const KEY: &'static str = "certs/rustcryp.to.rsa4096.key";

use std::path::Path;

struct PreTls {
    buf_in: Vec<u8>,
    buf_in_pos: usize,
    buf_out: Vec<u8>,
}

impl Left for PreTls {
    fn bufs(&mut self) -> (&mut [u8], &mut [u8]) {
        (&mut self.buf_in[0..self.buf_in_pos], &mut self.buf_out)
    }
}

struct PostTls {
    buf_in: Vec<u8>,
    buf_in_pos: usize,
    buf_out: Vec<u8>,
}

impl Right for PostTls {}

fn aft_emulate_network<'a>(tls_pos: &TlsPosition, tx: &mut PreTls, rx: &mut PreTls) {
    let zero_buf = vec![0u8; 8192];    

    if tls_pos.out_send > 0 {
        let out_bytes = tls_pos.out_send;
        println!("Sending = {}", out_bytes);
        let out_end = rx.buf_in_pos + out_bytes;
        rx.buf_in[rx.buf_in_pos..out_end]
            .copy_from_slice(&tx.buf_out[0..out_bytes]);
        rx.buf_in_pos += out_bytes;
        tx.buf_out = vec![0u8; 8192];
    }

    if tls_pos.in_discard > 0 {
        let discard = tls_pos.in_discard;
        let copy_count = 8192 - discard;
                tx.buf_in.rotate_left(discard);
        tx.buf_in[discard..8192].copy_from_slice(&zero_buf[discard..8192]);
        tx.buf_in_pos -= discard;
        println!(
            "Discarded {} pos after = {}",
            discard, tx.buf_in_pos
        );
    }            
}

fn main() {
    let config_server =
        TlsServerConfig::with_certs_and_key_file(Path::new(CA), Path::new(CERT), Path::new(KEY))
            .unwrap();
    let config_client = TlsClientConfig::with_hostname("localhost").unwrap();

    let mut server = TlsServer::with_config(config_server).unwrap();
    let mut client = TlsClient::with_config(config_client).unwrap();

    let mut client_left = PreTls {
        buf_in: vec![0u8; 8192],
        buf_in_pos: 0,
        buf_out: vec![0u8; 8192],
    };
    let mut client_right = PostTls {
        buf_in: vec![0u8; 8192],
        buf_in_pos: 0,
        buf_out: vec![0u8; 8192],
    };
    let mut server_left = PreTls {
        buf_in: vec![0u8; 8192],
        buf_in_pos: 0,
        buf_out: vec![0u8; 8192],
    };
    let mut server_right = PostTls {
        buf_in: vec![0u8; 8192],
        buf_in_pos: 0,
        buf_out: vec![0u8; 8192],
    };

    struct Empty;
    let mut u = Empty;

    let zero_buf = vec![0u8; 8192];

    loop {
        let tls_client_pos = client
            .advance_with(&mut u, &mut client_left, &mut client_right)
            .unwrap();
        println!("TLS Client Pos = {:?}", tls_client_pos);
        // Assume this was sent over network Client -> Server        
        aft_emulate_network(&tls_client_pos, &mut client_left, &mut server_left);
        
        println!(
            "-- Server Pos client_in = {}, server_in = {}",
            client_left.buf_in_pos, server_left.buf_in_pos
        );

        if server_left.buf_in_pos > 0 {
            println!("Server receive buffer len = {}", server_left.buf_in_pos);
            let tls_server_pos = server
                .advance_with(&mut u, &mut server_left, &mut server_right)
                .unwrap();
            println!("TLS Server Pos = {:?}", tls_server_pos);
            // Assume this was sent over network Server -> Client
            aft_emulate_network(&tls_server_pos, &mut server_left, &mut client_left);
        }
    }
}
