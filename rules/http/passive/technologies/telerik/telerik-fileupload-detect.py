metadata = {
    "id": "telerik-fileupload-detect",
    "about": "Detect Telerik Web UI Fileupload Handler (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}

import base64

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0] or "/"
    if target != "/Telerik.Web.UI.WebResource.axd?type=rau":
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    if "RadAsyncUpload handler is registered succesfully" in body:
        return [finding("telerik-fileupload-detect")]
    return []
