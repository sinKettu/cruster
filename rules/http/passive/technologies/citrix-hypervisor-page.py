metadata = {
    "id": "citrix-hypervisor-page",
    "about": "Citrix Hypervisor Page Detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    if "Welcome to Citrix Hypervisor" not in body or "XenCenter" not in body:
        return []
    extracted = [
        match.group(1)
        for match in re.finditer(r"Welcome to Citrix Hypervisor ([0-9. ]+)", body)
    ]
    return [finding("citrix-hypervisor-page", extracted=extracted)]
