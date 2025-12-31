mod server;

use clap::Parser;
use log::warn;
use simplelog::Config;
use tokio::time::{Duration, timeout};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Listen IP address
    #[arg(short = 'a', long = "address", default_value = "0.0.0.0")]
    listen_address: String,

    /// Listen port
    #[arg(short = 'p', long = "port", default_value_t = 443)]
    listen_port: u16,

    /// TLS key file path
    #[arg(short = None, long = "tls-key")]
    tls_key_path: String,

    /// TLS certificate file path
    #[arg(short = None, long = "tls-cert")]
    tls_cert_path: String,

    /// Enable verbose mode
    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,

    /// Enable colorized output
    #[arg(short = 'c', long = "color", default_value_t = false)]
    color: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    simplelog::TermLogger::init(
        if args.verbose {
            simplelog::LevelFilter::Info
        } else {
            simplelog::LevelFilter::Warn
        },
        Config::default(),
        simplelog::TerminalMode::Mixed,
        if args.color {
            simplelog::ColorChoice::Always
        } else {
            simplelog::ColorChoice::Never
        },
    )?;

    let server = server::server::Server::new(
        args.listen_address,
        args.listen_port,
        args.tls_key_path,
        args.tls_cert_path,
    )
    .await?;

    server.serve().await?;

    Ok(())
}

pub async fn handle_tls<S>(mut stream: S, peer: std::net::SocketAddr)
where
    S: tokio::io::AsyncReadExt + tokio::io::AsyncWriteExt + Unpin,
{
    let mut first = [0u8; 1];
    match timeout(Duration::from_secs(5), stream.read_exact(&mut first)).await {
        Ok(Ok(size)) if size == 1 => {}
        _ => {
            warn!("Failed to read from stream for {peer}");
            return;
        }
    }

    if first[0] != 0x01 {
        // if let Err(e) = handle_http_1_1(stream, first[0]).await {
        //     eprintln!("HTTP handler error: {e}");
        // }
    } else {
        // handle_tcp(stream).await;
    }
}
