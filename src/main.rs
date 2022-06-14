use crate::server::Server;
use clap::Parser;

mod http;
mod pool;
mod server;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    source: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    let config = server::Config {
        port: 5001,
        source: String::from(args.source.unwrap_or_default().to_str().unwrap_or_default()),
        ..Default::default()
    };

    Server::new(config).listen();
}