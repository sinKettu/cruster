use bstr::ByteSlice;
use std::borrow::Cow;
use std::time;
use colored::Colorize;
use crossbeam_channel::Receiver;
use hudsucker::WebSocketContext;

use crate::{
    cruster_proxy::{
        events::ProxyEvents,
        request_response::{
            HyperRequestWrapper,
            HyperResponseWrapper
        }
    },
    config::Config, http_storage::HTTPStorage, utils::CrusterError
};

pub(crate) trait DumpMode {
    fn dump_mode_enabled(&self) -> bool;
    fn get_verbosity(&self) -> u8;
    fn with_color(&self) -> bool;
}

impl DumpMode for Config {
    fn dump_mode_enabled(&self) -> bool {
        return if let Some(dm) = self.dump_mode.as_ref() {
            dm.enabled
        }
        else {
            false
        };
    }

    fn get_verbosity(&self) -> u8 {
        return self.dump_mode
            .as_ref()
            .unwrap()
            .verbosity;
    }

    fn with_color(&self) -> bool {
        return if let Some(dm) = self.dump_mode.as_ref() {
            dm.color
        }
        else {
            false
        };
    }   
}

fn print_request(wrapper: &HyperRequestWrapper, hash: usize, config: &super::config::Config) {
    let verbosity = config.get_verbosity();
    let first_line = format!("{} {} {}", &wrapper.method, &wrapper.uri, &wrapper.version);

    let prefix = if config.with_color() {
        let hash = hash.to_string().bright_black();
        let direction = format!("{}{}", "--".green(), ">".bright_green());
        
        format!("{} {:>6} {}", "http".yellow(), hash, direction)
    }
    else {
        let hash = hash.to_string();
        format!("http {} -->", hash)
    };

    println!("{} {}", &prefix, first_line);

    if verbosity >= 2 {
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
                "{}{} {}: {}\r\n",
                headers,
                &prefix,
                key,
                v_iter
            );
        }

        print!("{}", headers);
        println!("{}", &prefix);
    }

    if verbosity >= 4 {
        let body = wrapper.body.to_str_lossy();
        println!("{} {}", &prefix, body);
    }

    if config.get_verbosity() != 0 {
        println!("");
    }
}

fn print_response(wrapper: &HyperResponseWrapper, hash: usize, config: &super::config::Config) {
    let verbosity = config.get_verbosity();
    let first_line = format!("{} {}", &wrapper.version, &wrapper.status);

    let prefix = if config.with_color() {
        let hash = hash.to_string().bright_black();
        let direction = format!("{}{}", "<".bright_green(), "==".green());

        format!("{} {:>6} {}", "http".yellow(), hash, direction)
    }
    else {
        let hash = hash.to_string();
        format!("{} {} {}", "http", hash, "<==")
    };

    println!("{} {}", &prefix, first_line);

    if verbosity >= 1 {
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
                "{}{} {}: {}\r\n",
                headers,
                &prefix,
                key,
                v_iter
            );
        }

        print!("{}", headers);
        println!("{}", &prefix);
    }

    if verbosity >= 3 {
        let body = wrapper.body.to_str_lossy();
        println!("{} {}", &prefix, body);
    }

    if config.get_verbosity() != 0 {
        println!("");
    }
}

fn print_ws_message(msg: &[u8], ctx: &WebSocketContext, config: &super::config::Config) {
    match ctx {
        WebSocketContext::ClientToServer { src, dst, .. } => {
            let printable_mes = msg.to_str_lossy();
            let verbosity = config.get_verbosity();
            
            let prefix = if config.with_color() {
                let src = src.to_string().bright_black();
                let dst = dst.to_string().bright_black();
                let direction = format!("{}{}", "--".green(), ">".bright_green());

                format!("{} {} {} {}", "wskt".purple(), src, direction, dst)
            }
            else {
                format!("wskt {} --> {}", src, dst)
            };

            if verbosity >= 3 {
                println!("{} {}", &prefix, printable_mes);
            }
            else {
                let limit = if printable_mes.len() < 30 { printable_mes.len() } else { 30 };
                println!("{} {}...", &prefix, &printable_mes[.. limit]);
            }
        },
        WebSocketContext::ServerToClient { src, dst, .. } => {
            let printable_mes = msg.to_str_lossy();
            let verbosity = config.get_verbosity();
            
            let prefix = if config.with_color() {
                let src = src.to_string().bright_black();
                let dst = dst.to_string().bright_black();
                let direction = format!("{}{}", "<".bright_green(), "==".green());

                format!("{} {} {} {}", "wskt".purple(), dst, direction, src)
            }
            else {
                format!("wskt {} <== {}", dst, src)
            };

            if verbosity >= 3 {
                println!("{} {}", &prefix, printable_mes);
            }
            else {
                let limit = if printable_mes.len() < 30 { printable_mes.len() } else { 30 };
                println!("{} {}...", &prefix, &printable_mes[.. limit]);
            }
        }
    }
}

fn print_error(err: CrusterError, need_color: bool) {
    if need_color {
        eprintln!("{} {}", "errr".red(), err);
    }
    else {
        eprintln!("{} {}", "errr", err);
    }
}

pub(super) async fn launch_dump(rx: Receiver<ProxyEvents>, config: super::config::Config) {
    let mut http_storage = HTTPStorage::default();
    if let Some(proj_path) = config.project.as_ref() {
        let path = format!("{}/http.jsonl", proj_path);

        // Do it to set apropriate next_id in HTTPStorage state
        http_storage.load(&path).unwrap();
        http_storage.clear().unwrap();

        http_storage.keep_open(&path).unwrap();
    }
    else {
        print_error(
            CrusterError::UndefinedError("No storage defined, traffic will not be saved!".to_string()),
            config.with_color()
        )
    }

    // Remove uncompleted requests older than 5 minutes everu 10 minutes
    let mut time_mark = time::SystemTime::now();
    let cycle_duration = time::Duration::new(600, 0);

    loop {
        let event = rx.try_recv();
        if let Err(_) = event {
            continue;
        }

        match event.unwrap() {
            ProxyEvents::RequestSent((wrapper, hash)) => {
                let _ = http_storage.put_request(wrapper, hash);
            },
            ProxyEvents::ResponseSent((wrapper, hash)) => {
                let id = http_storage.put_response(wrapper, &hash);
                if let Some(id) = id {
                    let pair = http_storage.get_by_id(id).unwrap();
                    print_request(pair.request.as_ref().unwrap(), id, &config);
                    print_response(pair.response.as_ref().unwrap(), id, &config);

                    if let Err(err) = http_storage.flush_by_id(id) {
                        print_error(err, config.with_color());
                    }
                    else {
                        if let Err(err) = http_storage.remove_by_id(id) {
                            print_error(err, config.with_color());
                        }
                    }
                }
            },
            ProxyEvents::WebSocketMessageSent((_ctx, _msg)) => {
                let m = _msg.into_data();
                print_ws_message(m.as_slice(), &_ctx, &config);
            },
            ProxyEvents::Error((err, hash)) => {
                print_error(err, config.with_color());
                
                if let Some(hash) = hash {
                    if let Err(err) = http_storage.remove_uncompleted(hash) {
                        print_error(err, config.with_color());
                    }
                }
            }
        }

        let now = time::SystemTime::now();
        if now.duration_since(time_mark).unwrap() > cycle_duration {
            time_mark = now;
            if let Err(err) = http_storage.remove_uncompleted_older_than(time::Duration::new(300, 0)) {
                print_error(err, config.with_color());
            }
        }
    }
}