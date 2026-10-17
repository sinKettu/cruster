import base64
import re

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "posthog-rum-detect",
    "about": "PostHog Browser RUM - Detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies",
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}


PATTERNS = (
    r"window\.posthog",
    r"""api_host["']?\s*:\s*["'][^"']*posthog""",
    r"posthog\.init\s*\(",
)


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    detected = any(re.search(pattern, body) is not None for pattern in PATTERNS)
    return [finding("posthog-rum-detect", extracted=[])] if detected else []
