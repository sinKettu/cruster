# Cruster

`v0.5.0`

Intercepting HTTP(S)/WS(S) proxy for penetration tests' and DevSecOps purposes.
Inspired by `Burp Suite`, `OWASP ZAP`, `Mitmproxy` and `Nuclei`. Hope it could be as useful as them.

![cruster](https://github.com/sinKettu/cruster/raw/master/static/cruster-main.png)

## What Cruster can do

- Proxy HTTP;
- Proxy WebSocket;
- Interactive text interface:
  - Table visualization of HTTP messgages went through proxy;
  - Requests/Responses highlighted visualization;
  - Filtering content;
  - Manual requests repeater;
- Dump mode (`-d`);
- Process requests/responses basing on scope (`-I`, `-E`);
- ... *Coming soon*...

You can find more detailed description in [Usage.md](https://github.com/sinKettu/cruster/blob/master/docs/Usage.md)

## Usage

Just run `cruster` and it will create working directory in `~/.cruster`, putting there *base config*, *TLS certificate* with *key*. Then it will be listening to requests on address `127.0.0.1:8080`.

To use this proxy with browser you must import CA certificate of proxy (stored by default in `~/.cruster/cruster.cer`) into browser manually.

### Help output

``` shell
$ cruster -h
Cruster 0.5.0
Andrey Ivanov<avangard.jazz@gmail.com>

USAGE:
    cruster [FLAGS] [OPTIONS]

FLAGS:
    -d, --dump       Enable non-interactive dumping mode: all communications will be shown in terminal output
    -h, --help       Prints help information
        --strict     If set, none of out-of-scope data will be written in storage, otherwise it will be just hidden from
                     ui
    -V, --version    Prints version information

OPTIONS:
    -a, --address <ADDR>                Address for proxy to bind, default: 127.0.0.1
    -c, --config <YAML_CONFIG>          Path to config with YAML format. Cannot be set by config file.
        --debug-file <FILE-TO-WRITE>    A file to write debug messages, mostly needed for development
    -E, --exclude-scope <REGEX>...      Regex for URI to exclude from scope, i.e. ^https?://www\.google\.com/.*$.
                                        Processed after include regex if any. Option can repeat.
    -I, --include-scope <REGEX>...      Regex for URI to include in scope, i.e. ^https?://www\.google\.com/.*$. Option
                                        can repeat.
    -l, --load <PATH-TO-FILE>           Path to file to load previously stored data from
    -p, --port <PORT>                   Port for proxy to listen to, default: 8080
    -s, --store <PATH-TO-FILE>          Path to file to store proxy data. File will be rewritten!
    -P, --workplace <WORKPLACE_DIR>     Path to workplace, where data (configs, certs, projects, etc.) will be stored.
                                        Cannot be set by config file.
```

### Navigation on text user interface

``` text
? - Show this help view
<Enter> -
    <On Proxy Table> - Show interactive fullscreen view for selected request and response contents
    <On Filter View> - Apply written filter
    <On Repeater View> - Apply edited request / Send
<Esc> - Close secondary view (i.e. help, errors, etc.)
<Shift> + r - Repeat request selected on table
<Shift> + s - Store proxy data on drive, file path is configured on start
<Shift> + f - Set filter for table
e - Show error logs view
i -
    <On Repeater View> - Edit request
p -
    <On Repeater View> - Show parameters
r - Show active repeaters
t - Show fullscreen HTTP proxy table
q - Quit

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

## Features and Compilation

Cruster contains the following features (in terms of Rust):

- `rcgen-ca` - use `Rcgen` to build local CA;
- `crosstrem` - use `Crossterm` as Text User Interface backend (cross-platform);
- `default` - includes previous two features, enabled by default;
- `openssl-ca` - use `OpenSSL` to build local CA; requires `OpenSSL` (`libssl`) to be installed;
- `ncurses` - use `Ncurses` as Text User Interface backend; requires `Ncurses` (`libncurses`/`libncurses5`/`libncursesw5`) to be installed;
- `termion` - use `Termion` as Text User Interface backend;

All features can be devided in two groups:

- *CA backend*:
  - `rcgen-ca`
  - `openssl-ca`
- *TUI backend*:
  - `crossterm`
  - `ncurses`
  - `termion`

To successfully compile `Cruster` one feature from each group must be defined (`default` feature do it by default).

## Installation

The only option for now is to install from source code with `git` and `cargo`. You can use the following command:

### Fully Rust-Based Installation

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.5.0"
```

This command will install `Cruster` using `rcgen` library to build local certificate authority and `crossterm` as TUI backend. So, you are going to get full-rust package.

> In some case `crossterm` and `termion` backends can flicker. It is a known `cursive` [issue](https://github.com/gyscos/cursive/issues/667). For Cruster [buffered backend](https://github.com/agavrilov/cursive_buffered_backend) is implemented, but it is not for sure, that buffering will cover all cases. If you faced with such problem, you can use `ncurses` backend.

If, for some reason, you do not want to use `rcgen` to handle certificates, you can use openssl, see below.

### Using OpenSSL for Local CA

You can install `Cruster` and use `OpenSSL` to handle certificates. **In this case, you have to had `OpenSSL` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.5.0" --no-default-features --features openssl-ca,crossterm
```

### Using Ncurses as TUI Backend

`Ncurses` can be used as TUI backend instead of `Crossterm` (which is fully rust-written). **In this case, you have to had `Ncurses` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.5.0" --no-default-features --features ncurses,rcgen-ca
```

## With Docker

Instead of usual installation you can use Cruster from a docker container. You can build your own:

``` shell
$ cd cruster && sudo docker build . -f docker/Dockerfile -t local/cruster
```

Also you can build a version with use of `openssl` and `ncurses`:

``` shell
$ cd cruster && sudo docker build . -f docker/Dockerfile-openssl-ncurses -t local/cruster
```

Or you can use ready image:

``` shell
$ sudo docker pull sinfox/cruster:latest
$ sudo docker run -it sinfox/cruster
```

## RoadMap

- [X] Improve proxy performance.
- [X] Navigate over Requests/Responses text.
- [X] Requests/Responses syntax highlight.
- [ ] Intercepting requests/responses.
- [X] Manual repeater for requests.
- [X] Projects (like in Burp or ZAP), *this thing will be developing with further improvements of Cruster*.
- [X] Store projects and history on drive.
- [ ] **Scripting engine based on YAML syntax to write testcases and checks**.
- [ ] **Scripting engine based on Python to write testcases and checks**.
- [X] WS(S) support.
- [ ] Improve documentation.
- [ ] WS(S) proxy history visualization (like for HTTP(S)).
- [ ] Reverse proxy mode.
- [ ] And much more ...

## Gratitude

Thank to projects, which are basics for mine:

- [Hudsucker](https://github.com/omjadas/hudsucker) - Library to build MitM proxy;
- [Cursive](https://github.com/gyscos/cursive) - Library to build text (console) user interface.

## License

Copyright © Andrey Ivanov
