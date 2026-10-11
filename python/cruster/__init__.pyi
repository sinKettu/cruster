from typing import List, Optional, TypedDict


class HttpHeader(TypedDict):
    """An HTTP header. ``value`` is Base64-encoded to preserve arbitrary bytes."""

    name: str
    value: str


class HttpRequestDict(TypedDict):
    method: str
    uri: str
    version: str
    headers: List[HttpHeader]
    body: str


class HttpResponseDict(TypedDict):
    status: str
    version: str
    headers: List[HttpHeader]
    body: str


class HttpRequest:
    """Mutable request passed to a rule. ``body`` is Base64-encoded."""

    method: str
    uri: str
    version: str
    headers: List[HttpHeader]
    body: str

    def clone(self) -> "HttpRequest": ...
    def to_dict(self) -> HttpRequestDict: ...


class HttpResponse:
    """HTTP response exposed to a rule. ``body`` is Base64-encoded."""

    status: str
    version: str
    headers: List[HttpHeader]
    body: str

    def to_dict(self) -> HttpResponseDict: ...


class Exchange(TypedDict):
    request: HttpRequestDict
    response: HttpResponseDict
    payloads: List[str]


class Finding(TypedDict):
    name: str
    extracted: List[str]
    exchanges: List[Exchange]


def send(request: HttpRequest, max_redirects: int = 5) -> HttpResponse:
    """Send a request through Cruster. Available only in active rules."""
    ...


def finding(
    name: str,
    extracted: Optional[List[str]] = None,
    exchanges: Optional[List[Exchange]] = None,
) -> Finding:
    """Create one finding returned by ``scan``."""
    ...


def exchange(
    request: HttpRequest,
    response: HttpResponse,
    payloads: Optional[List[str]] = None,
) -> Exchange:
    """Attach an active request/response exchange as finding evidence."""
    ...
