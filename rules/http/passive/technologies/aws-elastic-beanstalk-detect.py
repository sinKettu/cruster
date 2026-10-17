metadata = {
    "id": "elastic-beanstalk-detect",
    "about": "AWS Elastic Beanstalk Detect (passive Nuclei technology fingerprint)",
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
    status = re.match(r"\s*(\d+)", response.status)
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    markers = (
        "http://docs.aws.amazon.com/elasticbeanstalk/latest/dg/Welcome.html",
        "http://docs.amazonwebservices.com/elasticbeanstalk/latest/dg/",
    )
    if status is None or int(status.group(1)) != 200:
        return []
    if not any(marker in body for marker in markers):
        return []

    titles = [
        match.group(1) for match in re.finditer(r"<title>([A-Za-z -]+)</title>", body)
    ]
    return [finding("elastic-beanstalk-detect", extracted=titles)]
