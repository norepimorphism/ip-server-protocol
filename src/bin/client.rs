// SPDX-License-Identifier: MPL-2.0

use std::{env, io::{Read as _, Write as _}, net::{Ipv4Addr, SocketAddr, TcpStream}};

use ip_service_protocol::Query;

fn main() {
    ip_service_protocol::run(print_usage, access);
}

fn print_usage() {
    println!(
        "Usage: {} <queries> [remote-addr]",
        env::args().next().unwrap_or("ipsp-server".into()),
    );
}

fn access() {
    let mut args = env::args();

    let Some(query_string) = args.nth(1) else {
        eprintln!("Nothing to do");
        return;
    };

    let mut queries: Vec<Query> = query_string
        .as_bytes()
        .into_iter()
        .flat_map(|byte| Query::from_byte(*byte))
        .collect();
    queries.dedup_by(|a, b| *a == *b);

    let remote_addr = args
        .next()
        .and_then(|addr| addr.parse().ok())
        .unwrap_or(Ipv4Addr::LOCALHOST);
    let socket_addr: SocketAddr = (remote_addr, ip_service_protocol::PORT).into();

    let mut stream = TcpStream::connect(socket_addr)
        .expect("failed to connect to remote address");
    println!("Connected to {}", socket_addr);

    let _ = stream.write_all({
        queries.iter().map(|query| query.as_byte()).collect::<Vec<u8>>().as_ref()
    });
    let _ = stream.flush();

    let mut output = Vec::new();
    if stream.read_to_end(&mut output).is_ok() {
        println!();
        let mut responses: Vec<&[u8]> = output.split(|byte| *byte == 0).collect();
        let _ = responses.pop();

        for (query, response) in queries.into_iter().zip(responses.into_iter()) {
            let query_desc = query.to_string();
            println!("{}", query_desc);

            for _ in 0..query_desc.len() {
                print!("-");
            }
            println!();

            let response = String::from_utf8_lossy(response);
            let response = response.trim_end();

            println!("{}", response);
            println!();
        }
    } else {
        eprintln!("Bad server output");
    }
}
