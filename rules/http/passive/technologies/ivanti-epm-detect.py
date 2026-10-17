metadata = {
    "id": "ivanti-epm-detect",
    "about": "Ivanti Endpoint Manager (EPM) - Detect (passive Nuclei technology fingerprint)",
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


def _mmh3(value):
    data = value.encode("latin-1")
    length = len(data)
    result = 0
    for offset in range(0, length - length % 4, 4):
        chunk = int.from_bytes(data[offset : offset + 4], "little")
        chunk = (chunk * 0xCC9E2D51) & 0xFFFFFFFF
        chunk = ((chunk << 15) | (chunk >> 17)) & 0xFFFFFFFF
        chunk = (chunk * 0x1B873593) & 0xFFFFFFFF
        result ^= chunk
        result = ((result << 13) | (result >> 19)) & 0xFFFFFFFF
        result = (result * 5 + 0xE6546B64) & 0xFFFFFFFF

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
        chunk = (chunk * 0x1B873593) & 0xFFFFFFFF
        result ^= chunk

    result ^= length
    result ^= result >> 16
    result = (result * 0x85EBCA6B) & 0xFFFFFFFF
    result ^= result >> 13
    result = (result * 0xC2B2AE35) & 0xFFFFFFFF
    result ^= result >> 16
    return result if result < 0x80000000 else result - 0x100000000


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in ("/favicon.ico", "/images/favicon.ico"):
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    encoded_body = base64.encodebytes(body.encode("latin-1")).decode("ascii")
    if _mmh3(encoded_body) != 362091310:
        return []

    return [finding("ivanti-epm-detect")]
