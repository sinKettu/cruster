metadata = {
    "id": "bigip-config-utility",
    "about": "BIG-IP Configuration Utility detected (passive Nuclei technology fingerprint)",
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
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    target = target.split("#", 1)[0].split("?", 1)[0]

    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""

    if target in ("/tmui/login.jsp", "/tmui/tmui/login/welcome.jsp"):
        if "Welcome to the BIG-IP" in body and "Configuration Utility" in body:
            return [finding("bigip-config-utility")]
        return []

    if target != "/mgmt/tm/sys/management-ip":
        return []

    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
    header = "\n".join(headers)

    header_match = (
        "Www-Authenticate: Basic realm" in header and "Enterprise Manager" in header
    )
    body_match = "check your user-id and password and try again" in body
    if not (header_match or body_match):
        return []
    return [finding("bigip-config-utility")]
