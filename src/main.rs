mod utils;
mod cruster_handler;
mod ui;
mod config;

use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use hudsucker::{ProxyBuilder, certificate_authority::OpensslAuthority, HttpContext};
use tokio::{
    self,
    sync::mpsc::{channel, Sender}
};
use cruster_handler::{CrusterHandler, CrusterWSHandler, request_response::CrusterWrapper};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

async fn start_proxy(socket_addr: SocketAddr, ca: OpensslAuthority, tx: Sender<(CrusterWrapper, HttpContext)>) {
    let proxy = ProxyBuilder::new()
        .with_addr(socket_addr)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(cruster_handler::CrusterHandler{proxy_tx: tx})
        .with_incoming_message_handler(CrusterWSHandler)
        .with_outgoing_message_handler(CrusterWSHandler)
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

    let (proxy_tx, ui_rx) = channel(1);
    tokio::spawn(async move { start_proxy(socket_addr, ca, proxy_tx).await; });
    ui::render(ui_rx).await?;
    Ok(())
}
