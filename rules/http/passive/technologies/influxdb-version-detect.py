metadata = {
    "id": "influxdb-version-detect",
    "about": "InfluxDB Version Information - Detect (passive Nuclei technology fingerprint)",
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
    headers = []
    versions = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
        if item["name"].lower().replace("-", "_") == "x_influxdb_version":
            versions.append(value)

    if "X-Influxdb-" not in "\n".join(headers):
        return []
    return [finding("influxdb-version-detect", extracted=versions)]
