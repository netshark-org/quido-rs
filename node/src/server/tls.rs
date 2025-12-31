use rustls::ServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_config(key_path: String, cert_path: String) -> anyhow::Result<ServerConfig> {
    if !Path::new(&key_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("TLS key not found: {}", key_path),
        )
        .into());
    }

    let key = {
        let mut pkcs8_key_reader: BufReader<File> = BufReader::new(File::open(key_path.clone())?);
        let mut pkcs8_key = rustls_pemfile::pkcs8_private_keys(&mut pkcs8_key_reader);

        match pkcs8_key.next() {
            Some(Ok(key)) => PrivateKeyDer::Pkcs8(key),
            _ => {
                let mut rsa_keys_reader = BufReader::new(File::open(key_path)?);
                let rsa_key = rustls_pemfile::rsa_private_keys(&mut rsa_keys_reader)
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Invalid private key"))??;
                PrivateKeyDer::Pkcs1(rsa_key)
            }
        }
    };

    if !Path::new(&cert_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("TLS certificate not found: {}", cert_path),
        )
        .into());
    }

    let tls_cert = {
        let mut cert_reader = BufReader::new(File::open(cert_path)?);
        rustls_pemfile::certs(&mut cert_reader).collect::<Result<Vec<CertificateDer>, _>>()?
    };

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_cert, key)?;

    Ok(config)
}
