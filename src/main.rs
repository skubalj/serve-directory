#![doc = include_str!("../README.md")]

use args::Arguments;
use clap::Parser;
use log::{LevelFilter, info};
use std::net::{IpAddr, SocketAddr};
use std::sync::LazyLock;
use tokio::signal::ctrl_c;

mod args;
mod routes;

/// The command line arguments passed into the program, parsed on first use
static ARGUMENTS: LazyLock<Arguments> = LazyLock::new(Arguments::parse);

#[tokio::main]
async fn main() {
    initialize_logger(ARGUMENTS.verbose);

    let socket_addr = SocketAddr::from((ip_addr(ARGUMENTS.localhost), ARGUMENTS.port));
    let server = warp::serve(routes::routes()).bind(socket_addr).await;
    info!("Server listening on http://{socket_addr}");

    server
        .graceful(async {
            ctrl_c().await.expect("Unable to get Ctrl+C signal");
            info!("Ctrl+C received. Shutting down");
        })
        .run()
        .await;
}

/// The address to bind to: loopback, or this machine's address on the local network
fn ip_addr(localhost: bool) -> IpAddr {
    if localhost {
        IpAddr::from([127, 0, 0, 1])
    } else {
        local_ipaddress::get()
            .expect("No local IP address found")
            .parse()
            .expect("Found IP address is invalid")
    }
}

/// Initialize the logger to print output
fn initialize_logger(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::builder()
        .format_target(false)
        .filter(Some(env!("CARGO_CRATE_NAME")), level)
        .init()
}
