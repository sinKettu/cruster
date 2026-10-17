metadata = {
    "id": "aem-detect",
    "about": "Favicon based AEM Detection (passive Nuclei technology fingerprint)",
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


def _favicon_hash(body):
    data = base64.encodebytes(body.encode("latin-1"))
    length = len(data)
    value = 0

    for offset in range(0, length - length % 4, 4):
        chunk = int.from_bytes(data[offset : offset + 4], "little")
        chunk = (chunk * 0xCC9E2D51) & 0xFFFFFFFF
        chunk = ((chunk << 15) | (chunk >> 17)) & 0xFFFFFFFF
        chunk = (chunk * 0x1B873593) & 0xFFFFFFFF
        value ^= chunk
        value = ((value << 13) | (value >> 19)) & 0xFFFFFFFF
        value = (value * 5 + 0xE6546B64) & 0xFFFFFFFF

    tail = data[length - length % 4 :]
    chunk = 0
    if len(tail) == 3:
        chunk ^= tail[2] << 16
    if len(tail) >= 2:
        chunk ^= tail[1] << 8
    if tail:
        chunk ^= tail[0]
        chunk = (chunk * 0xCC9E2D51) & 0xFFFFFFFF
        chunk = ((chunk << 15) | (chunk >> 17)) & 0xFFFFFFFF
        value ^= (chunk * 0x1B873593) & 0xFFFFFFFF

    value ^= length
    value ^= value >> 16
    value = (value * 0x85EBCA6B) & 0xFFFFFFFF
    value ^= value >> 13
    value = (value * 0xC2B2AE35) & 0xFFFFFFFF
    value ^= value >> 16
    if value >= 0x80000000:
        value -= 0x100000000

    return value


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = (
        (target[target.find("/") :] if "/" in target else "/")
        .split("?", 1)[0]
        .split("#", 1)[0]
    )
    status = re.match(r"\s*(\d+)", response.status)
    if (
        target != "/libs/granite/core/content/login/favicon.ico"
        or status is None
        or int(status.group(1)) != 200
    ):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    if _favicon_hash(body) != -144483185:
        return []

    return [finding("aem-detect")]
