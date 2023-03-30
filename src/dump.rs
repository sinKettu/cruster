use crate::{cruster_proxy::{events::{ProxyEvents}, request_response::{HyperRequestWrapper, HyperResponseWrapper}}, config::Config};
use crossbeam_channel::Receiver;
use hudsucker::WebSocketContext;
use bstr::ByteSlice;
use std::borrow::Cow;

pub(crate) trait DumpMode {
    fn dump_mode_enabled(&self) -> bool;
    fn get_verbosity(&self) -> u8;
}

impl DumpMode for Config {
    fn dump_mode_enabled(&self) -> bool {
        return if let Some(dm) = self.dump_mode.as_ref() {
            dm.enabled
        }
        else {
            false
        }
    }

    fn get_verbosity(&self) -> u8 {
        return self.dump_mode
            .as_ref()
            .unwrap()
            .verbosity;
    }
}

fn print_request(wrapper: HyperRequestWrapper, hash: usize, config: &super::config::Config) {
    let first_line = format!("{} {} {}", &wrapper.method, &wrapper.uri, &wrapper.version);
    println!("http {:x} ==> {}", hash, first_line);

    if config.get_verbosity() >= 2 {
        let mut headers = String::default();
        let mut keys_list: Vec<&str> = wrapper.headers
            .keys()
            .into_iter()
            .map(|k| {
                k.as_str()
            })
            .collect();

        keys_list.sort();
        for key in keys_list {
            let v_iter = wrapper.headers
                .get_all(key)
                .iter()
                .map(|val| {
                    val.as_bytes().to_str_lossy()
                })
                .collect::<Vec<Cow<str>>>()
                .join("; ");

            headers = format!(
                "{}http {:x} ==> {}: {}\r\n",
                headers,
                hash,
                key,
                v_iter
            );
        }

        print!("{}", headers);
        println!("http {:x} ==>", hash);
    }

    if config.get_verbosity() != 0 {
        println!("");
    }
}

fn print_response(wrapper: HyperResponseWrapper, hash: usize, config: &super::config::Config) {
    let first_line = format!("{} {}", &wrapper.version, &wrapper.status);
    println!("http {:x} <== {}", hash, first_line);

    if config.get_verbosity() >= 1 {
        let mut headers = String::default();
        let mut keys_list: Vec<&str> = wrapper.headers
            .keys()
            .into_iter()
            .map(|k| {
                k.as_str()
            })
            .collect();

        keys_list.sort();
        for key in keys_list {
            let v_iter = wrapper.headers
                .get_all(key)
                .iter()
                .map(|val| {
                    val.as_bytes().to_str_lossy()
                })
                .collect::<Vec<Cow<str>>>()
                .join("; ");

            headers = format!(
                "{}http {:x} ==> {}: {}\r\n",
                headers,
                hash,
                key,
                v_iter
            );
        }

        print!("{}", headers);
        println!("http {:x} ==>", hash);
    }

    if config.get_verbosity() != 0 {
        println!("");
    }
}

pub(super) async fn launch_dump(rx: Receiver<ProxyEvents>, config: super::config::Config) {
    loop {
        let event = rx.try_recv();
        if let Err(_) = event {
            continue;
        }

        match event.unwrap() {
            ProxyEvents::RequestSent((wrapper, hash)) => {
                print_request(wrapper, hash, &config);
            },
            ProxyEvents::ResponseSent((wrapper, hash)) => {
                print_response(wrapper, hash, &config);
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