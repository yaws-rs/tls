//! Yaws TLS (yTLS) Blueprint & Orbit

use crate::TlsError;
use crate::TlsPosition;
use blueprint::Orbit;
use blueprint::{Left, Right};

use ::ytls_traits::{CryptoConfig, CryptoRng};

#[cfg(feature = "server")]
use ytls_server::TlsServerCtxConfig;

#[cfg(feature = "server")]
use crate::TlsServer;

/// yTLS Server Orbit
#[cfg(feature = "server")]
pub struct TlsServerOrbit<Config, Crypto, Rng> {
    server: TlsServer<Config, Crypto, Rng>,
}

impl<Config, Crypto, Rng> TlsServerOrbit<Config, Crypto, Rng>
where
    Config: TlsServerCtxConfig,
    Crypto: CryptoConfig + Clone,
    Rng: CryptoRng,
{
    /// Initialize a yTLS Orbit with the required Configuration, CryptoConfig & CryptoRng
    pub fn with_required(c: Config, crypto: Crypto, rng: Rng) -> Self {
        Self {
            server: TlsServer::with_required(c, crypto, rng),
        }
    }
}

#[cfg(feature = "server")]
impl<Config, Crypto, Rng> Orbit for TlsServerOrbit<Config, Crypto, Rng>
where
    Config: TlsServerCtxConfig,
    Crypto: CryptoConfig + Clone,
    Rng: CryptoRng,
{
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
