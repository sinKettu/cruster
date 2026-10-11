import base64

from cruster import HttpRequest, HttpResponse, exchange, finding, send

metadata = {
    "id": "python-reflected-marker",
    "about": "Inject a marker into the query and detect reflection",
    "authors": ["Cruster"],
    "references": [],
    "tags": ["python", "reflection", "active"],
    "severity": "medium",
    "type": "active",
}


def scan(request: HttpRequest, response: HttpResponse):
    marker = "cruster-python-marker-7391"
    changed = request.clone()
    separator = "&" if "?" in changed.uri else "?"
    changed.uri += separator + "cruster_marker=" + marker
    probe = send(changed)
    body = base64.b64decode(probe.body).decode("utf-8", errors="replace")
    if marker not in body:
        return []
    return [finding(
        "reflected-marker",
        extracted=[marker],
        exchanges=[exchange(changed, probe, payloads=[marker])],
    )]
