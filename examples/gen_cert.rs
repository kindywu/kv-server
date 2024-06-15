use anyhow::Result;
use certify::{generate_ca, generate_cert, CertSigAlgo, CertType, CA};
use tokio::fs;

struct CertPem {
    cert_type: CertType,
    cert: String,
    key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (cert, key) = generate_ca(
        "acme.inc",
        "CN",
        "Acme Inc.",
        CertSigAlgo::ED25519,
        None,
        Some(10 * 365),
    )?;

    let ca = CA::load(&cert, &key)?;

    let (server_cert, server_key) = generate_cert(
        &ca,
        vec!["kvserver.acme.inc"],
        "CN",
        "Acme Inc.",
        "Acme KV server",
        CertSigAlgo::ED25519,
        None,
        false,
        Some(365),
    )?;

    let (client_cert, client_key) = generate_cert(
        &ca,
        Vec::<&str>::new(),
        "CN",
        "Acme Inc.",
        "awesome-device-id",
        CertSigAlgo::ED25519,
        None,
        false,
        Some(365),
    )?;

    let pem = CertPem {
        cert_type: CertType::CA,
        cert,
        key,
    };

    gen_files(&pem).await?;

    let pem = CertPem {
        cert_type: CertType::Server,
        cert: server_cert,
        key: server_key,
    };

    gen_files(&pem).await?;

    let pem = CertPem {
        cert_type: CertType::Client,
        cert: client_cert,
        key: client_key,
    };

    gen_files(&pem).await?;
    Ok(())
}

async fn gen_files(pem: &CertPem) -> Result<()> {
    let name = match pem.cert_type {
        CertType::Client => "client",
        CertType::Server => "server",
        CertType::CA => "ca",
    };
    fs::write(format!("fixtures/{}.cert", name), pem.cert.as_bytes()).await?;
    fs::write(format!("fixtures/{}.key", name), pem.key.as_bytes()).await?;
    Ok(())
}
