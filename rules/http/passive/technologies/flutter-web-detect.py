metadata = {
    "id": "flutter-web-detect",
    "about": "Flutter Web Application - Detect (passive Nuclei technology fingerprint)",
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
    target = target.split("?", 1)[0].split("#", 1)[0]
    if target not in (
        "/flutter_service_worker.js",
        "/main.dart.js",
        "/flutter_bootstrap.js",
    ):
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    markers = (
        "_flutter.loader.didCreateEngineInitializer",
        "_flutter.buildConfig",
        "flutter-app-manifest",
        "flutter-temp-cache",
        "flutter-app-cache",
        "ext.flutter.disassemble",
        "#flutterweb-theme",
    )
    return (
        [finding("flutter-web-detect")]
        if any(marker in body for marker in markers)
        else []
    )
