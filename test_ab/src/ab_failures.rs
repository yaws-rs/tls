//! Failure modes injection to test failures when it should fail

/// Expected failures to trigger & test-catch panics
pub enum ExpectedFailure {
    /// Client failed to verify server certificate
    ServerCertVerifyFailed,
    /// We think the server should not be a teapot
    ItsNotATeapot,
}
