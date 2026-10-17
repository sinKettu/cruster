metadata = {
    "id": "emqx-detect",
    "about": "Emqx Detection (passive Nuclei technology fingerprint)",
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
    if target != "/static/emq.ico":
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    data = base64.encodebytes(body.encode("latin-1"))
    length = len(data)
    digest = 0
    for offset in range(0, length - length % 4, 4):
        value = int.from_bytes(data[offset : offset + 4], "little")
        value = (value * 0xCC9E2D51) & 0xFFFFFFFF
        value = ((value << 15) | (value >> 17)) & 0xFFFFFFFF
        value = (value * 0x1B873593) & 0xFFFFFFFF
        digest ^= value
        digest = ((digest << 13) | (digest >> 19)) & 0xFFFFFFFF
        digest = (digest * 5 + 0xE6546B64) & 0xFFFFFFFF
    tail = data[length - length % 4 :]
    value = 0
    if len(tail) == 3:
        value ^= tail[2] << 16
    if len(tail) >= 2:
        value ^= tail[1] << 8
    if tail:
        value ^= tail[0]
        value = (value * 0xCC9E2D51) & 0xFFFFFFFF
        value = ((value << 15) | (value >> 17)) & 0xFFFFFFFF
        value = (value * 0x1B873593) & 0xFFFFFFFF
        digest ^= value
    digest ^= length
    digest ^= digest >> 16
    digest = (digest * 0x85EBCA6B) & 0xFFFFFFFF
    digest ^= digest >> 13
    digest = (digest * 0xC2B2AE35) & 0xFFFFFFFF
    digest ^= digest >> 16
    if digest >= 0x80000000:
        digest -= 0x100000000
    return (
        [finding("emqx-detect", extracted=["favicon"])] if digest == -670975485 else []
    )
