# Cruster

`v0.7.2`

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
- CLI, which is comparable with TUI;
- Process requests/responses basing on scope (`-I`, `-E`);
- Storing/Loading proxy data on/from drive;
- ... *Coming soon*...

## Usage

There are three ways you can use Cruster: with *interactive text interface*, in *dump mode* (logging) and as *CLI* tool.

To start, just run `cruster` and it will create working directory in `~/.cruster`, putting there *base config*, *TLS certificate* with *key*. Then it will be listening to requests on address `127.0.0.1:8080`.

To use this proxy with browser you must import CA certificate of proxy (stored by default in `~/.cruster/cruster.cer`) into browser manually.

### Help output

``` shell
$ cruster help
Usage: cruster [OPTIONS] [COMMAND]

Commands:
  interactive  Default interactive Cruster mode. This mode will be used if none is specified
  dump         Enable non-interactive dumping mode: all communications will be shown in terminal output
  cli          Cruster Command Line Interface
  help         Print this message or the help of the given subcommand(s)

Options:
  -W, --workplace <WORKPLACE_DIR>    Path to workplace, where data (configs, certs, projects, etc.) will be stored. Cannot be set by config file.
  -c, --config <YAML_CONFIG>         Path to config with YAML format. Cannot be set by config file.
  -a, --address <ADDR>               Address for proxy to bind, default: 127.0.0.1
  -p, --port <PORT>                  Port for proxy to listen to, default: 8080
      --debug-file <FILE-TO-WRITE>   A file to write debug messages, mostly needed for development
  -P, --project <PATH-TO-DIR>        Path to directory to store/load Cruster state. All files could be rewritten!
      --strict                       If set, none of out-of-scope data will be written in storage, otherwise it will be just hidden from ui
  -I, --include-scope <REGEX>        Regex for URI to include in scope, i.e. ^https?://www\.google\.com/.*$. Option can repeat.
  -E, --exclude-scope <REGEX>        Regex for URI to exclude from scope, i.e. ^https?://www\.google\.com/.*$. Processed after include regex if any. Option can repeat.
      --editor <PATH_TO_EXECUTABLE>  Path to editor executable to use in CLI mode
  -h, --help                         Print help
  -V, --version                      Print version
```

Cruster has several commands (`interactive`, `dump`, etc.) and options. Options used after executable name (i.e. `cruster -p 8082`) are global and also can be managed with config. Options used after commands are command-specific. You always can call `help` or `-h` to learn details.

### Text User Interface

You can find more details at [TUI.md](https://github.com/sinKettu/cruster/blob/master/docs/TUI.md)

To run TUI use

``` shell
cruster
```

or

``` shell
cruster interactive
```

Interactive mode is fully controlled by global Cruster cmd-options or config.

You will be provided with interactive interface inside your terminal, which you can control with keyboard (and mouse, sometimes).

> This type of interface will be developing longer than others, since it requires more efforts.

#### Navigation on text user interface

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

To run dump mode, use

```shell
cruster dump
```
Example:

``` shell
$ ./cruster -p 8082 dump -v
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
...

```

Dump mode has several command-specific options:

```shell
$ cruster dump -h
Enable non-interactive dumping mode: all communications will be shown in terminal output

Usage: cruster dump [OPTIONS]

Options:
  -v...       Verbosity in dump mode, ignored in intercative mode. 0: request/response first line,
              1: 0 + response headers, 2: 1 + request headers, 3: 2 + response body, 4: 3 + request body
      --nc    Disable colorizing in dump mode, ignored in interactive mode
  -h, --help  Print help
```

### CLI

You can find more details at [CLI.md](https://github.com/sinKettu/cruster/blob/master/docs/CLI.md)

CLI works with data, stored in dump or interactive modes, so it is required to provide a path to project to use CLI (via cmd argument or config).

CLI is good addition to dump mode, it allows to manage all data collected with proxy in command-by-command way. To run CLI, use

```shell
cruster cli
```

For example, it can simply print dumped HTTP traffic:

```shell
$ cruster cli http show 2
    ID   METHOD                         HOSTNAME                                                                   PATH      STATUS          LENGTH

     0      GET                       google.com /                                                                              301             219
     1      GET                   www.google.com /                                                                              200           19957
```

> Probably, in future all new features will be developing for CLI firstly and then for TUI

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
cargo install --git https://github.com/sinKettu/cruster --tag "v0.7.2"
```

This command will install `Cruster` using `rcgen` library to build local certificate authority and `crossterm` as TUI backend. So, you are going to get full-rust package.

> In some case `crossterm` and `termion` backends can flicker. It is a known `cursive` [issue](https://github.com/gyscos/cursive/issues/667). For Cruster the [buffered backend](https://github.com/agavrilov/cursive_buffered_backend) is implemented, but it is not for sure, that buffering will cover all cases. If you faced with such problem, you can use `ncurses` backend.

If, for some reason, you do not want to use `rcgen` to handle certificates, you can use openssl, see below.

### Using OpenSSL for Local CA

You can install `Cruster` and use `OpenSSL` to handle certificates. **In this case, you have to had `OpenSSL` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.7.2" --no-default-features --features openssl-ca,crossterm
```

### Using Ncurses as TUI Backend

`Ncurses` can be used as TUI backend instead of `Crossterm` (which is fully rust-written). **In this case, you have to had `Ncurses` installed on your computer.**

``` shell
cargo install --git https://github.com/sinKettu/cruster --tag "v0.7.2" --no-default-features --features ncurses,rcgen-ca
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
