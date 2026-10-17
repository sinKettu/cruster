metadata = {
    "id": "joomla-detect",
    "about": "Joomla! Detect (passive Nuclei technology fingerprint)",
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
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    words_match = all(
        word in body for word in ("<version>", "<creationDate>", "</metafile>")
    )
    regex_match = any(
        re.search(pattern, body)
        for pattern in (
            r'(?:<div[^>]+id="wrapper_r"|<(?:link|script)[^>]+(?:feed|components)/com_|<table[^>]+class="pill)',
            r'(?i)<meta\s+name=["\']generator["\']\s+content=["\']\s*joomla[^"\']*',
            r"(?i)Joomla_([\d.|\d]+)_version_history",
            r"FILES_JOOMLA_XML_DESCRIPTION",
        )
    )
    if not words_match and not regex_match:
        return []

    extracted = []
    for pattern in (
        r"<version>(.*?)</version>",
        r"(?i)Joomla_([\d.|\d]+)_version_history",
    ):
        extracted.extend(match.group(1) for match in re.finditer(pattern, body))
    return [finding("joomla-detect", extracted=list(dict.fromkeys(extracted)))]
