metadata = {
    "id": "jenkins-detect",
    "about": "Jenkins Detection (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    has_jenkins_header = False
    extracted = []
    for header in response.headers:
        name = header["name"]
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            value = ""

        normalized_name = name.lower().replace("-", "_")
        if normalized_name == "x_jenkins":
            has_jenkins_header = True
            if value:
                extracted.append(value)
        elif normalized_name == "version" and value:
            extracted.append(value)

    if not has_jenkins_header:
        return []
    if "Jenkins" not in body:
        return []

    return [finding("jenkins-detect", extracted=list(dict.fromkeys(extracted)))]
