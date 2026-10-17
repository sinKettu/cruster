metadata = {
    "id": "bigcommerce-detect",
    "about": "BigCommerce Detection (passive Nuclei technology fingerprint)",
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

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)

    complete_response = "\n".join(headers) + "\n\n" + body
    lowered = complete_response.lower()
    if not any(
        marker in lowered
        for marker in (
            "powered by bigcommerce",
            "x-bigcommerce-request-id",
            "bigcommerce.com",
        )
    ):
        return []

    evidence = []
    for pattern in (
        r"BigCommerce ([0-9.]+)",
        r"Powered by BigCommerce ([0-9.]+)",
        r"bigcommerce-site-id=([a-z0-9]+)",
        r'bc-site-id="([a-zA-Z0-9]+)"',
        r"bigcommerce\.com/([^/]+)",
    ):
        evidence.extend(match.group(0) for match in re.finditer(pattern, body))
    return [finding("bigcommerce-detect", extracted=list(dict.fromkeys(evidence)))]
