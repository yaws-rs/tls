//! A/B Testing driver

pub use blueprint::{Left, NoLeft, NoRight, Orbit, Right};

use crate::AbStream;
use crate::{AbIoLeft, AbIoRight};

use openssl::ssl::{SslConnector, SslMethod};

/// Testing Orbit Context
pub struct TestingCtx<O> {
    /// Orbit under test
    pub o: O,
    /// Left side of Orbit
    pub l: AbIoLeft,
    /// Right side of Orbit
    pub r: AbIoRight,
}

/// Create a testing context for an Orbit
pub fn testing_ctx(orbit: impl Orbit) -> TestingCtx<impl Orbit> {
    let mut ctx = TestingCtx {
        o: orbit,
        l: AbIoLeft::default(),
        r: AbIoRight::default(),
    };

    ctx.l.out_bytes = vec![0; 16384];
    ctx.l.out_bytes_len = 0;

    ctx
}

#[test]
fn init_rustls() {
    let mut _test_ctx = testing_ctx(crate::ab_rustls::init_server());
}

#[test]
fn init_ytls() {
    let mut _test_ctx = testing_ctx(crate::ab_ytls::init_server());
}

use crate::ExpectedFailure;

/// Trigger a teapot testing
pub fn teapot(ctx: impl Orbit, exp_failure: Option<ExpectedFailure>) {
    let test_ctx = testing_ctx(ctx);

    let mut openssl_build = SslConnector::builder(SslMethod::tls()).unwrap();

    openssl_build.set_ca_file(crate::CA).unwrap();

    let openssl = openssl_build.build();

    use crate::AbStreamForward;

    let client = AbStream::with_read_injector(test_ctx, crate::ab_injector::ab_read_injector);

    let hostname = match exp_failure {
        Some(ExpectedFailure::ServerCertVerifyFailed) => "here.we.fail",
        _ => "test.rustcryp.to",
    };

    use openssl::ssl::HandshakeError;
    let mut ssl_stream = match openssl.connect(hostname, client) {
        Ok(s) => s,
        Err(HandshakeError::SetupFailure(e)) => panic!("Setup Failure: {:?}", e),
        Err(HandshakeError::Failure(mid_handshake)) => {
            panic!("Failure mid-handshake: {:?}", mid_handshake.into_error())
        }
        Err(HandshakeError::WouldBlock(mid_handshake)) => {
            panic!("WouldBlock mid-handshake {:?}", mid_handshake.into_error())
        }
    };

    //************************************************
    // Application layer
    //************************************************
    use std::io::{Read, Write};
    ssl_stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();

    let mut app_recvd: Vec<u8> = vec![0; 1024];

    let bytes_in = ssl_stream.read(&mut app_recvd).unwrap();

    let response = core::str::from_utf8(&app_recvd[..bytes_in]);

    let expected_response = match exp_failure {
        Some(ExpectedFailure::ItsNotATeapot) => "200 OK",
        _ => "418 I'm a teapot\r\n\r\n",
    };

    assert_eq!(response, Ok(expected_response), "unexpected response");
}

#[test]
fn teapot_rustls() {
    teapot(crate::ab_ytls::init_server(), None);
}

#[test]
fn teapot_ytls() {
    teapot(crate::ab_ytls::init_server(), None);
}

#[test]
#[should_panic = "certificate verify failed"]
fn teapot_rustls_cert_verify_failed() {
    let fail = Some(ExpectedFailure::ServerCertVerifyFailed);
    teapot(crate::ab_ytls::init_server(), fail);
}

#[test]
#[should_panic = "certificate verify failed"]
fn teapot_ytls_cert_verify_failed() {
    let fail = Some(ExpectedFailure::ServerCertVerifyFailed);
    teapot(crate::ab_ytls::init_server(), fail);
}

#[test]
#[should_panic = "unexpected response"]
fn teapot_rustls_teapot_not_a_teapot() {
    let fail = Some(ExpectedFailure::ItsNotATeapot);
    teapot(crate::ab_ytls::init_server(), fail);
}

#[test]
#[should_panic = "unexpected response"]
fn teapot_ytls_teapot_not_a_teapot() {
    let fail = Some(ExpectedFailure::ItsNotATeapot);
    teapot(crate::ab_ytls::init_server(), fail);
}
