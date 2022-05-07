mod utils;
mod cruster_handler;
mod ui;
mod config;

use hudsucker::ProxyBuilder;
use std::net::{IpAddr, SocketAddr};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
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

    let proxy = ProxyBuilder::new()
        .with_addr(socket_addr)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(cruster_handler::CrusterHandler)
        .build();

    proxy.start(shutdown_signal()).await?;
    ui::render().await?;
    Ok(())
}
