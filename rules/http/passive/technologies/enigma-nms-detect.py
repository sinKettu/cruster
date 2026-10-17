metadata = {
    "id": "enigma-nms-detect",
    "about": "Enigma NMS - Detection (passive Nuclei technology fingerprint)",
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
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/cgi-bin/protected/main.cgi":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    lines = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        lines.append(f'{item["name"]}: {value}')
    header = "\n".join(lines)
    if (
        not status
        or int(status.group(1)) != 401
        or "enigma" not in header.lower()
        or "www-authenticate" not in header.lower()
    ):
        return []
    evidence = [
        match.group(1)
        for match in re.finditer(r'WWW-Authenticate:\s*Basic\s+realm="([^"]+)"', header)
    ]
    return [finding("enigma-nms-detect", extracted=evidence)]
