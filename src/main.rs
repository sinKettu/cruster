mod utils;
mod cruster_proxy;
mod config;
mod http_storage;
mod siv_ui;

use std::net::{IpAddr, SocketAddr};
use hudsucker::{ProxyBuilder, certificate_authority::OpensslAuthority};
use tokio::{
    self,
    sync::mpsc::{channel, Sender},
    signal
};
use cruster_proxy::{CrusterHandler, CrusterWSHandler, request_response::CrusterWrapper};
use utils::CrusterError;

use cursive::{Cursive, CbSink};
use crossbeam_channel::Sender as CB_Sender;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

async fn start_proxy(
        socket_addr: SocketAddr,
        ca: OpensslAuthority,
        tx: Sender<(CrusterWrapper, usize)>,
        err_tx: Sender<CrusterError>,
        cursive_sink: CbSink,
        dump_mode: bool) {

    let proxy = ProxyBuilder::new()
        .with_addr(socket_addr)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(
            CrusterHandler {
                proxy_tx: tx,
                err_tx: err_tx.clone(),
                dump: dump_mode,
                cursive_sink,
                request_hash: 0
            }
        )
        .with_incoming_message_handler(
            CrusterWSHandler {
                dump: dump_mode,
                from_client: false
            }
        )
        .with_outgoing_message_handler(
            CrusterWSHandler {
                dump: dump_mode,
                from_client: true
            }
        )
        .build();

    let result = proxy.start(shutdown_signal()).await;
    if let Err(e) = result {
        err_tx
            .send(e.into())
            .await
            .unwrap_or_else(|send_error| {
                panic!("Could not communicate with UI thread: {}", send_error.to_string())
            });
    }
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

    let (proxy_tx, ui_rx) = channel(10);
    let (err_tx, err_rx) = channel(10);

    let siv = Cursive::default();
    let cb_sink: CB_Sender<Box<dyn FnOnce(&mut Cursive)+Send>> = siv.cb_sink().clone();

    tokio::task::spawn(
        async move {
            start_proxy(
                socket_addr,
                ca,
                proxy_tx,
                err_tx,
                cb_sink,
                config.dump_mode
            ).await
        });

    if config.dump_mode {
        match signal::ctrl_c().await {
            Ok(_) => Ok(()),
            Err(err) => {
                panic!("Unable to listen for shutdown signal: {}", err);
            },
        }
    }
    else {
        siv_ui::bootstrap_ui(siv, ui_rx, err_rx);
        Ok(())
    }
}
