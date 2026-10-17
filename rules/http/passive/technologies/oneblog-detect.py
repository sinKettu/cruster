qmetadata = {
    "id": "oneblog-detect",
    "about": "OneBlog Detect (passive Nuclei technology fingerprint)",
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
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    status = re.match(r"\s*(\d+)", response.status)
    marker = (
        "<title>OneBlog开源博客后台管理系统</title>" in body
        or '<meta name="keywords" content="OneBlog' in body
    )
    if status and int(status.group(1)) == 200 and marker:
        return [finding("oneblog-detect", extracted=[])]
    return []
