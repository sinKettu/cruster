metadata = {
    "id": "default-asp-net-page",
    "about": "ASP.Net Default Page (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = (
        "Home Page - My ASP.NET Application",
        "ASP.NET is a free web framework for building great Web sites and Web applications",
        "My ASP.NET Application",
    )
    return (
        [finding("default-asp-net-page")]
        if any(marker in body for marker in markers)
        else []
    )
