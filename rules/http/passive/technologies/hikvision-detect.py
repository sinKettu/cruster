metadata = {
    "id": "hikvision-detect",
    "about": "Hikvision Panel (passive Nuclei technology fingerprint)",
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


def _murmur_hash(value):
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
    if target not in ("/favicon.ico", "/doc/page/login.asp"):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    evidence = []
    if "Hikvision Digital Technology" in body:
        evidence.append("body")

    header_lines = []
    for header in response.headers:
        try:
            value = base64.b64decode(header["value"]).decode("latin-1")
        except Exception:
            return []
        header_lines.append(header["name"] + ": " + value)
    if "Hikvision-Webs" in "\n".join(header_lines):
        evidence.append("server")

    encoded_body = base64.encodebytes(body.encode("latin-1")).decode("ascii")
    if str(_murmur_hash(encoded_body)) == "999357577":
        evidence.append("favicon")

    if not evidence:
        return []
    return [finding("hikvision-detect", extracted=evidence)]
