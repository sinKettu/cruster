# Cruster

`v0.7.0`

Intercepting HTTP(S)/WS(S) proxy for penetration tests' and DevSecOps purposes.
Inspired by `Burp Suite`, `OWASP ZAP`, `Mitmproxy` and `Nuclei`. Hope it could be as useful as them.

> Cruster is highly under development for now. Unfortuantely, I have not got enough free time to develop it faster.

![cruster](https://github.com/sinKettu/cruster/raw/master/static/cruster-main.png)

## What Cruster can do

- Proxy HTTP;
- Proxy WebSocket;
- Interactive text interface:
  - Table visualization of HTTP messgages went through proxy;
  - Requests/Responses highlighted visualization;
  - Filtering content;
  - Manual requests repeater;
- Dump mode (`-d`) with controlable verbosity;
- Process requests/responses basing on scope (`-I`, `-E`);
- Storing/Loading proxy data on/from drive;
- ... *Coming soon*...

You can find more detailed description in [Usage.md](https://github.com/sinKettu/cruster/blob/master/docs/Usage.md)

## Usage

Just run `cruster` and it will create working directory in `~/.cruster`, putting there *base config*, *TLS certificate* with *key*. Then it will be listening to requests on address `127.0.0.1:8080`.

To use this proxy with browser you must import CA certificate of proxy (stored by default in `~/.cruster/cruster.cer`) into browser manually.

### Help output

``` shell
$ cruster -h
Cruster 0.7.0
Andrey Ivanov<avangard.jazz@gmail.com>

USAGE:
    cruster [FLAGS] [OPTIONS]

FLAGS:
    -d, --dump       Enable non-interactive dumping mode: all communications will be shown in terminal output
    -h, --help       Prints help information
        --nc         Disable colorizing in dump mode, ignored in interactive mode
        --strict     If set, none of out-of-scope data will be written in storage, otherwise it will be just hidden from
                     ui
    -V, --version    Prints version information
    -v               Verbosity in dump mode, ignored in intercative mode. 0: request/response first line,
                     1: 0 + response headers, 2: 1 + request headers, 3: 2 + response body, 4: 3 + request body

OPTIONS:
    -a, --address <ADDR>                Address for proxy to bind, default: 127.0.0.1
    -c, --config <YAML_CONFIG>          Path to config with YAML format. Cannot be set by config file.
        --debug-file <FILE-TO-WRITE>    A file to write debug messages, mostly needed for development
    -E, --exclude-scope <REGEX>...      Regex for URI to exclude from scope, i.e. ^https?://www\.google\.com/.*$.
                                        Processed after include regex if any. Option can repeat.
    -I, --include-scope <REGEX>...      Regex for URI to include in scope, i.e. ^https?://www\.google\.com/.*$. Option
                                        can repeat.
    -p, --port <PORT>                   Port for proxy to listen to, default: 8080
    -P, --project <PATH-TO-DIR>         Path to directory to store/load Cruster state. All files could be rewritten!
    -W, --workplace <WORKPLACE_DIR>     Path to workplace, where data (configs, certs, projects, etc.) will be stored.
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
f - 
    <On FullScreen Request/Response> - Copy request and response content to clipboard
i - 
    <On Repeater View> - Edit request
p - 
    <On Repeater View> - Show parameters
r - 
    <On Proxy Table> - Show active repeaters
    <On FullScreen Request/Response> - Copy request content to clipboard
s - 
    <On FullScreen Request/Response> - Copy response content to clipboard
t - Show fullscreen HTTP proxy table
q - Quit

```

### Dump mode

By default, `Cruster` will run in interactive mode, drawing interface in you terminal. You can also run it in `dump-mode` 
using option `--dump`/`-d` and it will be just logging traffic:

``` shell
$ ./cruster -p 8082 -d -v
errr No storage defined, traffic will not be saved!
http      0 --> GET http://google.com/ HTTP/1.1

http      0 <== HTTP/1.1 301 Moved Permanently
http      0 <== cache-control: public, max-age=2592000
http      0 <== content-length: 219
http      0 <== content-security-policy-report-only: object-src 'none';base-uri 'self';script-src 'nonce-9zrh7P5SjSprYVnylsm-xg' 'strict-dynamic' 'report-sample' 'unsafe-eval' 'unsafe-inline' https: http:;report-uri https://csp.withgoogle.com/csp/gws/other-hp
http      0 <== content-type: text/html; charset=UTF-8
http      0 <== date: Sat, 27 May 2023 13:05:45 GMT
http      0 <== expires: Mon, 26 Jun 2023 13:05:45 GMT
http      0 <== location: http://www.google.com/
http      0 <== server: gws
http      0 <== x-frame-options: SAMEORIGIN
http      0 <== x-xss-protection: 0
http      0 <==

http      1 --> GET http://www.google.com/ HTTP/1.1

http      1 <== HTTP/1.1 200 OK
http      1 <== accept-ranges: none
http      1 <== cache-control: private, max-age=0
http      1 <== content-security-policy-report-only: object-src 'none';base-uri 'self';script-src 'nonce-pikCUd1GEkT5BQk0K5Ru2g' 'strict-dynamic' 'report-sample' 'unsafe-eval' 'unsafe-inline' https: http:;report-uri https://csp.withgoogle.com/csp/gws/other-hp
http      1 <== content-type: text/html; charset=ISO-8859-1
http      1 <== date: Sat, 27 May 2023 13:05:45 GMT
http      1 <== expires: -1
http      1 <== p3p: CP="This is not a P3P policy! See g.co/p3phelp for more info."
http      1 <== server: gws
http      1 <== set-cookie: NID=511=Vl-ShZeMOBsqbJzLjL_cM81bgPNYfAWQZSERvIFbYs6X70yGsDJ4e9Kuh_AM_y3mb6Ya0N34mCj18z-2qhBQnVrrviSMrMsEKChv3SAqIf6vwYE20PlherbpuU2ZGp2x8edD4WxWJQ0GoqlRS8K45aRtD6Ri9EZSwSTEUqTsRR4; expires=Sun, 26-Nov-2023 13:05:45 GMT; path=/; domain=.google.com; HttpOnly
http      1 <== transfer-encoding: chunked
http      1 <== vary: Accept-Encoding
http      1 <== x-frame-options: SAMEORIGIN
http      1 <== x-xss-protection: 0
http      1 <==

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
cargo install --git https://github.com/sinKettu/cruster --tag "v0.7.0"
```

This command will install `Cruster` using `rcgen` library to build local certificate authority and `crossterm` as TUI backend. So, you are going to get full-rust package.

> In some case `crossterm` and `termion` backends can flicker. It is a known `cursive` [issue](https://github.com/gyscos/cursive/issues/667). For Cruster the [buffered backend](https://github.com/agavrilov/cursive_buffered_backend) is implemented, but it is not for sure, that buffering will cover all cases. If you faced with such problem, you can use `ncurses` backend.

If, for some reason, you do not want to use `rcgen` to handle certificates, you can use openssl, see below.

### Using OpenSSL for Local CA

You can install `Cruster` and use `OpenSSL` to handle certificates. **In this case, you have to had `OpenSSL` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.7.0" --no-default-features --features openssl-ca,crossterm
```

### Using Ncurses as TUI Backend

`Ncurses` can be used as TUI backend instead of `Crossterm` (which is fully rust-written). **In this case, you have to had `Ncurses` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.7.0" --no-default-features --features ncurses,rcgen-ca
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
