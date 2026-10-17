metadata = {
    "id": "moinmoin-detect",
    "about": "MoinMoin wiki detect (passive Nuclei technology fingerprint)",
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
    del request
    body = base64.b64decode(response.body).decode("latin-1")
    status = re.match(r"\s*(\d+)", response.status)
    indicators = (
        '<a href="http://moinmo.in/" title="This site uses the MoinMoin Wiki software.">MoinMoin Powered</a>',
        '<a href="http://moinmo.in/Python" title="MoinMoin is written in Python.">Python Powered</a>',
    )
    if (
        not status
        or int(status.group(1)) != 200
        or not any(value in body for value in indicators)
    ):
        return []
    versions = [match.group(1) for match in re.finditer(r"/moin_static(\w+)/", body)]
    return [finding("moinmoin-detect", extracted=versions)]
