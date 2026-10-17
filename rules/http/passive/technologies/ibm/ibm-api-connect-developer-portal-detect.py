metadata = {
    "id": "ibm-api-connect-detect",
    "about": "IBM API Connect Developer Portal - Detect (passive Nuclei technology fingerprint)",
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


def murmur3(value):
    data = value.encode("latin-1")
    length = len(data)
    result = 0
    for offset in range(0, length - length % 4, 4):
        block = int.from_bytes(data[offset : offset + 4], "little")
        block = (block * 0xCC9E2D51) & 0xFFFFFFFF
        block = ((block << 15) | (block >> 17)) & 0xFFFFFFFF
        block = (block * 0x1B873593) & 0xFFFFFFFF
        result ^= block
        result = ((result << 13) | (result >> 19)) & 0xFFFFFFFF
        result = (result * 5 + 0xE6546B64) & 0xFFFFFFFF
    tail = data[length - length % 4 :]
    block = 0
    if len(tail) == 3:
        block ^= tail[2] << 16
    if len(tail) >= 2:
        block ^= tail[1] << 8
    if tail:
        block ^= tail[0]
        block = (block * 0xCC9E2D51) & 0xFFFFFFFF
        block = ((block << 15) | (block >> 17)) & 0xFFFFFFFF
        block = (block * 0x1B873593) & 0xFFFFFFFF
        result ^= block
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
    if target not in ("/", "/favicon.ico", "/modules/ibm_apim/ibm_apim.info.yml"):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    lowered = body.lower()
    encoded = base64.encodebytes(body.encode("latin-1")).decode("ascii")
    matched = (
        "ibm developer portal is ready" in lowered
        or "package: ibm api developer" in lowered
        or murmur3(encoded) == 801517258
    )
    if not matched:
        return []
    versions = [
        match.group(1) for match in re.finditer(r"(?i)version:\s+([0-9a-z\-_.]+)", body)
    ]
    return [finding("ibm-api-connect-detect", extracted=versions)]
