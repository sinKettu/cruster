metadata = {
    "id": "livewire-detect",
    "about": "Laravel Livewire - Detect (passive Nuclei technology fingerprint)",
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
    markers = (
        "wire:id=",
        "wire:initial-data=",
        "window.livewire_app_url",
        "window.livewire_token",
        "livewire.js",
        "livewire.min.js",
    )
    if any(marker in body for marker in markers):
        return [finding("livewire-detect")]
    return []
