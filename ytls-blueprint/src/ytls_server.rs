//! yTLS Server Blueprint

use crate::TlsError;
use crate::TlsPosition;

#[doc(inline)]
pub use ytls_server::TlsServerCtxConfig;
#[doc(inline)]
pub use ytls_typed::Alpn;

use ::ytls_server::TlsServerCtx;
use ::ytls_traits::{CryptoRng, CryptoConfig};
use ::ytls_traits::{TlsLeftIn, TlsLeftOut, TlsRight};

use blueprint::{Left, Right, InBuffer};

use heapless::Vec;

use rand::rngs::ThreadRng;

// Proxy configuration to yTLS Server Context
pub struct TlsServerConfig {
    ca_cert: Vec<u8, 1024>,
    server_cert: Vec<u8, 1024>,
    server_private_key: Vec<u8, 512>,
}

#[derive(Debug)]
pub enum ConfigError {
    OversizedCa,
    OversizedCert,
    OversizedPrivateKey,
}

impl TlsServerConfig {
    pub fn with_ca_cert_key(s_ca: &[u8], s_cert: &[u8], s_key: &[u8]) -> Result<Self, ConfigError> {

        let mut ca_cert = Vec::<u8, 1024>::new();
        let mut server_cert = Vec::<u8, 1024>::new();
        let mut server_private_key = Vec::<u8, 512>::new();

        ca_cert.extend_from_slice(s_ca)
            .map_err(|_| ConfigError::OversizedCa)?;
        
        server_cert.extend_from_slice(s_cert)
            .map_err(|_| ConfigError::OversizedCert)?;

        server_private_key.extend_from_slice(s_key)
            .map_err(|_| ConfigError::OversizedPrivateKey)?;
        
        Ok(Self { ca_cert, server_cert, server_private_key })
    }
}

impl TlsServerCtxConfig for TlsServerConfig {
    #[inline]
    fn dns_host_name(&self, host: &str) -> bool {
        host == "test.rustcryp.to"
    }
    #[inline]
    fn alpn<'r>(&self, alpn: Alpn<'r>) -> bool {
        if alpn == Alpn::Http11 {
            return true;
        }
        false
    }
    #[inline]
    fn server_private_key(&self) -> &[u8] {
        &self.server_private_key
    }
    #[inline]
    fn server_cert_chain(&self) -> &[u8] {
        &[0, 1]
    }
    #[inline]
    fn server_cert(&self, id: u8) -> &[u8] {
        match id {
            0 => &self.server_cert,
            1 => &self.ca_cert,
            _ => unreachable!(),
        }
    }    
}

/// yTLS Server Blueprint
pub struct TlsServer {
    ctx: TlsServerCtx<TlsServerConfig, ytls_rustcrypto::RustCrypto, ThreadRng>
}

impl core::fmt::Debug for TlsServer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "TlsServer")
    }
}

struct ProxyLeftIn<'r> {
    left_in: InBuffer<'r>,
    discard_in: usize,
}

impl<'r> TlsLeftIn for ProxyLeftIn<'r> {
    fn left_buf_in(&self) -> &[u8] {
        match &self.left_in {
            InBuffer::Single(b) => &b[self.discard_in..],
            InBuffer::Double(_a, _b) => todo!(),
        }
    }
    fn left_buf_mark_discard_in(&mut self, len: usize) -> () {
        self.discard_in += len;
    }
}

struct ProxyLeftOut<'r> {
    left_out: &'r mut [u8],
    sent_out: usize,
}

impl<'r> TlsLeftOut for ProxyLeftOut<'r> {
    fn send_record_out(&mut self, data: &[u8]) -> () {

        let remaining = self.left_out.len() - self.sent_out;

        if remaining < data.len() {
            // TODO: Also need ot ensure we have enough left out buffers when spinning
            todo!("Need to temp buffer in the overflow.");
        }

        let start = self.sent_out;
        let end = self.sent_out + data.len();
        
        self.left_out[start..end].copy_from_slice(data);
        self.sent_out += data.len();
    }
}

struct ProxyRight<'r, R> {
    r: &'r mut R,
}

impl<'r, R: Right> TlsRight for ProxyRight<'r, R> {
    fn on_decrypted(&mut self, data: &[u8]) -> () {
        println!("on_decrypt Called");
        self.r.add_right_in(data);
        self.r.set_wants_right_next_in(true);
    }
    fn on_encrypt(&self) -> &[u8] {
        println!("on_encrypt Called buf_right_out.len = {}", self.r.buf_right_out().len());
        self.r.buf_right_out()
    }
    fn right_buf_mark_discard_out(&mut self, _len: usize) -> () {
        let buf_out = self.r.buf_right_out();
        todo!("discard _len: {} vs out_buf: {}", _len, buf_out.len());
    }
}

impl TlsServer {
    /// Construct new
    pub fn with_config(config: TlsServerConfig) -> Result<Self, TlsError> {

        let rng = rand::rng();
        let crypto_cfg = ytls_rustcrypto::RustCrypto;

        let mut tls_ctx = TlsServerCtx::with_required(
            config, crypto_cfg, rng
        );
        Ok(Self { ctx: tls_ctx })
    }
    /// Advance the state machine
    #[inline]
    pub fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<TlsPosition, TlsError> {
        let (left_in_len, left_out_len) = l.left_lens();
        let (left_inputs, mut left_out_b) = l.left_bufs_mut();


        //if left_in_len == 0 {
        //    return Ok(TlsPosition {});
        //}

        let mut new_len_in = left_in_len;
        let mut new_len_out = left_out_len;

        println!("Initial lens = (in={}, out={})", new_len_in, new_len_out); 
        
        left_out_b.split_off_mut(..left_out_len);
        
        match &left_inputs {
            InBuffer::Single(ref b) => {
                println!("Left input[{}] = {}", left_in_len, hex::encode(b));
            }
            _ => todo!(),
        }

        let mut left_in = ProxyLeftIn {
            left_in: left_inputs,
            discard_in: 0,
        };

        let mut left_out = ProxyLeftOut {
            left_out: left_out_b,
            sent_out: 0,
        };

        let mut right_proxy = ProxyRight {
            r,
        };

        self.ctx
            .advance_with(&mut left_in, &mut left_out, &mut right_proxy)
            .unwrap();

        if left_out.sent_out > 0 {
            new_len_out += left_out.sent_out;
        }

        if left_in.discard_in > 0 {
            new_len_in -= left_in.discard_in;
        }

        println!("New lens = (in={}, out={})", new_len_in, new_len_out); 

        if !self.ctx.is_handshaking() {
            l.set_ready(true);
            l.set_left_in_blocked(false);
        }
        
        l.left_set_lens(new_len_in, new_len_out);

        Ok(TlsPosition {})
        /*
        left_out_b.split_off_mut(..left_out_len);

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
        */

    }
    
}

    
