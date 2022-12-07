# Cruster Usage

Here I will try to describe some Cruster usecases.

## Beginning

When cruster starts, it opens a view with table, which contains short data about traffic went through proxy, and request-response previews.

![table](https://github.com/sinKettu/cruster/raw/master/static/http-table.png)

*Table*
 <!-- ![cruster](https://github.com/sinKettu/cruster/raw/master/static/cruster-main.png) -->

![request-response](https://github.com/sinKettu/cruster/raw/master/static/req_res.png)

*Request/Response*

Also, there is `Status Bar` on top with useful messages and simple statistics.

## Table

You can navigate over table with keys `<up>` and `<down>`. Keys `<left>` and `<right>` allow you to select a column to sort by. Pressed `<Enter>` you can see full request/response view.

![fullrequest-response](https://github.com/sinKettu/cruster/raw/master/static/full_req_res.png)
*Full Request/Response*

Views are scrollable, you can switch them with `<left>` and `<right>`. In *Cruster you can scroll anything scrollable with `<up>`/`<down>`, `<home>`/`<end>`, `<page up>`/`<page down>` and with `mouse wheel`.* Just like in GUI programs.

*Cruster shows only cut bodies of requests/responses, because large bodies can significantly slow down Cruster*. Unfortunately, I see no way for now, how I can fix it, seems like this behavior is a feature of TUI. If you want to see full body (for some reason), you can can store request/response (see below).

## Filtering

Pressed `<Shift> + f` you can see the Filter View. Here you can write regular expression. Press `<Enter>` to apply filter or `<Esc>` to refuse changes. Cruster fill filter out (without deletion) all requests/responses did not match. How matching work (in this order):

- match *first line* of request (i.e. `GET / HTTP/2\r\n`)
- match every *header line* of request (i.e. `host: example.com\r\n`)
- match *body* of request (i.e. `user=sinkettu`)
- match *first line* if response (i.e. `HTTP/2 200 OK\r\n`)
- match every *header line* (i.e. `server: example.com\r\n`)
- match *body* of response (i.e. `anything`)

If at least one of variants is matched, then request-response pair considered matched and is not filtered out.

*To cancel filtering*, in filter view just clear out filter (make it empty string) and apply it.

## Scope

Filter can be only one at a moment. If you want more powerfull and flexible control on the content in Cruster, you can use `Scope`. Scope is maintained with a config or CLI before Cruster starts. [Here](https://github.com/sinKettu/cruster/blob/master/docs/Cruster%20YAML%20Config%20Format.md) you can find how to make it. With scope you can define (by regular expressions tested against URIs) which requests/responses should be included, which ones should be excluded and if excluded ones should be just hidden or removed from storage fully.

*With excludings you can make includings more accurate*. For example, you want site `example.com` to included, but there is a page that is not interesting for you, i.e. `/this/page`. So, you can include `.*\.example\.com/.*` and exclude `.*\.example\.com/this/page($|/.*$)`.

There is no big deal that you can't change scope after start, because you always can store your data, change scope and reload data with new scope and this would not take much time. *I try to design Cruster so that you can return to it's current state after restarting the program, and so that it restarts are as fast as possible.*

## Store and Load

In the [config](https://github.com/sinKettu/cruster/blob/master/docs/Cruster%20YAML%20Config%20Format.md) or with CLI you can point a path to file where cruster should store it's data and a path where it should load data from.

When you store or load, you can control by `scope`, which requests/responses should not be stored or loaded.

[Here](https://github.com/sinKettu/cruster/blob/master/docs/Stored%20HTTP%20Data%20Format.md) you can find description of format of stored data.

For now, stored data is the only way to get bodies or some headers' values in their origin form, because they can be too long to render them in terminal or they can have unusual encoding, which will be decoded lossely.
