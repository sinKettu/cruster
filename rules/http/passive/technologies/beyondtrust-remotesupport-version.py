metadata = {
    "id": "beyondtrust-remotesupport-version",
    "about": "BeyondTrust Remote Support Version - Detect (passive Nuclei technology fingerprint)",
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
from datetime import datetime, timezone

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0]
    if target != "/get_rdf?comp=sdcust&locale_code=en-us":
        return []

    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)

    if "BeyondTrust" not in "\n".join(headers) + "\n\n" + body:
        return []

    timestamp_match = re.search(r"en-us\n(.*)\n", body)
    version_match = re.search(r"\x07.([\.\d]+)\x00", body)
    timestamp = timestamp_match.group(1) if timestamp_match else ""
    version = version_match.group(1) if version_match else ""
    evidence = ["Version: " + version, "Timestamp: " + timestamp]
    try:
        release_date = datetime.fromtimestamp(float(timestamp), timezone.utc).strftime(
            "%Y-%m-%d"
        )
        evidence.append("Release Date: " + release_date)
    except (OverflowError, ValueError):
        pass
    return [finding("beyondtrust-remotesupport-version", extracted=evidence)]
