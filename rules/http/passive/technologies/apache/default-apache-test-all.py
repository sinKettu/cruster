metadata = {
    "id": "default-apache-test-all",
    "about": "Apache HTTP Server Test Page (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
import base64
import re
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if not any(
        re.search(p, body)
        for p in (
            r"<title>.*?Apache(|\d+) .*?(Default|Test).*?</title>",
            r"<title>(Default|Test).*? Apache(|\d+).*?</title>",
        )
    ):
        return []
    servers = [
        base64.b64decode(i["value"]).decode("latin-1")
        for i in response.headers
        if i["name"].lower() == "server"
    ]
    return [finding("default-apache-test-all", extracted=servers)]
