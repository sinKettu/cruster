metadata = {
    "id": "dell-idrac7-detect",
    "about": "Detect Dell iDRAC7 (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0] or "/"
    status = re.match(r"\s*(\d+)", response.status)
    body = base64.b64decode(response.body).decode("latin-1")
    content_type = "\n".join(
        base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
        if item["name"].lower() == "content-type"
    )
    required = (
        "<prodServerGen>",
        "<Manufacturer>Dell",
        "<isDellBranded>1</isDellBranded>",
        "12G",
    )
    generations = [
        match.group(1)
        for match in re.finditer(r"<prodServerGen>([0-9]{2}G)</prodServerGen>", body)
    ]
    if (
        target == "/data?get=prodServerGen"
        and status
        and int(status.group(1)) == 200
        and "text/xml" in content_type
        and all(word in body for word in required)
    ):
        return [finding("dell-idrac7-detect", extracted=generations)]
    return []
