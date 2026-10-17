metadata = {
    "id": "openreplay-rum-detect",
    "about": "OpenReplay RUM - Tech Detect (passive Nuclei technology fingerprint)",
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
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if path != "/":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    status = re.match(r"\s*(\d+)", response.status)
    if (
        not status
        or not 200 <= int(status.group(1)) < 400
        or "text/html" not in "\n".join(headers).lower()
    ):
        return []
    patterns = (
        r"static\.openreplay\.com/(?:latest|[0-9.]+)/openreplay\.js",
        r"""projectKey["']?\s*:\s*["'][A-Za-z0-9]{10,}["']""",
        r"""ingestPoint["']?\s*:\s*["']https?://[^"']+/ingest["']""",
        r"[a-z0-9-]+\.openreplay\.cloud/ingest",
        r"window\.OpenReplay\s*=\s*\[",
        r"openReplay\s*:\s*\{\s*projectKey",
        r"window\.asayer\s*=\s*\[",
        r"asayer\.io/ingest",
    )
    if any(re.search(pattern, body) for pattern in patterns):
        return [finding("openreplay-rum-detect", extracted=[])]
    return []
