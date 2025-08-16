//! Yaws TLS Blueprint & Orbit

use crate::TlsError;
use blueprint::BluePrint;
use blueprint::Orbit;

use crate::{TlsClient, TlsServer};
use crate::{TlsClientConfig, TlsServerConfig};

/// .
pub struct TlsPosition {}

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
    fn advance_with<B>(&mut self, _: &mut B, _: &mut [u8]) -> Result<Self::Position, Self::Error> {
        todo!()
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
