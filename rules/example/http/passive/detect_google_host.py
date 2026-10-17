metadata = {
    "id": "python-detect-google-host",
    "about": "Detect HTTP traffic sent to a Google host",
    "authors": ["Cruster"],
    "references": [],
    "tags": ["python", "host", "google", "passive"],
    "severity": "info",
    "type": "passive"
}


from cruster import HttpRequest, HttpResponse, finding, log


def scan(request: HttpRequest, response: HttpResponse):
    # The request URI is absolute in Cruster, so hostname extraction does not
    # depend on the presence or casing of the Host header.
    authority = request.uri.split("://", 1)[-1].split("/", 1)[0]
    hostname = authority.rsplit("@", 1)[-1].split(":", 1)[0].lower().rstrip(".")
    log(f"checking hostname {hostname}", level="debug")

    if hostname != "google.com" and not hostname.endswith(".google.com"):
        return []

    log(f"detected Google hostname {hostname}")
    return [finding("google-host", extracted=[hostname])]
