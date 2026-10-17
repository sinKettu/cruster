metadata = {
    "id": "cleo-detect",
    "about": "Cleo Technology - Detect (passive Nuclei technology fingerprint)",
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
    server = ""
    for item in response.headers:
        if item["name"].lower() == "server":
            try:
                server += ("\n" if server else "") + base64.b64decode(
                    item["value"]
                ).decode("latin-1")
            except Exception:
                pass
    if not any(
        word in server for word in ("Cleo VLTrader", "Cleo Harmony", "Cleo LexiCom")
    ):
        return []
    extracted = []
    for pattern in (
        r"Cleo VLTrader/([0-9.]+)",
        r"Cleo Harmony/([0-9.]+)",
        r"Cleo LexiCom/([0-9.]+)",
    ):
        extracted.extend(match.group(0) for match in re.finditer(pattern, server))
    return [finding("cleo-detect", extracted=list(dict.fromkeys(extracted)))]
