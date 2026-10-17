metadata = {
    "id": "wordpress-detect",
    "about": "WordPress Detect (passive Nuclei technology fingerprint)",
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


def scan(_request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    detection_patterns = [
        r"<generator>https?:\/\/wordpress\.org.*</generator>",
        r"wp-login.php",
        r'name="generator" content="wordpress',
        r"<link[^>]+s\d+\.wp\.com",
        r"<!-- This site is optimized with the Yoast (?:WordPress )?SEO plugin v([\d.]+) -",
        r"<!--[^>]+WP-Super-Cache",
        r"/wp-content/uploads/(.*)",
    ]
    if not any(re.search(pattern, body) for pattern in detection_patterns):
        return []
    extraction_patterns = [
        r"(?m)https:\/\/wordpress.org\/\?v=([0-9.]+)",
        r"wp-emoji-release\.min\.js\?ver=((\d+\.?)+)\b",
        r"install\.min\.css\?ver=((\d+\.?)+)\b",
    ]
    extracted = []
    for pattern in extraction_patterns:
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("wordpress-detect", extracted=extracted)]
