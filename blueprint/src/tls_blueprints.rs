//! Yaws TLS Blueprint & Orbit

use crate::TlsError;
use blueprint::Orbit;
use blueprint::BluePrint;

use crate::{TlsClientConfig, TlsServerConfig};
use crate::{TlsClient, TlsServer};

/// .
pub struct TlsPosition {
}

/// .
pub enum TlsContext<'c> {
    /// .
    Client(TlsClient<'c>),
    /// .
    Server(TlsServer<'c>),
}

impl<'c> Orbit for TlsContext<'c> {
    type Position = TlsPosition;
    type Error = TlsError;
    fn advance_with<B>(&mut self, _: &mut B, _: &mut [u8]) -> Result<Self::Position, Self::Error> {
        todo!()
    }
}

impl<'c> BluePrint<TlsContext<'c>> for TlsServer<'c> {
    type Config = TlsServerConfig<'c>;
    type Error = TlsError;    
    
    fn with_defaults() -> Result<TlsContext<'c>, Self::Error> {
        todo!()
    }
    fn with_configuration(c: Self::Config) -> Result<TlsContext<'c>, Self::Error> {
        Ok(TlsContext::<'c>::Server(TlsServer::<'c>::with_config(c)?))
    }
}

impl<'c> BluePrint<TlsContext<'c>> for TlsClient<'c> {
    type Config = TlsClientConfig<'c>;
    type Error = TlsError;

    fn with_defaults() -> Result<TlsContext<'c>, Self::Error> {
        todo!()
    }
    fn with_configuration(c: Self::Config) -> Result<TlsContext<'c>, Self::Error>{
        Ok(TlsContext::Client(TlsClient::<'c>::with_config(c)?))
    }    
}
