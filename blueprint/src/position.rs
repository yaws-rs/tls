//! Tls position

/// .
#[derive(Debug)]
pub struct TlsPosition {
    pub in_discard: usize,
    pub out_send: usize,
    pub out_encoded: usize,
}
