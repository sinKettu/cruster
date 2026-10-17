import base64

from cruster import HttpRequest, HttpResponse, finding

metadata = {
    "id": "plone-cms-detect",
    "about": "Plone CMS detect (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}

SIGNATURES = (
    '<meta name="generator" content="Plone - ',
    '<div xmlns:css="http://namespaces.plone.org/diazo/css"',
    "/++plone++static/plone-compiled.css",
    "/++plone++static/tinymce-styles.css",
    ">Powered by Plone &amp; Python</a>",
)


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    return (
        [finding("plone-cms-detect", extracted=[])]
        if any(value in body for value in SIGNATURES)
        else []
    )
