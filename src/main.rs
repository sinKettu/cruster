mod utils;
mod cruster_proxy;
mod config;
mod http_storage;
mod siv_ui;
mod scope;
mod dump;


#[cfg(feature = "rcgen-ca")]
use hudsucker::{ProxyBuilder, certificate_authority::{RcgenAuthority as HudSuckerCA}};

#[cfg(feature = "openssl-ca")]
use hudsucker::{ProxyBuilder, certificate_authority::{OpensslAuthority as HudSuckerCA}};

use tokio::{
    self,
    sync::mpsc::{channel, Sender},
};

use utils::CrusterError;
use cursive::{Cursive, CbSink};
use std::{net::{IpAddr, SocketAddr}, process::exit};
use crossbeam_channel::Sender as CB_Sender;
use crossbeam_channel::{unbounded, Sender as CrusterSender, Receiver as CrusterReceiver};
use cruster_proxy::{CrusterHandler, CrusterWSHandler, events::ProxyEvents};
use dump::DumpMode;

// use log::debug;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

async fn start_proxy(
        socket_addr: SocketAddr,
        ca: HudSuckerCA,
        tx: CrusterSender<ProxyEvents>,
        err_tx: Sender<CrusterError>,
        cursive_sink: CbSink
    ) {

    let proxy = ProxyBuilder::new()
        .with_addr(socket_addr)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(
            CrusterHandler {
                proxy_tx: tx.clone(),
                err_tx: err_tx.clone(),
                cursive_sink,
                request_hash: 0
            }
        )
        .with_websocket_handler(
            CrusterWSHandler {
                proxy_tx: tx.clone()
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
    let ca: HudSuckerCA = utils::get_ca(&config.tls_key_name, &config.tls_cer_name)?;

    let socket_addr = SocketAddr::from((
        config
            .address
            .parse::<IpAddr>()?,
        config.port
    ));

    let (tx, rx): (CrusterSender<ProxyEvents>, CrusterReceiver<ProxyEvents>) = unbounded();
    let (err_tx, err_rx) = channel(10);

    let siv = Cursive::default();
    let cb_sink: CB_Sender<Box<dyn FnOnce(&mut Cursive)+Send>> = siv.cb_sink().clone();

    tokio::task::spawn(
        async move {
            start_proxy(
                socket_addr,
                ca,
                tx,
                err_tx,
                cb_sink,
            ).await
        }
    );

    if config.dump_mode_enabled() {
        tokio::task::spawn(
            async move {
                dump::launch_dump(rx).await;
            }
        );

        match tokio::signal::ctrl_c().await {
            Ok(_) => { exit(0); },
            Err(err) => {
                panic!("Unable to listen for shutdown signal: {}", err);
            }
        }
    }
    else {
        siv_ui::bootstrap_ui(siv, config, rx, err_rx);
        Ok(())
    }
}
