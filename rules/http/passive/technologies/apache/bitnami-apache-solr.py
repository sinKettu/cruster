metadata = {
    "id": "bitnami-apache-solr",
    "about": "Bitnami Apache Solr Stack - Detection (passive Nuclei technology fingerprint)",
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


def scan(request: HttpRequest, response: HttpResponse):
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    return (
        [finding("bitnami-apache-solr")]
        if response.status.startswith("200") and "Bitnami Apache Solr Stack" in body
        else []
    )
