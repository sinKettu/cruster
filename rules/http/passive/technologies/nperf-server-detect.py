metadata = {
    "id": "nperf-server-detect",
    "about": "nPerf Server - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    server_values = [
        base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
        if item["name"].lower() == "server"
    ]
    header_indicators = ("Nperfserver-Remote-Endpoint", "nPerf/")
    if "<title>nPerfServer</title>" in body or any(
        value in headers for value in header_indicators
    ):
        return [finding("nperf-server-detect", extracted=server_values)]
    return []
