//! Yaws TLS (yTLS) Blueprint & Orbit

use crate::TlsError;
use crate::TlsPosition;
use blueprint::BluePrint;
use blueprint::Orbit;
use blueprint::{Left, Right};

#[cfg(feature = "server")]
use crate::{TlsServer, TlsServerConfig};

#[cfg(feature = "server")]
use ytls_server::TlsServerCtxConfig;

#[cfg(feature = "server")]
pub struct TlsServerOrbit {
    server: TlsServer,
}

#[cfg(feature = "server")]
impl Orbit for TlsServerOrbit {
    type Position = TlsPosition;
    type Error = TlsError;
    fn advance_with<B, L: Left, R: Right>(
        &mut self,
        _u: &mut B,
        l: &mut L,
        r: &mut R,
    ) -> Result<Self::Position, Self::Error> {
        self.server.advance_with(_u, l, r)
    }    
}

#[cfg(feature = "server")]
impl BluePrint<TlsServerOrbit> for TlsServer {
    type Config = TlsServerConfig;
    type Error = TlsError;

    fn with_defaults() -> Result<TlsServerOrbit, Self::Error> {
        todo!()
    }
    fn with_configuration(c: Self::Config) -> Result<TlsServerOrbit, Self::Error> {
        Ok(TlsServerOrbit { server: TlsServer::with_config(c)? })
    }
}
