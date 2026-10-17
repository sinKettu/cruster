metadata = {
    "id": "newrelic-rum-detect",
    "about": "New Relic Browser Monitoring (RUM) - Tech Detect (passive Nuclei technology fingerprint)",
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
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    indicators = (
        "nreum.loader_config",
        "newrelic-browser-agent",
        "js-agent.newrelic.com",
        "nreum",
        "nr-data",
    )
    if (
        status
        and int(status.group(1)) in (200, 400)
        and any(indicator in body for indicator in indicators)
    ):
        return [finding("newrelic-rum-detect")]
    return []
