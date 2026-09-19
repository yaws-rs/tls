//! Tls position

/// Orbit position for Rustls
#[derive(Debug, Default)]
pub struct TlsPosition {
    /// Out discard size
    pub in_discard: usize,
    /// Out send size
    pub out_send: usize,
    /// Out encoded size
    pub out_encoded: usize,
    /// Want to write right side?
    pub want_write_right: bool,
    /// Peer closed?
    pub peer_closed: bool,
}

// TODO: make builder / typestate
impl TlsPosition {
    pub(crate) fn peer_closed(in_discard: usize) -> Self {
        Self {
            in_discard,
            peer_closed: true,
            ..Default::default()
        }
    }
    pub(crate) fn with_blocked_handshake(in_discard: usize) -> Self {
        Self {
            in_discard,
            ..Default::default()
        }
    }
    pub(crate) fn with_discard_only(in_discard: usize) -> Self {
        Self {
            in_discard,
            ..Default::default()
        }
    }
    /*
    pub(crate) fn with_encoded(in_discard: usize, out_encoded: usize) -> Self {
        Self {
            in_discard,
            out_encoded,
            ..Default::default()
        }
    } */
    pub(crate) fn with_send(in_discard: usize, out_send: usize) -> Self {
        Self {
            in_discard,
            out_send,
            ..Default::default()
        }
    }
    pub(crate) fn want_write_right(in_discard: usize) -> Self {
        Self {
            in_discard,
            want_write_right: true,
            ..Default::default()
        }
    }
}
