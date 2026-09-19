//! A/B Traits

/// A/B Test (Server0 Adapter for impl testing
pub trait AbServerAdapter {
    /// Relevant TLS Server under test impl
    type TlsServer;
    /// Instantiate TLS Server Context
    fn init_server() -> impl AbServerCtx;
}

// blueprint_ytls::TlsServerOrbit
// blueprint_rustls::TlsContext

/// A/B Test server context
pub trait AbServerCtx {}
