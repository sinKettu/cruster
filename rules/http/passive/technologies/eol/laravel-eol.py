metadata = {
    "id": "laravel-eol",
    "about": "Laravel End-of-Life Detection (passive Nuclei technology fingerprint)",
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
    if target not in {"/", "/composer.json", "/composer.lock"}:
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    versions = []
    sources = (
        (r"(?i)x-powered-by:\s*Laravel[/\s]+([0-9]+\.[0-9]+(?:\.[0-9]+)?)", headers),
        (r"(?i)x-laravel-version:\s*([0-9]+\.[0-9]+(?:\.[0-9]+)?)", headers),
        (r'"laravel/framework":\s*"\^([0-9]+\.[0-9]+)', body),
        (r"&quot;laravelVersion&quot;:&quot;([0-9.]+)&quot;", body),
    )
    for pattern, value in sources:
        versions.extend(match.group(1) for match in re.finditer(pattern, value))
    identified = (
        "laravel" in body.lower()
        or "x-powered-by" in headers.lower()
        or "x-laravel-version" in headers.lower()
    )
    if (
        versions
        and identified
        and tuple(map(int, versions[0].split("."))) <= (11, 51, 0)
    ):
        return [
            finding(
                "laravel-eol",
                extracted=["eol-version-detected"] + list(dict.fromkeys(versions)),
            )
        ]
    return []
