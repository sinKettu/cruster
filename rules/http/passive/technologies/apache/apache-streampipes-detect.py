metadata = {
    "id": "apache-streampipes-detect",
    "about": "Apache StreamPipes - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64
from cruster import HttpRequest, HttpResponse, finding


def murmur3(data):
    length = len(data)
    h = 0
    for offset in range(0, length - length % 4, 4):
        k = int.from_bytes(data[offset : offset + 4], "little")
        k = (k * 0xCC9E2D51) & 0xFFFFFFFF
        k = ((k << 15) | (k >> 17)) & 0xFFFFFFFF
        k = (k * 0x1B873593) & 0xFFFFFFFF
        h ^= k
        h = ((h << 13) | (h >> 19)) & 0xFFFFFFFF
        h = (h * 5 + 0xE6546B64) & 0xFFFFFFFF
    k = 0
    tail = data[length - length % 4 :]
    for index, value in enumerate(tail):
        k |= value << (8 * index)
    if tail:
        k = (k * 0xCC9E2D51) & 0xFFFFFFFF
        k = ((k << 15) | (k >> 17)) & 0xFFFFFFFF
        k = (k * 0x1B873593) & 0xFFFFFFFF
        h ^= k
    h ^= length
    h ^= h >> 16
    h = (h * 0x85EBCA6B) & 0xFFFFFFFF
    h ^= h >> 13
    h = (h * 0xC2B2AE35) & 0xFFFFFFFF
    h ^= h >> 16
    return str(h if h < 0x80000000 else h - 0x100000000)


def scan(request: HttpRequest, response: HttpResponse):
    path = request.uri.split("://", 1)[-1]
    path = path[path.find("/") :] if "/" in path else "/"
    path = path.split("#", 1)[0].split("?", 1)[0] or "/"
    if (
        path
        not in (
            "/streampipes-backend/api/openapi.json",
            "/assets/img/favicon/favicon.ico",
        )
        and path != "/"
    ):
        pass
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    text = body.lower()
    digest = murmur3(base64.encodebytes(body.encode("latin-1")))
    if (
        "<title>apache streampipes" in text
        or "apache streampipes api" in text
        or (response.status.startswith("200") and digest in ("1937041138", "480680877"))
    ):
        return [finding("apache-streampipes-detect")]
    return []
