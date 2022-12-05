# Storeed HTTP Data Format

All HTTP data stored in format of [JSONLines](https://jsonlines.org).

## Single Line Format

### Serializable HTTP Message

| Key | Value / Value Type | Comment |
| --- | --- | --- |
| index | *Integer* | Sequential number |
| request | *JSON* | Format described in `Serializable HTTP Request` section |
| response | *JSON* | Format described in `Serializable HTTP Response` section |

### Serializable HTTP Request

| Key | Value / Value Type | Comment |
| --- | --- | --- |
| method | *String* | HTTP Method |
| host | *String* | `Host` part of HTTP URI |
| path | *String* | `Path` part of HTTP URI |
| query | *String* OR *null* | `Query` part of HTTP URI, including `Anchor` |
| version | *String* | HTTP Version |
| headers | *List* | List containing `Header` structure. See `Serializable Headers` section |
| body | *String* OR *null* | Base64-encoded raw body bytes |

### Serializable HTTP Response

| Key | Value / Value Type | Comment |
| --- | --- | --- |
| status | *String* | HTTP Response Status |
| version | *String* | HTTP Version |
| headers | *List* | List containing `Header` structure. See `Serializable Headers` section |
| body | *String* OR *null* | Base64-encoded raw body bytes |

### Serializable Headers

| Key | Value / Value Type | Comment |
| --- | --- | --- |
| key | *String* | Header name (key) |
| encoding | *String* | Encoding method used. There are two options at the moment: `base64`, `utf-8` |
| value | *String* | Value encoded in string using encoding mentioned above |

## Example

``` json
{
  "index": 0,
  "request": {
    "method": "GET",
    "scheme": "http://",
    "host": "google.com",
    "path": "/search",
    "query": "?q=123",
    "version": "HTTP/1.1",
    "headers": [
      {
        "key": "host",
        "encoding": "utf-8",
        "value": "google.com"
      },
      {
        "key": "user-agent",
        "encoding": "utf-8",
        "value": "curl/7.85.0"
      },
      {
        "key": "accept",
        "encoding": "utf-8",
        "value": "*/*"
      },
      {
        "key": "proxy-connection",
        "encoding": "utf-8",
        "value": "Keep-Alive"
      }
    ],
    "body": null
  },
  "response": {
    "status": "301 Moved Permanently",
    "version": "HTTP/1.1",
    "headers": [
      {
        "key": "location",
        "encoding": "utf-8",
        "value": "http://www.google.com/search?q=123"
      },
      {
        "key": "content-type",
        "encoding": "utf-8",
        "value": "text/html; charset=UTF-8"
      },
      {
        "key": "content-security-policy",
        "encoding": "utf-8",
        "value": "object-src 'none';base-uri 'self';script-src 'nonce-WUklQPvck6VungCLCqaF4A' 'strict-dynamic' 'report-sample' 'unsafe-eval' 'unsafe-inline' https: http:;report-uri https://csp.withgoogle.com/csp/gws/xsrp"
      },
      {
        "key": "cross-origin-opener-policy-report-only",
        "encoding": "utf-8",
        "value": "same-origin-allow-popups; report-to=\"gws\""
      },
      {
        "key": "report-to",
        "encoding": "utf-8",
        "value": "{\"group\":\"gws\",\"max_age\":2592000,\"endpoints\":[{\"url\":\"https://csp.withgoogle.com/csp/report-to/gws/xsrp\"}]}"
      },
      {
        "key": "date",
        "encoding": "utf-8",
        "value": "Sun, 04 Dec 2022 18:05:38 GMT"
      },
      {
        "key": "expires",
        "encoding": "utf-8",
        "value": "Tue, 03 Jan 2023 18:05:38 GMT"
      },
      {
        "key": "cache-control",
        "encoding": "utf-8",
        "value": "public, max-age=2592000"
      },
      {
        "key": "server",
        "encoding": "utf-8",
        "value": "gws"
      },
      {
        "key": "content-length",
        "encoding": "utf-8",
        "value": "231"
      },
      {
        "key": "x-xss-protection",
        "encoding": "utf-8",
        "value": "0"
      },
      {
        "key": "x-frame-options",
        "encoding": "utf-8",
        "value": "SAMEORIGIN"
      }
    ],
    "body": "PEhUTUw+PEhFQUQ+PG1ldGEgaHR0cC1lcXVpdj0iY29udGVudC10eXBlIiBjb250ZW50PSJ0ZXh0L2h0bWw7Y2hhcnNldD11dGYtOCI+CjxUSVRMRT4zMDEgTW92ZWQ8L1RJVExFPjwvSEVBRD48Qk9EWT4KPEgxPjMwMSBNb3ZlZDwvSDE+ClRoZSBkb2N1bWVudCBoYXMgbW92ZWQKPEEgSFJFRj0iaHR0cDovL3d3dy5nb29nbGUuY29tL3NlYXJjaD9xPTEyMyI+aGVyZTwvQT4uDQo8L0JPRFk+PC9IVE1MPg0K"
  }
}
```
