metadata = {
    "id": "dell-idrac9-detect",
    "about": "Detect Dell iDRAC9 (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    status = re.match(r"\s*(\d+)", response.status)
    body = base64.b64decode(response.body).decode("latin-1")
    extracted = [match.group(0) for match in re.finditer(r"[0-9]{2}G", body)]
    extracted.extend(
        match.group(1) for match in re.finditer(r'"FwVer" *: *"([^"]+)"', body)
    )
    if (
        target == "/sysmgmt/2015/bmc/info"
        and status
        and int(status.group(1)) == 200
        and ("BuildVersion" in body or "SystemModelName" in body)
    ):
        return [finding("dell-idrac9-detect", extracted=extracted)]
    return []
