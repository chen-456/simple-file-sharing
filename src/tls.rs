use std::{fs::File, io::BufReader};

use anyhow::Context;
use rustls::{Certificate, PrivateKey, ServerConfig};

pub fn rustls_config() -> anyhow::Result<ServerConfig> {
    let mut key_ders = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(
        File::open("config/server-key.pem").context("open config/server-key.pem")?,
    ))
    .context("parse config/server-key.pem")?;
    let cert_chain = rustls_pemfile::certs(&mut BufReader::new(
        File::open("config/server-cert.cer").context("open config/server-cert.cer")?,
    ))
    .context("parse server-cert.cer")?;

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(
            cert_chain.into_iter().map(Certificate).collect(),
            PrivateKey(key_ders.pop().unwrap()),
        )
        .map_err(Into::into)
}
