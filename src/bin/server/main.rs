// SPDX-License-Identifier: MPL-2.0

use std::{
    env,
    io::{Read as _, Write as _},
    net::{Ipv4Addr, SocketAddr, TcpListener},
    time::Duration,
};

static PRIVACY_POLICY: &str = include_str!("privacy-policy.txt");
static TERMS_OF_SERVICE: &str = include_str!("terms-of-service.txt");
static CONTACT: &str = include_str!("contact.txt");

fn main() {
    ip_service_protocol::run(print_usage, serve);
}

fn print_usage() {
    println!("Usage: {} [local-addr]", env::args().next().unwrap_or("ipsp-server".into()));
}

fn serve() {
    let local_addr = env::args()
        .nth(1)
        .and_then(|addr| addr.parse().ok())
        .unwrap_or(Ipv4Addr::UNSPECIFIED);
    let socket_addr: SocketAddr = (local_addr, ip_service_protocol::PORT).into();

    let listener = TcpListener::bind(socket_addr)
        .expect("failed to bind to local address");
    for mut stream in listener.incoming().filter_map(Result::ok) {
        println!(
            "Connected to {}",
            stream.peer_addr().map(|it| it.to_string()).unwrap_or("remote".into()),
        );

        let _ = stream.set_read_timeout(Some(Duration::from_millis(500)));

        let mut input = [0; 4];
        let mut input_len = 0;
        loop {
            match stream.read(&mut input) {
                Ok(0) | Err(_) => break,
                Ok(len) => {
                    input_len += len;
                }
            }
        }
        if input_len == 0 {
            eprintln!("Input is empty");
            continue;
        }

        let mut byte_was_seen = [false; 256];
        for byte in input.into_iter().take(input_len) {
            // SAFETY: TODO
            let byte_was_seen = unsafe {
                byte_was_seen.get_unchecked_mut(usize::from(byte))
            };

            if *byte_was_seen {
                continue;
            }
            *byte_was_seen = true;

            let maybe_response = match byte {
                b'v' => Some(ip_service_protocol::VERSION),
                b'p' => Some(PRIVACY_POLICY),
                b't' => Some(TERMS_OF_SERVICE),
                b'c' => Some(CONTACT),
                _ => None,
            };

            if let Some(response) = maybe_response {
                let _ = stream.write_all(response.as_bytes());
                let _ = stream.write_all(&[0]);
            }
        }

        let _ = stream.flush();
    }
}
