metadata = {
    "id": "msexchange-eol",
    "about": "Microsoft Exchange Server End-of-Life - Detect (passive Nuclei technology fingerprint)",
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
    if target not in {"/owa/", "/owa/auth/logon.aspx"}:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    versions = []
    sources = (
        (r"(?i)X-Owa-Version:\s*([0-9.]+)", headers),
        (r'(?i)X-Owa-Version["\']?\s*[:=]\s*["\']?([0-9.]+)', headers),
        (r"(?i)/owa/([0-9.]+)/", body),
        (r'(?i)data-owa-version=["\']([0-9.]+)', body),
    )
    for pattern, value in sources:
        versions.extend(match.group(1) for match in re.finditer(pattern, value))
    if versions and tuple(map(int, versions[0].split("."))) < (15, 2, 2562):
        return [finding("msexchange-eol", extracted=list(dict.fromkeys(versions)))]
    return []
