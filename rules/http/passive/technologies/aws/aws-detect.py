metadata = {
    "id": "aws-detect",
    "about": "AWS Service - Detect (passive Nuclei technology fingerprint)",
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
    del request
    headers = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)

    header = "\n".join(headers).lower()
    checks = (
        (
            "aws-service",
            (
                "x-amz-content-sha256:",
                "x-amz-date:",
                "x-amz-version-id:",
                "x-amz-id-2:",
                "x-amz-delete-marker:",
            ),
        ),
        (
            "aws-alb",
            ("server: awselb/2.0", "set-cookie: awsalb=", "set-cookie: awsalbcors="),
        ),
        ("aws-cloudfront", ("x-amz-cf-id:", "x-amz-cf-pop:")),
        (
            "aws-cloudfront",
            (
                "x-cache: hit from cloudfront",
                "x-cache: refreshhit from cloudfront",
                "x-cache: miss from cloudfront",
                "x-cache: error from cloudfront",
            ),
        ),
        (
            "aws-codebuild",
            (
                "arn: arn:aws:codebuild",
                "x-amz-meta-codebuild-buildarn:",
                "x-amz-meta-codebuild-content-sha256:",
                "x-amz-meta-codebuild-content-md5:",
            ),
        ),
        (
            "aws-api-gateway",
            (
                "x-amz-apigw-id:",
                "x-amzn-requestid:",
                "x-amzn-errortype: missingauthenticationtokenexception",
                "x-amzn-remapped-connection:",
                "x-amzn-remapped-content-length:",
                "x-amzn-remapped-date:",
            ),
        ),
        ("aws-kms", ("x-amz-server-side-encryption:",)),
        ("aws-xray", ("x-amzn-trace-id:",)),
        ("aws-waf-captcha", ("x-amzn-waf-action:",)),
        ("aws-dynamodb", ("x-amz-crc32:", "x-amz-target:")),
    )
    evidence = []
    for name, markers in checks:
        if any(marker in header for marker in markers) and name not in evidence:
            evidence.append(name)
    if evidence:
        return [finding("aws-detect", extracted=evidence)]
    return []
