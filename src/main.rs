mod proxy;
mod ui;

use std::fmt::format;
use rcgen;
use std::path::Path;
use std::fs;
use shellexpand::tilde;

fn handle_certificates(raw_path: &str) -> Result<(), String> {
    let path = tilde(raw_path).to_string();
    let cert_name = format!("{}/cruster.cer", &path);
    let key_name = format!("{}/cruster.key", &path);
    let generate = |cert, key| -> Result<(), String> {
        match rcgen::generate_simple_self_signed(
            vec![
                String::from("cruster.intercepting.proxy"),
                String::from("localhost"),
                String::from("127.0.0.1")
            ]
        ) {
            Ok(certificate) => {
                fs::write(&cert_name, certificate.serialize_der().unwrap()).unwrap();
                fs::write(&key_name, certificate.serialize_private_key_der()).unwrap();
                Ok(())
            },
            Err(err) => Err(format!("Unable to generate certificates: {}", err))
        }
    };

    if !Path::new(&path).exists() {
        if let Err(err) = fs::create_dir(&path) {
            return Err(format!("Unable to generate certificates: {}", err));
        }
        return generate(&cert_name, &key_name);
    }
    else if !(
        Path::new(&cert_name).exists()
        && Path::new((&key_name)).exists()
    ) {
        return generate(&cert_name, &key_name);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // Generating self-signed certificate if there is no one
    if let Err(err) = handle_certificates("~/.cruster") {
        return Err(err);
    }

    ui::render().await;
    proxy::run_proxy();
    Ok(())
}
