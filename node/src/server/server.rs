use crate::handle_tls;
use crate::server::tls;
use log::{error, info};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

pub struct Server {
    address: String,
    acceptor: TlsAcceptor,
    listener: TcpListener,
}

impl Server {
    pub async fn new(
        address: String,
        port: u16,
        tls_key_path: String,
        tls_cert_path: String,
    ) -> anyhow::Result<Server> {
        let addr = format!("{}:{}", address, port);
        Ok(Server {
            address: addr.clone(),
            acceptor: TlsAcceptor::from(std::sync::Arc::new(tls::load_config(
                tls_key_path,
                tls_cert_path,
            )?)),
            listener: TcpListener::bind(addr).await?,
        })
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        info!("Quido node listening on {}", self.address);

        loop {
            let (tcp, peer) = self.listener.accept().await?;
            let acceptor = self.acceptor.clone();

            tokio::spawn(async move {
                let tls = match acceptor.accept(tcp).await {
                    Ok(s) => s,
                    Err(e) => {
                        log::warn!("TLS handshake failed with {}: {}", peer, e);
                        return;
                    }
                };

                if let Err(e) = async {
                    handle_tls(tls, peer).await;
                    Ok::<(), anyhow::Error>(())
                }
                .await
                {
                    error!("Connection handler failed for {peer}: {e}");
                }

                info!("Accepted TLS connection from {peer}");
            });
        }
    }
}
