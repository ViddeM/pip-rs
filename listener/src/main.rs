use std::{
    io::Write,
    net::{Ipv4Addr, TcpListener},
};

use clap::Parser;
use eyre::Context;
use log::{error, info};

#[derive(Parser)]
struct Args {
    #[clap(long, env = "BIND_IP")]
    ip: Ipv4Addr,
    #[clap(long, short, env = "PORT")]
    port: u16,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let args = Args::parse();

    let host = format!("{}:{}", args.ip.to_string(), args.port);

    let listener = TcpListener::bind(host.clone()).wrap_err("Failed to setup TCP listener")?;

    info!("Started listening on: {host}");

    for stream in listener.incoming() {
        let mut s = match stream {
            Ok(s) => s,
            Err(err) => {
                error!("Failed to accept connection due to: {err}");
                continue;
            }
        };

        let peer = match s.peer_addr() {
            Ok(p) => p,
            Err(err) => {
                error!("Failed to read peer address due to: {err}");
                continue;
            }
        };

        let ip_type = if peer.is_ipv4() { "IPv4" } else { "IPv6" };
        info!(
            "Peer connected from {ip_type} {} from port {} responding with IP",
            peer.ip(),
            peer.port()
        );

        if let Err(err) = s.write(format!("{ip_type}: {}", peer.ip().to_string()).as_bytes()) {
            error!("Failed to send response, sadge: {err}");
            continue;
        }
    }

    Ok(())
}
