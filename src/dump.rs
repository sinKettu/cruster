use crate::cruster_proxy::events::{ProxyEvents};
use crossbeam_channel::Receiver;
use hudsucker::WebSocketContext;
use bstr::ByteSlice;

pub(super) async fn launch_dump(rx: Receiver<ProxyEvents>) {
    loop {
        let event = rx.try_recv();
        if let Err(_) = event {
            continue;
        }

        match event.unwrap() {
            ProxyEvents::RequestSent((wrapper, hash)) => {
                let first_line = format!("{} {} {}", &wrapper.method, &wrapper.uri, &wrapper.version);
                println!("http {:x} ==> {}", hash, first_line);
            },
            ProxyEvents::ResponseSent((wrapper, hash)) => {
                let first_line = format!("{} {}", &wrapper.version, &wrapper.status);
                println!("http {:x} <== {}", hash, first_line);
            },
            ProxyEvents::WebSocketMessageSent((_ctx, _msg)) => {
                match _ctx {
                    WebSocketContext::ClientToServer { src, dst, .. } => {
                        println!("wskt {} ==> {} -- {}...", src, dst, &_msg.into_data().to_str_lossy()[..30]);
                    },
                    WebSocketContext::ServerToClient { src, dst, .. } => {
                        println!("wskt {} ==> {} -- {}...", src, dst, &_msg.into_data().to_str_lossy()[..30]);
                    }
                }
            }
        }
    }
}