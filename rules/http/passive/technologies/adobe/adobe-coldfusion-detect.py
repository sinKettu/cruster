metadata = {
    "id": "adobe-coldfusion-detect",
    "about": "Adobe ColdFusion Detector (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive",
}
import base64
import hashlib
import re

from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    paths = (
        "/CFIDE/administrator/images/mx_login.gif",
        "/cfide/administrator/images/mx_login.gif",
        "/CFIDE/administrator/images/background.jpg",
        "/cfide/administrator/images/background.jpg",
        "/CFIDE/administrator/images/componentutilslogin.jpg",
        "/cfide/administrator/images/componentutilslogin.jpg",
    )
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0] or "/"
    if target not in paths:
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if status is None or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        return []
    digest = hashlib.md5(body.encode("latin-1")).hexdigest()
    versions = {
        "da07693b70ddbac5bc0d8bf98d4a3539": "coldfusion-8",
        "c0757351b00f7ecf35a035c976068d12": "coldfusion-9",
        "a4c81b7a6289b2fc9b36848fa0cae83c": "coldfusion-10",
        "7f024de9f480481ca03049e0d66679d6": "coldfusion-11",
        "f1281b6866aef66e35dc36fe4f0bf990": "coldfusion-2016",
        "a88530d7f1980412dac076de732a4e86": "coldfusion-2021",
        "92ef6ee3c4d1700e3cca797b19d3e7ba": "coldfusion-2018",
        "cb594e69af5ba15bca453f76aca53615": "coldfusion-mx-7",
    }
    if digest in versions:
        return [finding("adobe-coldfusion-detect", extracted=[versions[digest]])]
    return []
