metadata = {
    "id": "python-missing-security-headers",
    "about": "Detect responses without basic browser security headers",
    "authors": ["Cruster"],
    "references": ["https://owasp.org/www-project-secure-headers/"],
    "tags": ["python", "headers", "passive"],
    "severity": "low",
    "type": "passive"
}


from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    names = [header["name"].lower() for header in response.headers]
    required = ["content-security-policy", "x-content-type-options"]
    missing = [name for name in required if name not in names]
    if not missing:
        return []
    return [finding("missing-security-headers", extracted=missing)]
