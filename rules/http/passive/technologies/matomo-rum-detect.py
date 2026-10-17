metadata = {
    "id": "matomo-rum-detect",
    "about": "Matomo (Piwik) RUM - Tech Detect (passive Nuclei technology fingerprint)",
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
    if target != "/":
        return []
    body = base64.b64decode(response.body).decode("latin-1")
    headers = "\n".join(
        item["name"] + ": " + base64.b64decode(item["value"]).decode("latin-1")
        for item in response.headers
    )
    status = re.match(r"\s*(\d+)", response.status)
    if not status or not 200 <= int(status.group(1)) < 400:
        return []
    if "text/html" not in headers.lower() and "<html" not in body.lower():
        return []
    patterns = (
        r"window\._paq",
        r"_paq\.push\s*\(",
        r"var\s+_paq\s*=\s*window\._paq",
        r"MatomoTagManager",
        r"window\.MatomoTagManager",
        r"_paq\.push\s*\(\s*\[\s*['\"]trackPageView['\"]",
        r"_paq\.push\s*\(\s*\[\s*['\"]setSiteId['\"]",
        r"_paq\.push\s*\(\s*\[\s*['\"]setTrackerUrl['\"]",
        r"_paq\.push\s*\(\s*\[\s*['\"]enableLinkTracking['\"]",
        r"(Matomo|Piwik)\.getTracker\s*\(",
        r"src\s*=\s*['\"][^'\"]*(matomo|piwik)\.js['\"]",
        r"g\.src\s*=\s*[^;]*(matomo|piwik)\.js",
        r"['\"]https?://[^'\"]+/(matomo|piwik)\.php['\"]",
        r"u\s*\+\s*['\"](matomo|piwik)\.php['\"]",
        r"[a-z0-9-]+\.matomo\.cloud/",
        r"<img[^>]+src\s*=\s*['\"][^'\"]*(matomo|piwik)\.php\?idsite=",
        r"name=['\"]generator['\"]\s+content=['\"](Matomo|Piwik)",
    )
    if any(re.search(pattern, body) for pattern in patterns):
        return [finding("matomo-rum-detect", extracted=["rum_indicators"])]
    return []
