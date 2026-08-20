//! Yaws TLS Blueprint & Orbit

use crate::TlsError;
use crate::TlsPosition;
use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint::{Left, Right};

use crate::{TlsClient, TlsServer};
use crate::{TlsClientConfig, TlsServerConfig};

/// .
pub enum TlsContext {
    /// .
    Client(TlsClient),
    /// .
    Server(TlsServer),
}

impl Orbit for TlsContext {
    type Position = TlsPosition;
    type Error = TlsError;
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        match self {
            Self::Client(c) => c.advance_with(_u, l, r),
            Self::Server(s) => s.advance_with(_u, l, r),
        }
    }
}

impl BluePrint<TlsContext> for TlsServer {
    type Config = TlsServerConfig;
    type Error = TlsError;

    fn with_defaults() -> Result<TlsContext, Self::Error> {
        todo!()
    }
    fn with_configuration(c: Self::Config) -> Result<TlsContext, Self::Error> {
        Ok(TlsContext::Server(TlsServer::with_config(c)?))
    }
}

impl BluePrint<TlsContext> for TlsClient {
    type Config = TlsClientConfig;
    type Error = TlsError;

    fn with_defaults() -> Result<TlsContext, Self::Error> {
        todo!()
    }
    fn with_configuration(c: Self::Config) -> Result<TlsContext, Self::Error> {
        Ok(TlsContext::Client(TlsClient::with_config(c)?))
    }
}
