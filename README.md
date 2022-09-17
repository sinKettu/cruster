# Cruster

> Current working branch is `ui_improvements`

`v0.3.1`

Intercepting HTTP(S)/WS(S) proxy for penetration tests' and DevSecOps purposes.
Inspired by `Burp Suite`, `OWASP ZAP`, `Mitmproxy` and `Nuclei`. Hope it could be as useful as them.

## Usage

Just run `cruster` and it will create working directory in `~/.cruster`, putting there _base config_, _TLS certificate_ with _key_. Then it will be listening to requests on address `127.0.0.1:8080`.

To use this proxy with browser you must import CA certificate of proxy (stored by default in `~/.cruster/cruster.cer`) into browser manually.

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

### Help output:

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

## RoadMap

- [X] Improve proxy performance.
- [ ] Navigate over Requests/Responses text.
- [ ] Requests/Responses syntax highlight.
- [ ] Manual repeater for requests.
- [ ] Projects (like in Burp or ZAP).
- [ ] Store projects and history on drive.
- [ ] **Scripting engine based on YAML syntax to write testcases and checks**.
- [ ] **Scripting engine based on Python to write testcases and checks**
- [X] WS(S) support.
- [ ] Improve documentation.
- [ ] WS(S) proxy history visualisation (like for HTTP(S))
- [ ] And much more ...

## License

Licensed with MIT license.

Copyright 2022 Andrey Ivanov.
