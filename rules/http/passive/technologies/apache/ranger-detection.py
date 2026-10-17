metadata = {
    "id": "ranger-detection",
    "about": "Apache Ranger Detection (passive Nuclei technology fingerprint)",
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
    hash_value = 0
    complete_length = length - length % 4

    for offset in range(0, complete_length, 4):
        block = int.from_bytes(data[offset : offset + 4], "little")
        block = (block * 0xCC9E2D51) & 0xFFFFFFFF
        block = ((block << 15) | (block >> 17)) & 0xFFFFFFFF
        block = (block * 0x1B873593) & 0xFFFFFFFF
        hash_value ^= block
        hash_value = ((hash_value << 13) | (hash_value >> 19)) & 0xFFFFFFFF
        hash_value = (hash_value * 5 + 0xE6546B64) & 0xFFFFFFFF

    tail_block = 0
    tail = data[complete_length:]
    for index, value in enumerate(tail):
        tail_block |= value << (8 * index)

    if tail:
        tail_block = (tail_block * 0xCC9E2D51) & 0xFFFFFFFF
        tail_block = ((tail_block << 15) | (tail_block >> 17)) & 0xFFFFFFFF
        tail_block = (tail_block * 0x1B873593) & 0xFFFFFFFF
        hash_value ^= tail_block

    hash_value ^= length
    hash_value ^= hash_value >> 16
    hash_value = (hash_value * 0x85EBCA6B) & 0xFFFFFFFF
    hash_value ^= hash_value >> 13
    hash_value = (hash_value * 0xC2B2AE35) & 0xFFFFFFFF
    hash_value ^= hash_value >> 16

    if hash_value >= 0x80000000:
        hash_value -= 0x100000000
    return str(hash_value)


def scan(request: HttpRequest, response: HttpResponse):
    uri_without_scheme = request.uri.split("://", 1)[-1]
    if "/" in uri_without_scheme:
        request_target = uri_without_scheme[uri_without_scheme.find("/") :]
    else:
        request_target = "/"
    request_target = request_target.split("#", 1)[0]
    request_target = request_target.split("?", 1)[0]
    if request_target not in ("/login.jsp", "/images/favicon.ico"):
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if "<title> Ranger - Sign In</title>" in body:
        return [finding("ranger-detection", extracted=[])]

    encoded_body = base64.encodebytes(body.encode("latin-1"))
    if response.status.startswith("200") and murmur3(encoded_body) == "1302629996":
        return [finding("ranger-detection", extracted=["favicon"])]

    return []
