#![doc = include_str!("../README.md")]

use clap::Parser;
use log::{LevelFilter, info};
use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::LazyLock;
use tokio::signal::ctrl_c;

mod routes;

/// The command line arguments passed into the program, parsed on first use
static ARGUMENTS: LazyLock<Arguments> = LazyLock::new(Arguments::parse);

/// Serve files in the specified directory and subdirectories
#[derive(Debug, Parser)]
#[command(version)]
pub struct Arguments {
    /// The root directory that should be served
    #[arg(default_value = ".")]
    pub folder: PathBuf,

    /// The port that the server should bind to
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Deploy on localhost, rather than binding to the external interface
    #[arg(short, long)]
    pub localhost: bool,

    /// Log additional data to the console
    #[arg(short, long)]
    pub verbose: bool,
}

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
fn ip_addr(localhost: bool) -> Ipv4Addr {
    if localhost {
        Ipv4Addr::LOCALHOST
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
