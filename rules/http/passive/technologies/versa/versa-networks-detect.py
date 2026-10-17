metadata = {
    "id": "versa-networks-detect",
    "about": "Versa Networks Detection (passive Nuclei technology fingerprint)",
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
        chunk = (chunk * 0x1B873593) & 0xFFFFFFFF
        value ^= chunk
    value ^= length
    value ^= value >> 16
    value = (value * 0x85EBCA6B) & 0xFFFFFFFF
    value ^= value >> 13
    value = (value * 0xC2B2AE35) & 0xFFFFFFFF
    value ^= value >> 16
    if value < 0x80000000:
        return value
    return value - 0x100000000


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    paths = {
        "/favicon.ico",
        "/favicon.png",
        "/images/versalogo.png",
        "/images/versalogo2.png",
        "/common/images/Logo.png",
        "/versa/dist/images/versa-logo.png",
        "/versa/styles/img/versa-logo.png",
        "/versa/app/img/versa-logo.png",
    }
    status = re.match(r"\s*(\d+)", response.status)
    if target not in paths or not status or int(status.group(1)) != 200:
        return []
    value = _favicon_hash(base64.b64decode(response.body).decode("latin-1"))
    if value in {-1053531639, -1086395444, 186362384, 2033952700}:
        return [finding("versa-networks-detect", extracted=["versa-analytics"])]
    if value == -534530225:
        return [finding("versa-networks-detect", extracted=["versa-concerto"])]
    return []
