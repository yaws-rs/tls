//! A/B Injector injects into std Read call to spin the state machine in between

use crate::AbStreamForward;
use crate::TestingCtx;
use blueprint::{Left, Orbit, Right};

pub(crate) fn ab_read_injector<O: Orbit>(
    u: &mut TestingCtx<O>,
    to_client: &mut [u8],
    to_server: &mut [u8],
) -> AbStreamForward {
    // Process network layer / TLS data from Left side
    u.l.copy_in(to_server);

    struct User;
    let r = u.o.advance_with(&mut User, &mut u.l, &mut u.r);

    let (l_pending_in, l_pending_out) = u.l.left_lens();

    if to_client.len() == 0 {
        panic!("No RX headroom for read.");
    }

    // Process application layer data from Right side and send back to Left
    // Canon way is for App to consume Left (mirrored Right by I/O runtime)
    // but here we have a special case just direct from Right for testing
    let (r_pending_in, _) = u.r.right_lens();
    if r_pending_in > 0 {
        let app_buf_in = &u.r.in_bytes[..r_pending_in];
        println!("App: {}", core::str::from_utf8(app_buf_in).unwrap());

        match core::str::from_utf8(app_buf_in) {
            Ok("GET / HTTP/1.0\r\n\r\n") => {}
            _ => panic!("Wrong cleartext data received."),
        }

        u.r.in_bytes_len = 0;

        let send_out = b"418 I'm a teapot\r\n\r\n";

        u.r.add_right_out(send_out);
        let r = u.o.advance_with(&mut User, &mut u.l, &mut u.r);

        let (l_pending_in, l_pending_out) = u.l.left_lens();

        println!(
            "After App left pending in/{} out/{}",
            l_pending_in, l_pending_out
        );
    }

    let (l_pending_in, l_pending_out) = u.l.left_lens();
    if l_pending_out > 0 {
        let copy_len = if to_client.len() < l_pending_out {
            to_client.len()
        } else {
            l_pending_out
        };
        to_client[..copy_len].copy_from_slice(&u.l.out_bytes[..copy_len]);
        let new_len = l_pending_out - copy_len;
        u.l.left_set_lens(l_pending_in, new_len);

        return AbStreamForward {
            discard_write: to_server.len(),
            read_status: Ok(copy_len),
        };
    }
    AbStreamForward {
        discard_write: to_server.len(),
        read_status: Ok(0),
    }
}
