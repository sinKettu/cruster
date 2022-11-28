# Cruster

`v0.4.3`

Intercepting HTTP(S)/WS(S) proxy for penetration tests' and DevSecOps purposes.
Inspired by `Burp Suite`, `OWASP ZAP`, `Mitmproxy` and `Nuclei`. Hope it could be as useful as them.

![cruster](https://raw.githubusercontent.com/sinKettu/static/cruster-main.png)

## What Cruster can do

- Proxy HTTP;
- Proxy WebSocket;
- Interactive text interface:
  - Table visualization of HTTP messgages went through proxy;
  - Requests/Responses highlighted visualization;
- Dump mode.
- ... *Coming soon*...

## Usage

Just run `cruster` and it will create working directory in `~/.cruster`, putting there _base config_, _TLS certificate_ with _key_. Then it will be listening to requests on address `127.0.0.1:8080`.

To use this proxy with browser you must import CA certificate of proxy (stored by default in `~/.cruster/cruster.cer`) into browser manually.

### Help output

``` shell
$ cruster -h
Cruster 0.3.1
Andrey Ivanov<avangard.jazz@gmail.com>

USAGE:
    cruster [FLAGS] [OPTIONS]

FLAGS:
    -d, --dump       Enable non-interactive dumping mode: all communications will be shown in terminal output
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --address <ADDR>                Address for proxy to bind, default: 127.0.0.1
    -c, --config <YAML_CONFIG>          Path to config with YAML format [default: ~/.cruster/config.yaml]
        --debug-file <FILE-TO-WRITE>    A file to write debug messages, mostly needed for development
    -p, --port <PORT>                   Port for proxy to listen to, default: 8080
    -P, --workplace <WORKPLACE_DIR>     Path to workplace, where data (configs, certs, projects, etc.) will be stored
                                        [default: ~/.cruster/]
```

### Navigation on text user interface

``` text
? - Show help view
q - Quit
e - Show error logs view
t - Show fullscreen HTTP proxy table
<Enter>
│     <On Proxy Table> - Show interactive fullscreen view for selected request and response contents
<Esc> - Close secondary view (i.e. help, errors, etc.)
```

### Dump mode

By default, `Cruster` will run in interactive mode, drawing interface in you terminal. You can also run it in `dump-mode` 
using option `--dump`/`-d` and it will be just logging traffic:

``` shell
$ cruster --dump
http ==> GET http://www.google.com/
http ==> host: www.google.com
http ==> user-agent: curl/7.83.1
http ==> accept: */*
http ==> proxy-connection: Keep-Alive
http ==>
http ==>
http <== 200
http <== date: Sun, 03 Jul 2022 12:17:36 GMT
http <== expires: -1
http <== cache-control: private, max-age=0
http <== content-type: text/html; charset=ISO-8859-1
http <== p3p: CP="This is not a P3P policy! See g.co/p3phelp for more info."
http <== server: gws
http <== x-xss-protection: 0
http <== x-frame-options: SAMEORIGIN
http <== accept-ranges: none
http <== vary: Accept-Encoding
http <== transfer-encoding: chunked
http <==
http <== <!doctype html><html i
http <==
```

## Installation

The only option for now is to install from source code with `git` and `cargo`. You can use the following command:

### Fully Rust-Based Installation

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.4.3" --locked
```

This command will install `Cruster` using `rcgen` library to build local certificate authority and `crossterm` as TUI backend. So, you are going to get full-rust package.

> There are a problem with using `rcgen`, because of which local CA can wrongly sign site's certificates and browsers will be refusing them (problem is not in `rcgen` library): https://github.com/omjadas/hudsucker/issues/39. There is a way to avoid this problem, while it would not be solved, see below.

### Using OpenSSL for Local CA

You can install `Cruster` and use `OpenSSL` to handle certificates. **In this case, you have to had `OpenSSL` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.4.3" --locked --no-default-features --features openssl-ca,crossterm
```

### Using Ncurses as TUI Backend

`Ncurses` can be used as TUI backend instead of `Crossterm` (which is fully rust-written). **In this case, you have to had `Ncurses` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.4.3" --locked --no-default-features --features ncurses,rcgen-ca
```

## RoadMap

- [X] Improve proxy performance.
- [X] Navigate over Requests/Responses text.
- [X] Requests/Responses syntax highlight.
- [ ] Manual repeater for requests.
- [ ] Projects (like in Burp or ZAP).
- [ ] Store projects and history on drive.
- [ ] **Scripting engine based on YAML syntax to write testcases and checks**.
- [ ] **Scripting engine based on Python to write testcases and checks**
- [X] WS(S) support.
- [ ] Improve documentation.
- [ ] WS(S) proxy history visualisation (like for HTTP(S))
- [ ] And much more ...

## Gratitude

Thank to projects, which are basics for mine:

- [Hudsucker](https://github.com/omjadas/hudsucker) - Library to build MitM proxy;
- [Cursive](https://github.com/gyscos/cursive) - Library to build text (console) user interface.

## License

Licensed with GNU GENERAL PUBLIC LICENSE Version 3.

Copyright 2022 Andrey Ivanov.
