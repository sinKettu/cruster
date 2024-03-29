mod utils;
mod cruster_proxy;
mod config;
mod http_storage;
mod siv_ui;
mod scope;
mod dump;
mod cli;


#[cfg(feature = "rcgen-ca")]
use hudsucker::{ProxyBuilder, certificate_authority::RcgenAuthority as HudSuckerCA};

#[cfg(feature = "openssl-ca")]
use hudsucker::{ProxyBuilder, certificate_authority::OpensslAuthority as HudSuckerCA};

use tokio;
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
        cursive_sink: CbSink,
        dump: bool
    ) {

    let proxy = ProxyBuilder::new()
        .with_addr(socket_addr)
        .with_native_tls_client()
        .with_ca(ca)
        .with_http_handler(
            CrusterHandler {
                proxy_tx: tx.clone(),
                dump,
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

    // TODO: something better than unwrap()
    proxy.start(shutdown_signal()).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), utils::CrusterError> {
    let (config, mode) = config::handle_user_input()?;

    if let config::CrusterMode::CLI(subcmd_args) = mode {
        if let Err(err) = cli::launch(subcmd_args, config).await {
            let err_str: String = err.into();
            eprintln!("Error in Cruster CLI: {}", err_str);
            exit(-1);
        }
        else {
            return Ok(());
        }
    }

    utils::generate_key_and_cer(&config.tls_key_name, &config.tls_cer_name);
    let ca: HudSuckerCA = utils::get_ca(&config.tls_key_name, &config.tls_cer_name)?;

    let socket_addr = SocketAddr::from((
        config
            .address
            .parse::<IpAddr>()?,
        config.port
    ));

    let (tx, rx): (CrusterSender<ProxyEvents>, CrusterReceiver<ProxyEvents>) = unbounded();
    let siv = Cursive::default();
    let cb_sink: CB_Sender<Box<dyn FnOnce(&mut Cursive)+Send>> = siv.cb_sink().clone();
    let dump_mode = config.dump_mode_enabled();

    tokio::task::spawn(
        async move {
            start_proxy(
                socket_addr,
                ca,
                tx,
                cb_sink,
                dump_mode
            ).await
        }
    );

    if config.dump_mode_enabled() {
        let dump_thread = tokio::task::spawn(
            async move {
                dump::launch_dump(rx, config).await;
            }
        );

        tokio::task::spawn(
            async move {
                if let Err(err) = dump_thread.await {
                    eprintln!("{}", err);
                    exit(1);
                }
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
        siv_ui::bootstrap_ui(siv, config, rx);
        Ok(())
    }
}
