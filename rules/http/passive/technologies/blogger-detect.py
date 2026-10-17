metadata = {
    "id": "blogger-detect",
    "about": "Blogger Detection (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    full_response = ("\n".join(headers) + "\n\n" + body).lower()

    if not any(
        marker in full_response
        for marker in (
            "powered by blogger",
            "blogspot.com",
            "x-powered-by-blogger",
            "blogger.com",
        )
    ):
        return []

    extracted = []
    for pattern in (
        r"Blogger ([0-9.]+)",
        r"Powered by Blogger ([0-9.]+)",
        r"blogger-id=([a-z0-9]+)",
        r"blogspot\.com/([^/]+)",
    ):
        extracted.extend(match.group(0) for match in re.finditer(pattern, body))
    return [finding("blogger-detect", extracted=list(dict.fromkeys(extracted)))]
