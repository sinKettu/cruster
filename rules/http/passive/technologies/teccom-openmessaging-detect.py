metadata = {
    "id": "teccom-openmessaging-detect",
    "about": "TecConnect OpenMessaging Webservice Detection (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0]
    paths = (
        "/openmessaging.asmx?wsdl",
        "/tecopenmessaging/openmessaging.asmx?wsdl",
        "/tomconnect/openmessaging.asmx?wsdl",
        "/tomconnect_apo/openmessaging.asmx?wsdl",
        "/tompreprod/openmessaging.asmx?wsdl",
    )
    if target not in paths or response.status.lstrip().split(" ", 1)[0] != "200":
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    content_types = []
    for header in response.headers:
        if header["name"].lower() != "content-type":
            continue
        try:
            content_types.append(base64.b64decode(header["value"]).decode("latin-1"))
        except Exception:
            continue
    if not any("text/xml" in value for value in content_types):
        return []
    if "ProcessRequestResponse" not in body or "ProcessRequestResult" not in body:
        return []
    return [finding("teccom-openmessaging-detect")]
