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
    -r, --raw                  Print raw data as it was dumped in project (JSONLines)
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
$ cruster cli http show -e status=404 -u 3-5
     4 http://google.com/some/interesting/path
```

> **Note**: There is required argument `<INDEX>` (that is actually `line number`) which represents range of line numbers with which CLI will work. So, even if you will provide index (`-i <NUMBER>`), Cruster will look for it only within initial range.

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
