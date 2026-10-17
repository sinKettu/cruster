metadata = {
    "id": "huggingface-tei-tgi-detect",
    "about": "HuggingFace TEI & TGI - Detect (passive Nuclei technology fingerprint)",
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


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target != "/info":
        return []

    status_match = re.match(r"\s*(\d+)", response.status)
    if status_match is None or int(status_match.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    content_types = []
    for header in response.headers:
        if header["name"].lower().replace("-", "_") != "content_type":
            continue
        try:
            content_types.append(base64.b64decode(header["value"]).decode("latin-1"))
        except Exception:
            return []
    if "application/json" not in "\n".join(content_types):
        return []

    if '"model_id"' not in body:
        return []
    is_tei = '"model_type"' in body and '"embedding"' in body
    is_tgi = '"text-generation-router"' in body
    if not is_tei and not is_tgi:
        return []

    versions = re.findall(r'"version"\s*:\s*"([0-9][0-9A-Za-z.\-+]*)"', body)
    model_ids = re.findall(r'"model_id"\s*:\s*"([^"]+)"', body)
    return [finding("huggingface-tei-tgi-detect", extracted=versions + model_ids)]
