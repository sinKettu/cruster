# Cruster CLI Usage

Cruster CLI interface is similar to such programs like `Docker`, `Kubectl`, `Git`, etc.

## Basic commands

CLI has the following basic commands:

- `http` - to work with dumped history of HTTP requests/responses.
- `repeater` - to observe and replay requests (*with editing!*).
- `help` - to get commands' description.

## http

`http` command has the following subcommands:

- `show` - list all the history of HTTP traffic with a bunch of options.
Available arguments and options:

    ```shell
    $ cruster cli http show -h
    Filter/Sort/Find HTTP data and print it

    Usage: cruster cli http show [OPTIONS] <INDEX>

    Arguments:
    <INDEX>  range of line numbers or exact line number in file with stored HTTP data to print: n -- first n pairs, n-m -- pairs from n to m, a -- all stored pairs, n$ -- only Nth pair

    Options:
    -u, --urls                 Print only indexes and full URLs
    -p, --pretty               Print full formated requests and responses (if any)
    -r, --raw                  Print raw data as it was dumped in project
    -f, --filter <filter>      Filter pairs in specifyied bounds with regular expression in format of 're2'
    -e, --extract <ATTRIBUTE>  Extract pairs from range by attribute. parameter syntax: method=<name>|status=<value>|host=<prefix>|path=<prefix>
    -i, --index <NUMBER>       Get pair with specific ID
    -h, --help                 Print help
    ```

For example, here is the basic output:

```shell
$ cruster cli http show a
    ID   METHOD                         HOSTNAME                                                                   PATH      STATUS          LENGTH

     0      GET                       google.com /                                                                              301             219
     1      GET                   www.google.com /                                                                              200           19957
     2      GET                       google.com /                                                                              301             219
     3      GET                   www.google.com /                                                                              200           19973
     4      GET                       google.com /some/interesting/path                                                         404            1582
```

CLI can extract records with specific attributes and print full URLs:

```shell
$ cruster cli http show -e status=404 -u a
     4 http://google.com/some/interesting/path
```

**Note**: There is required argument `<INDEX>` (that is actually `line number`) which represents range of line numbers with which CLI will work.

## repeater

