//! Tick Tock simulator

use blueprint::{BluePrint, Orbit};
use blueprint::{Left, Right};
use blueprint_tls::{TlsClient, TlsServer};
use blueprint_tls::{TlsClientConfig, TlsServerConfig};

const CA: &'static str = "certs/ca.rsa4096.crt";
const CERT: &'static str = "certs/rustcryp.to.rsa4096.ca_signed.crt";
const KEY: &'static str = "certs/rustcryp.to.rsa4096.key";

use std::path::Path;

struct ClientPreTls {
    buf_in: Vec<u8>,
    buf_out: Vec<u8>,
}

struct ServerPreTls {
    buf_in: Vec<u8>,
    buf_out: Vec<u8>,
}

impl Left for ClientPreTls {
    fn bufs(&mut self) -> (&mut [u8], &mut [u8]) {
        (&mut self.buf_in, &mut self.buf_out)
    }
}

impl Left for ServerPreTls {
    fn bufs(&mut self) -> (&mut [u8], &mut [u8]) {
        (&mut self.buf_in, &mut self.buf_out)
    }
}

struct ClientPostTls {
    buf_in: Vec<u8>,
    buf_out: Vec<u8>,
}

struct ServerPostTls {
    buf_in: Vec<u8>,
    buf_out: Vec<u8>,
}

impl Right for ClientPostTls {}

impl Right for ServerPostTls {}

fn main() {
    let config_server =
        TlsServerConfig::with_certs_and_key_file(Path::new(CA), Path::new(CERT), Path::new(KEY))
            .unwrap();
    let config_client = TlsClientConfig::with_hostname("localhost").unwrap();

    let mut server = TlsServer::with_config(config_server).unwrap();
    let mut client = TlsClient::with_config(config_client).unwrap();

    /*
    let mut buf_client_in = vec![0u8; 8192];
    let mut buf_client_out = vec![0u8; 8192];
    let mut buf_server_in = vec![0u8; 8192];
    let mut buf_server_out = vec![0u8; 8192];
    */

    struct Empty;
    let mut u = Empty;

    let mut client_in_pos = 0;
    let mut server_in_pos = 0;

    let zero_buf = vec![0u8; 8192];

    loop {
        let tls_client_pos = client
            .advance_with(
                &mut u,
                &mut buf_client_in[0..client_in_pos],
                &mut buf_client_out,
            )
            .unwrap();
        println!("TLS Client Pos = {:?}", tls_client_pos);
        // Assume this was sent over network Client -> Server
        if tls_client_pos.out_send > 0 {
            let out_bytes = tls_client_pos.out_send;
            println!("Client sending = {}", out_bytes);
            let out_end = server_in_pos + out_bytes;
            buf_server_in[server_in_pos..out_end].copy_from_slice(&buf_client_out[0..out_bytes]);
            buf_client_out = vec![0u8; 8192];
            server_in_pos += out_bytes;
        }

        if tls_client_pos.in_discard > 0 {
            let discard = tls_client_pos.in_discard;
            let copy_count = 8192 - discard;
            buf_client_in.rotate_left(discard);
            buf_client_in[discard..8192].copy_from_slice(&zero_buf[discard..8192]);
            client_in_pos -= discard;
            println!("Client discarded {} pos after = {}", discard, client_in_pos);
        }

        println!(
            "-- Server Pos client_in = {}, server_in = {}",
            client_in_pos, server_in_pos
        );

        if server_in_pos > 0 {
            println!("Server receive buffer len = {}", server_in_pos);
            let tls_server_pos = server
                .advance_with(
                    &mut u,
                    &mut buf_server_in[0..server_in_pos],
                    &mut buf_server_out,
                )
                .unwrap();
            println!("TLS Server Pos = {:?}", tls_server_pos);
            // Assume this was sent over network Server -> Client
            if tls_server_pos.out_send > 0 {
                let out_bytes = tls_server_pos.out_send;
                println!("Server sending = {}", out_bytes);
                let out_end = client_in_pos + out_bytes;
                buf_client_in[client_in_pos..out_bytes]
                    .copy_from_slice(&buf_server_out[0..out_bytes]);
                buf_server_out = vec![0u8; 8192];
                client_in_pos += out_bytes;
            }

            if tls_server_pos.in_discard > 0 {
                let discard = tls_server_pos.in_discard;
                let copy_count = 8192 - discard;
                buf_server_in.rotate_left(discard);
                buf_server_in[discard..8192].copy_from_slice(&zero_buf[discard..8192]);
                server_in_pos -= discard;
                println!("Server discarded {} pos after = {}", discard, server_in_pos);
            }
        }
    }
}
