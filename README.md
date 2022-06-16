# Cruster

`v0.2.3`

Intercepting HTTP(S)/WS(S) proxy for pentration tests' and DevSecOps purposes.

Inspired by `Burp Suite`, `OWASP ZAP` and `Nuclei`. Hope it could be as useful as them.

## Usage

Just run `cruster` and it will create working directory in `~/.cruster`, putting there base config, TLS certificate with key. Then it will be listening to requests on address `127.0.0.1:8080`.

Run

``` shell
$ cruster -h
```

to see more info about usages.

## RoadMap

- [ ] Improve proxy performance.
- [ ] Navigate over Requests/Responses text.
- [ ] Requests/Responses syntax highlight.
- [ ] Manual repeater for requests.
- [ ] Projects (like in Burp or ZAP).
- [ ] Store projects and history on drive.
- [ ] **Scripting engine based on YAML syntax to write testcases in checks**.
- [ ] WS(S) support.
- [ ] Improve documentation.
- [ ] And much more ...

## License

Licensed with MIT license.

Copyright 2022 Andrey Ivanov.
