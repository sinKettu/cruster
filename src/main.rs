mod utils;
mod cruster_handler;
mod ui;

use hudsucker::ProxyBuilder;
use std::net::SocketAddr;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() -> Result<(), utils::CrusterError> {
    let ca = utils::get_ca(
        "target/debug/ca/hudsucker.key",
        "target/debug/ca/hudsucker.cer"
    )?;

    let proxy = ProxyBuilder::new()
        .with_addr(SocketAddr::from(([127, 0, 0, 1], 3000)))
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(cruster_handler::CrusterHandler)
        .build();

    proxy.start(shutdown_signal()).await?;
    Ok(())
}