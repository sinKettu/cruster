mod utils;
mod cruster_handler;
mod ui;
mod config;

use std::net::{IpAddr, SocketAddr};
use hudsucker::{
    ProxyBuilder,
    certificate_authority::OpensslAuthority
};
use tokio::{
    self,
    sync::mpsc::{channel, Sender}
};
use cruster_handler::request_response::CrusterWrapper;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

async fn start_proxy(socket_addr: SocketAddr, ca: OpensslAuthority, tx: Sender<CrusterWrapper>) {
    let proxy = ProxyBuilder::new()
        .with_addr(socket_addr)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(cruster_handler::CrusterHandler{proxy_tx: tx})
        .build();

    proxy.start(shutdown_signal()).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), utils::CrusterError> {
    let config = config::handle_user_input()?;
    utils::generate_key_and_cer(&config.tls_key_name, &config.tls_cer_name);
    let ca = utils::get_ca(&config.tls_key_name, &config.tls_cer_name)?;
    let socket_addr = SocketAddr::from((
        config
            .address
            .parse::<IpAddr>()?,
        config.port
    ));

    let (mut proxy_tx, mut ui_rx) = channel(100);
    tokio::spawn(async move { start_proxy(socket_addr, ca, proxy_tx).await; });
    ui::render(ui_rx).await?;
    Ok(())
}
