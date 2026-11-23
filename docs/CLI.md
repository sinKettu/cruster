# Cruster CLI Usage

Cruster CLI interface is similar to such programs like `Docker`, `Kubectl`, `Git`, etc.

CLI works with data, stored in dump or interactive modes, so it is required to provide a path to project to use CLI.

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

    Usage: cruster http show [OPTIONS] [INDEX]

    Arguments:
    [INDEX]  Records to print: omitted or n -- first n, a -- all, n-m -- inclusive range, -n -- last n, iN/Ni -- pair with ID N (pretty by default) [default: 100]

    Options:
    -u, --urls                 Print only indexes and full URLs
    -p, --pretty               Print full formated requests and responses (if any)
    -b, --no-body              Do not print request and response bodies in pretty mode
    -c, --no-cookie            Replace Cookie and Set-Cookie header values with 'stripped' in pretty output
    -r, --raw                  Print raw data as it was dumped in project (JSONLines)
    -f, --filter <filter>      Filter pairs in specifyied bounds with regular expression in format of 're2'
    -e, --extract <ATTRIBUTE>  Extract pairs from range by attribute. parameter syntax: method=<name>|status=<value>|host=<prefix>|path=<prefix>
    -i, --index <NUMBER>       Get pair with specific ID (pretty by default; supports --raw and --urls)
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
$ cruster cli http show -e status=404 -u 3-5
     4 http://google.com/some/interesting/path
```

`INDEX` is optional and defaults to the first 100 records. An explicit pair ID passed as `iN`, `Ni`, or with `-i <NUMBER>` is searched across the complete HTTP history and printed in pretty mode by default. Explicit output options (`--raw`, `--urls`, or `--pretty`) work with every selection syntax and take precedence over the default. For `-i`, the positional range is ignored, but output options are preserved.

```shell
cruster cli http show 2i --raw
cruster cli http show i2 --urls
cruster cli http show --index 2 --pretty --no-body
```

Raw output returns the original JSONL records; `--filter` searches those records directly, including when selecting an exact ID.

Use `-b` or `--no-body` with pretty output, including automatic pretty output for an exact ID, to print request/response lines and headers without their bodies.

Use `-c` or `--no-cookie` to keep `Cookie` and `Set-Cookie` headers visible while replacing their values with `stripped`. It works with every selection syntax in pretty output and can be combined with `--no-body`:

```shell
cruster cli http show 2i --no-cookie
cruster cli http show 2i -c
cruster cli http show a --pretty --no-body --no-cookie
```

This is output-only redaction: stored records and filtering still use the original values. Only cookie header values are hidden, not cookie-like text in bodies or URLs. Like `--no-body`, `--no-cookie` cannot be combined with `--raw`, which preserves original JSONL records.

## Audit HTTP-pair filtering

Use the repeatable `--filter` (`-f`) option when running an audit:

```shell
cruster audit run selected -f '^GET https?://example\.com/' -f '(?i)x-powered-by:.*php'
```

Only HTTP pairs matching at least one expression are passed to audit rules
(logical OR). The search checks request and response first lines, headers, and
bodies in sequence using the same matcher as HTTP history filtering. The patterns
are appended to the `filter` list in the audit YAML configuration. With no patterns,
all pairs remain eligible. Invalid regular expressions fail before scanning begins.

## repeater

`repeater` has the following subcommands:

- `list` - print list of parameters of all repeaters in project.
- `show` - print details of specific repeater.

    ```shell
    $ cruster cli repeater show -h
    Show specific repeater state verbously

    Usage: cruster cli repeater show [OPTIONS] <mark>

    Arguments:
    <mark>  Number or name of repeater to print

    Options:
    -b, --no-body  Print request/response without body
    -c, --no-cookie  Replace Cookie and Set-Cookie header values with 'stripped' in output
    -h, --help     Print help
    ```

- `exec` - execute repeater (send request, receive response).

    ```shell
    $ cruster cli repeater exec -h
    Execute choosed repeater

    Usage: cruster cli repeater exec [OPTIONS] <mark>

    Arguments:
    <mark>  Number or name of repeater to execute

    Options:
    -f, --force    Execute repeater without editing
    -b, --no-body  Print request/response without body
    -c, --no-cookie  Replace Cookie and Set-Cookie header values with 'stripped' in output
    -h, --help     Print help
    ```

- `edit` - edit parameters of specific repeater.

    ```shell
    $ cruster cli repeater edit -h
    Edit repeaters' parameters

    Usage: cruster cli repeater edit [OPTIONS] <mark>

    Arguments:
    <mark>  Number or name of repeater to edit

    Options:
    -s, --https <https>             Enable/disable https, syntax [possible values: true, false]
    -r, --redirects <redirects>     Enable/disable redirects following [possible values: true, false]
    -m, --max-redirects <NUMBER>    Maximum number of redirects
    -n, --name <NAME>               A name for the repeater
    -a, --address <IP-OR-HOSTNAME>  Host to send request to
    -h, --help                      Print help
    ```

- `add` - create new repeater using records from HTTP history.

Usage of every comand is pretty simple, just follow arguments described in help output.

Both `repeater show` and `repeater exec` support `-c` / `--no-cookie`, optionally together with `--no-body`. Cookie headers are shown as `Cookie: stripped` / `Set-Cookie: stripped`; saved repeaters and the requests sent over the network retain their original cookies.

### Editing request in repeater via external editor

To edit requests before repeating in CLI mode you can use external text editors like `Vim`. I prefer [Micro](https://github.com/zyedidia/micro). You can use almost *any* text editor. There are only two conditions:

1. Editor must be able to open files via first command line argument. For example:

    ```shell
    vim /path/to/file.txt
    ```

2. You have to save changes and then **quit editor** to bring control back to Cruster (it is very similar to editing commits in `git rebase ...`)

> I have not tested it, but theoretically you can use even graphical text editors.

There are two ways to enable editing with external editor:

1. Use `--editor` global command line argument:

    ```shell
    cruster --editor /usr/bin/vim ...
    ```

    or just

    ```shell
    cruster --editor vim
    ```

2. Specify editor in `config`. Details you can find in description of [config format](https://github.com/sinKettu/cruster/blob/master/docs/Cruster%20YAML%20Config%20Format.md).
