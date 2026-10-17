metadata = {
    "id": "favicon-detect",
    "about": "favicon-detection (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
FAVICON_SIGNATURES = [
    {
        "id": "favicon-detect",
        "name": "favicon-detection",
        "r": [
            {
                "p": ["/favicon.ico", "/images/favicon.ico"],
                "c": "or",
                "m": [
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("385597939" == mmh3(base64_py(body)))'
                        ],
                        "name": "skysea-clientview",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-60395993" == mmh3(base64_py(body)))'
                        ],
                        "name": "ictbroadcast",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("394527072" == mmh3(base64_py(body)))'
                        ],
                        "name": "brightdata-proxies",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("213144638" == mmh3(base64_py(body)))'
                        ],
                        "name": "proxmox",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-503480258" == mmh3(base64_py(body)))'
                        ],
                        "name": "blogger",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1414300307" == mmh3(base64_py(body)))'
                        ],
                        "name": "oracle",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("92627686" == mmh3(base64_py(body)))'
                        ],
                        "name": "hitachi",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("94231047" == mmh3(base64_py(body)))'
                        ],
                        "name": "hitachi-management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-676779906" == mmh3(base64_py(body)))'
                        ],
                        "name": "meinberg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("99395752" == mmh3(base64_py(body)))'
                        ],
                        "name": "slack-instance",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("116323821" == mmh3(base64_py(body)))'
                        ],
                        "name": "spring-boot",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-235701012" == mmh3(base64_py(body)))'
                        ],
                        "name": "cnservers-llc",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("743365239" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2128230701" == mmh3(base64_py(body)))'
                        ],
                        "name": "chainpoint",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1277814690" == mmh3(base64_py(body)))'
                        ],
                        "name": "lacie",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("246145559" == mmh3(base64_py(body)))'
                        ],
                        "name": "parse",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("628535358" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("855273746" == mmh3(base64_py(body)))'
                        ],
                        "name": "jira",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1318124267" == mmh3(base64_py(body)))'
                        ],
                        "name": "avigilon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-305179312" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian-confluence",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("786533217" == mmh3(base64_py(body)))'
                        ],
                        "name": "openstack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("432733105" == mmh3(base64_py(body)))'
                        ],
                        "name": "pi-star",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("705143395" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1255347784" == mmh3(base64_py(body)))'
                        ],
                        "name": "angular-io-angularjs",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1275226814" == mmh3(base64_py(body)))'
                        ],
                        "name": "xampp",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2009722838" == mmh3(base64_py(body)))'
                        ],
                        "name": "react",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("981867722" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian-jira",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-923088984" == mmh3(base64_py(body)))'
                        ],
                        "name": "openstack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("494866796" == mmh3(base64_py(body)))'
                        ],
                        "name": "aplikasi",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1249285083" == mmh3(base64_py(body)))'
                        ],
                        "name": "ubiquiti-aircube",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1379982221" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian-bamboo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("420473080" == mmh3(base64_py(body)))'
                        ],
                        "name": "exostar-managed-access-gateway",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1642532491" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian-confluence",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("163842882" == mmh3(base64_py(body)))'
                        ],
                        "name": "cisco-meraki",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1378182799" == mmh3(base64_py(body)))'
                        ],
                        "name": "archivematica",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-702384832" == mmh3(base64_py(body)))'
                        ],
                        "name": "tcn",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-532394952" == mmh3(base64_py(body)))'
                        ],
                        "name": "cx",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-183163807" == mmh3(base64_py(body)))'
                        ],
                        "name": "ace",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("552727997" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian-jira",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1302486561" == mmh3(base64_py(body)))'
                        ],
                        "name": "netdata",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-609520537" == mmh3(base64_py(body)))'
                        ],
                        "name": "opengeo-suite",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1961046099" == mmh3(base64_py(body)))'
                        ],
                        "name": "dgraph-ratel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1581907337" == mmh3(base64_py(body)))'
                        ],
                        "name": "atlassian-jira",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1913538826" == mmh3(base64_py(body)))'
                        ],
                        "name": "material-dashboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1319699698" == mmh3(base64_py(body)))'
                        ],
                        "name": "form.io",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1203021870" == mmh3(base64_py(body)))'
                        ],
                        "name": "kubeflow",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-182423204" == mmh3(base64_py(body)))'
                        ],
                        "name": "netdata-dashboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("988422585" == mmh3(base64_py(body)))'
                        ],
                        "name": "caprover",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2113497004" == mmh3(base64_py(body)))'
                        ],
                        "name": "wijungle",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1234311970" == mmh3(base64_py(body)))'
                        ],
                        "name": "onera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("430582574" == mmh3(base64_py(body)))'
                        ],
                        "name": "smartping",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1232596212" == mmh3(base64_py(body)))'
                        ],
                        "name": "openstack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1585145626" == mmh3(base64_py(body)))'
                        ],
                        "name": "netdata-dashboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-219752612" == mmh3(base64_py(body)))'
                        ],
                        "name": "fritz!box",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-697231354" == mmh3(base64_py(body)))'
                        ],
                        "name": "ubiquiti-airos",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("945408572" == mmh3(base64_py(body)))'
                        ],
                        "name": "fortinet-forticlient",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1768726119" == mmh3(base64_py(body)))'
                        ],
                        "name": "outlook-web-application",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2109473187" == mmh3(base64_py(body)))'
                        ],
                        "name": "huawei-claro",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("552592949" == mmh3(base64_py(body)))'
                        ],
                        "name": "asus-aicloud",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("631108382" == mmh3(base64_py(body)))'
                        ],
                        "name": "sonicwall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("708578229" == mmh3(base64_py(body)))'
                        ],
                        "name": "google",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-134375033" == mmh3(base64_py(body)))'
                        ],
                        "name": "plesk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2019488876" == mmh3(base64_py(body)))'
                        ],
                        "name": "dahua-storm-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1395400951" == mmh3(base64_py(body)))'
                        ],
                        "name": "huawei-adsl -router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1601194732" == mmh3(base64_py(body)))'
                        ],
                        "name": "sophos-cyberoam-appliance",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-325082670" == mmh3(base64_py(body)))'
                        ],
                        "name": "lancom-systems",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1050786453" == mmh3(base64_py(body)))'
                        ],
                        "name": "plesk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1346447358" == mmh3(base64_py(body)))'
                        ],
                        "name": "tilginab-homegateway",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1410610129" == mmh3(base64_py(body)))'
                        ],
                        "name": "supermicro-intelligent-management-ipmi",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-440644339" == mmh3(base64_py(body)))'
                        ],
                        "name": "zyxel-zywall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("363324987" == mmh3(base64_py(body)))'
                        ],
                        "name": "dell-sonicwall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1446794564" == mmh3(base64_py(body)))'
                        ],
                        "name": "ubiquiti-login-portals",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1045696447" == mmh3(base64_py(body)))'
                        ],
                        "name": "sophos-user-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-297069493" == mmh3(base64_py(body)))'
                        ],
                        "name": "apache-tomcat",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("396533629" == mmh3(base64_py(body)))'
                        ],
                        "name": "openvpn",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1462981117" == mmh3(base64_py(body)))'
                        ],
                        "name": "cyberoam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1772087922" == mmh3(base64_py(body)))'
                        ],
                        "name": "asp.net-favicon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1594377337" == mmh3(base64_py(body)))'
                        ],
                        "name": "technicolor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("165976831" == mmh3(base64_py(body)))'
                        ],
                        "name": "vodafone-technicolor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1677255344" == mmh3(base64_py(body)))'
                        ],
                        "name": "ubnt-router-ui",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-359621743" == mmh3(base64_py(body)))'
                        ],
                        "name": "intelbras-wireless",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-677167908" == mmh3(base64_py(body)))'
                        ],
                        "name": "kerio-connect-webmail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("878647854" == mmh3(base64_py(body)))'
                        ],
                        "name": "big-ip",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("442749392" == mmh3(base64_py(body)))'
                        ],
                        "name": "microsoft-owa",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1405460984" == mmh3(base64_py(body)))'
                        ],
                        "name": "pfsense",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-271448102" == mmh3(base64_py(body)))'
                        ],
                        "name": "ikuai-networks",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("31972968" == mmh3(base64_py(body)))'
                        ],
                        "name": "dlink-webcam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("970132176" == mmh3(base64_py(body)))'
                        ],
                        "name": "3cx-phone-system",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1119613926" == mmh3(base64_py(body)))'
                        ],
                        "name": "bluehost",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("123821839" == mmh3(base64_py(body)))'
                        ],
                        "name": "sangfor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("459900502" == mmh3(base64_py(body)))'
                        ],
                        "name": "zte-corporation-gateway-appliance",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2069844696" == mmh3(base64_py(body)))'
                        ],
                        "name": "ruckus-wireless",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1607644090" == mmh3(base64_py(body)))'
                        ],
                        "name": "bitnami",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2141724739" == mmh3(base64_py(body)))'
                        ],
                        "name": "juniper-device-manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1835479497" == mmh3(base64_py(body)))'
                        ],
                        "name": "technicolor-gateway",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1278323681" == mmh3(base64_py(body)))'
                        ],
                        "name": "gitlab",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1929912510" == mmh3(base64_py(body)))'
                        ],
                        "name": "netasq-secure-stormshield",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1255992602" == mmh3(base64_py(body)))'
                        ],
                        "name": "vmware-horizon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1895360511" == mmh3(base64_py(body)))'
                        ],
                        "name": "vmware-horizon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-991123252" == mmh3(base64_py(body)))'
                        ],
                        "name": "vmware-horizon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1642701741" == mmh3(base64_py(body)))'
                        ],
                        "name": "vmware-secure-file-transfer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-266008933" == mmh3(base64_py(body)))'
                        ],
                        "name": "sap-netweaver",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1967743928" == mmh3(base64_py(body)))'
                        ],
                        "name": "sap-id-service",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1347937389" == mmh3(base64_py(body)))'
                        ],
                        "name": "sap-conversational-ai",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("602431586" == mmh3(base64_py(body)))'
                        ],
                        "name": "palo-alto-login-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-318947884" == mmh3(base64_py(body)))'
                        ],
                        "name": "palo-alto-networks",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1356662359" == mmh3(base64_py(body)))'
                        ],
                        "name": "outlook-web-application",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1453890729" == mmh3(base64_py(body)))'
                        ],
                        "name": "webmin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1814887000" == mmh3(base64_py(body)))'
                        ],
                        "name": "docker",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1937209448" == mmh3(base64_py(body)))'
                        ],
                        "name": "docker",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1544605732" == mmh3(base64_py(body)))'
                        ],
                        "name": "amazon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("716989053" == mmh3(base64_py(body)))'
                        ],
                        "name": "amazon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1010568750" == mmh3(base64_py(body)))'
                        ],
                        "name": "phpmyadmin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1240222446" == mmh3(base64_py(body)))'
                        ],
                        "name": "zhejiang-uniview-technologies",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-986678507" == mmh3(base64_py(body)))'
                        ],
                        "name": "isp-manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1616143106" == mmh3(base64_py(body)))'
                        ],
                        "name": "axis-network-cameras",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-976235259" == mmh3(base64_py(body)))'
                        ],
                        "name": "roundcube-webmail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("768816037" == mmh3(base64_py(body)))'
                        ],
                        "name": "unifi-video-controller-airvision",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1015545776" == mmh3(base64_py(body)))'
                        ],
                        "name": "pfsense",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1838417872" == mmh3(base64_py(body)))'
                        ],
                        "name": "freebox-os",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1188645141" == mmh3(base64_py(body)))'
                        ],
                        "name": "hxxps",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("547282364" == mmh3(base64_py(body)))'
                        ],
                        "name": "keenetic",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1571472432" == mmh3(base64_py(body)))'
                        ],
                        "name": "sierra-wireless-ace-manager-airlink",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("149371702" == mmh3(base64_py(body)))'
                        ],
                        "name": "synology-diskstation",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1169314298" == mmh3(base64_py(body)))'
                        ],
                        "name": "instar-ip-cameras",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1038557304" == mmh3(base64_py(body)))'
                        ],
                        "name": "webmin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1307375944" == mmh3(base64_py(body)))'
                        ],
                        "name": "octoprint-3d-printer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1280907310" == mmh3(base64_py(body)))'
                        ],
                        "name": "webmin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1954835352" == mmh3(base64_py(body)))'
                        ],
                        "name": "vesta-hosting-control-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("509789953" == mmh3(base64_py(body)))'
                        ],
                        "name": "farming-simulator-dedicated-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1933493443" == mmh3(base64_py(body)))'
                        ],
                        "name": "residential-gateway",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1993518473" == mmh3(base64_py(body)))'
                        ],
                        "name": "cpanel-login",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1477563858" == mmh3(base64_py(body)))'
                        ],
                        "name": "arris",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-895890586" == mmh3(base64_py(body)))'
                        ],
                        "name": "plex-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1354933624" == mmh3(base64_py(body)))'
                        ],
                        "name": "dlink-webcam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("944969688" == mmh3(base64_py(body)))'
                        ],
                        "name": "deluge",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("479413330" == mmh3(base64_py(body)))'
                        ],
                        "name": "webmin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-435817905" == mmh3(base64_py(body)))'
                        ],
                        "name": "cambium-networks",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-981606721" == mmh3(base64_py(body)))'
                        ],
                        "name": "plesk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("833190513" == mmh3(base64_py(body)))'
                        ],
                        "name": "dahua-storm-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1314864135" == mmh3(base64_py(body)))'
                        ],
                        "name": "10",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-652508439" == mmh3(base64_py(body)))'
                        ],
                        "name": "parallels-plesk-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-569941107" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireware-watchguard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1326164945" == mmh3(base64_py(body)))'
                        ],
                        "name": "shock&innovation-netis-setup",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1738184811" == mmh3(base64_py(body)))'
                        ],
                        "name": "cacaoweb",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("904434662" == mmh3(base64_py(body)))'
                        ],
                        "name": "loxone-automation",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("905744673" == mmh3(base64_py(body)))'
                        ],
                        "name": "hp-printer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("902521196" == mmh3(base64_py(body)))'
                        ],
                        "name": "netflix",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2063036701" == mmh3(base64_py(body)))'
                        ],
                        "name": "linksys-smart-wi-fi",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1205024243" == mmh3(base64_py(body)))'
                        ],
                        "name": "lwip-a-lightweight-tcp-ip-stack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("607846949" == mmh3(base64_py(body)))'
                        ],
                        "name": "hitron-technologies",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1281253102" == mmh3(base64_py(body)))'
                        ],
                        "name": "dahua-storm-dvr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("661332347" == mmh3(base64_py(body)))'
                        ],
                        "name": "mobotix-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-520888198" == mmh3(base64_py(body)))'
                        ],
                        "name": "blue-iris-webcam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("104189364" == mmh3(base64_py(body)))'
                        ],
                        "name": "vigor-router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1227052603" == mmh3(base64_py(body)))'
                        ],
                        "name": "alibaba-cloud-block-page",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("252728887" == mmh3(base64_py(body)))'
                        ],
                        "name": "dd-wrt-milli-httpd",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1922044295" == mmh3(base64_py(body)))'
                        ],
                        "name": "mitel-networks-micollab-end-user-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1221759509" == mmh3(base64_py(body)))'
                        ],
                        "name": "dlink-webcam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1037387972" == mmh3(base64_py(body)))'
                        ],
                        "name": "dlink-router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-655683626" == mmh3(base64_py(body)))'
                        ],
                        "name": "prtg-network-monitor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1611729805" == mmh3(base64_py(body)))'
                        ],
                        "name": "elastic-database",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1144925962" == mmh3(base64_py(body)))'
                        ],
                        "name": "dlink-webcam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1666561833" == mmh3(base64_py(body)))'
                        ],
                        "name": "wildfly",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("804949239" == mmh3(base64_py(body)))'
                        ],
                        "name": "cisco-meraki-dashboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-459291760" == mmh3(base64_py(body)))'
                        ],
                        "name": "workday",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1734609466" == mmh3(base64_py(body)))'
                        ],
                        "name": "justhost",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1507567067" == mmh3(base64_py(body)))'
                        ],
                        "name": "baidu-ip-error-page",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2006716043" == mmh3(base64_py(body)))'
                        ],
                        "name": "intelbras-sa",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1298108480" == mmh3(base64_py(body)))'
                        ],
                        "name": "yii-php-framework-default-favicon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1782271534" == mmh3(base64_py(body)))'
                        ],
                        "name": "truvision-nvr-interlogix",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("603314" == mmh3(base64_py(body)))'
                        ],
                        "name": "redmine",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-476231906" == mmh3(base64_py(body)))'
                        ],
                        "name": "phpmyadmin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-646322113" == mmh3(base64_py(body)))'
                        ],
                        "name": "cisco-eg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-629047854" == mmh3(base64_py(body)))'
                        ],
                        "name": "jetty-",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1351901211" == mmh3(base64_py(body)))'
                        ],
                        "name": "luma-surveillance",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-519765377" == mmh3(base64_py(body)))'
                        ],
                        "name": "parallels-plesk-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2144363468" == mmh3(base64_py(body)))'
                        ],
                        "name": "hp-printer-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-127886975" == mmh3(base64_py(body)))'
                        ],
                        "name": "metasploit",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1139788073" == mmh3(base64_py(body)))'
                        ],
                        "name": "metasploit",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1235192469" == mmh3(base64_py(body)))'
                        ],
                        "name": "metasploit",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1876585825" == mmh3(base64_py(body)))'
                        ],
                        "name": "alibi-nvr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1810847295" == mmh3(base64_py(body)))'
                        ],
                        "name": "sangfor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-291579889" == mmh3(base64_py(body)))'
                        ],
                        "name": "websockets-test-page-eg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1629518721" == mmh3(base64_py(body)))'
                        ],
                        "name": "macos-server-apple",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-986816620" == mmh3(base64_py(body)))'
                        ],
                        "name": "openrg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-299287097" == mmh3(base64_py(body)))'
                        ],
                        "name": "cisco-router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1926484046" == mmh3(base64_py(body)))'
                        ],
                        "name": "sangfor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-873627015" == mmh3(base64_py(body)))'
                        ],
                        "name": "herospeed-digital-technology-co",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2071993228" == mmh3(base64_py(body)))'
                        ],
                        "name": "nomadix-access-gateway",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("516963061" == mmh3(base64_py(body)))'
                        ],
                        "name": "gitlab",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-38580010" == mmh3(base64_py(body)))'
                        ],
                        "name": "magento",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1490343308" == mmh3(base64_py(body)))'
                        ],
                        "name": "mk-auth",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-632583950" == mmh3(base64_py(body)))'
                        ],
                        "name": "shoutcast-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("95271369" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireeye",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1476335317" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireeye",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-842192932" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireeye",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("105083909" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireeye",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("240606739" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireeye",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2121539357" == mmh3(base64_py(body)))'
                        ],
                        "name": "fireeye",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-333791179" == mmh3(base64_py(body)))'
                        ],
                        "name": "adobe-campaign-classic",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1437701105" == mmh3(base64_py(body)))'
                        ],
                        "name": "xampp",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-676077969" == mmh3(base64_py(body)))'
                        ],
                        "name": "niagara-web-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2138771289" == mmh3(base64_py(body)))'
                        ],
                        "name": "technicolor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("711742418" == mmh3(base64_py(body)))'
                        ],
                        "name": "hitron-technologies-inc",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("728788645" == mmh3(base64_py(body)))'
                        ],
                        "name": "ibm-notes",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1436966696" == mmh3(base64_py(body)))'
                        ],
                        "name": "barracuda",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("86919334" == mmh3(base64_py(body)))'
                        ],
                        "name": "servicenow",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1211608009" == mmh3(base64_py(body)))'
                        ],
                        "name": "openfire-admin-console",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2059618623" == mmh3(base64_py(body)))'
                        ],
                        "name": "hp-ilo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1975413433" == mmh3(base64_py(body)))'
                        ],
                        "name": "sunny-webbox",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("943925975" == mmh3(base64_py(body)))'
                        ],
                        "name": "zyxel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("281559989" == mmh3(base64_py(body)))'
                        ],
                        "name": "huawei",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2145085239" == mmh3(base64_py(body)))'
                        ],
                        "name": "tenda-web-master",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1399433489" == mmh3(base64_py(body)))'
                        ],
                        "name": "prometheus-time-series-collection-and-processing-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1786752597" == mmh3(base64_py(body)))'
                        ],
                        "name": "wdcp-cloud-host-management-system",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("90680708" == mmh3(base64_py(body)))'
                        ],
                        "name": "domoticz-home-automation",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1441956789" == mmh3(base64_py(body)))'
                        ],
                        "name": "tableau",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-675839242" == mmh3(base64_py(body)))'
                        ],
                        "name": "openwrt-luci",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1020814938" == mmh3(base64_py(body)))'
                        ],
                        "name": "ubiquiti-airos",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-766957661" == mmh3(base64_py(body)))'
                        ],
                        "name": "mdaemon-webmail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("119741608" == mmh3(base64_py(body)))'
                        ],
                        "name": "teltonika",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1973665246" == mmh3(base64_py(body)))'
                        ],
                        "name": "entrolink",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("74935566" == mmh3(base64_py(body)))'
                        ],
                        "name": "windriver-webserver",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1723752240" == mmh3(base64_py(body)))'
                        ],
                        "name": "microhard-systems",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1807411396" == mmh3(base64_py(body)))'
                        ],
                        "name": "skype",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1612496354" == mmh3(base64_py(body)))'
                        ],
                        "name": "teltonika",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1877797890" == mmh3(base64_py(body)))'
                        ],
                        "name": "eltex-router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-375623619" == mmh3(base64_py(body)))'
                        ],
                        "name": "bintec-elmeg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1483097076" == mmh3(base64_py(body)))'
                        ],
                        "name": "syncthru-web-service-printers",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1169183049" == mmh3(base64_py(body)))'
                        ],
                        "name": "boaserver",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1051648103" == mmh3(base64_py(body)))'
                        ],
                        "name": "securepoint",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-438482901" == mmh3(base64_py(body)))'
                        ],
                        "name": "moodle",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1492966240" == mmh3(base64_py(body)))'
                        ],
                        "name": "radix",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1466912879" == mmh3(base64_py(body)))'
                        ],
                        "name": "cradlepoint-technology-router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-167656799" == mmh3(base64_py(body)))'
                        ],
                        "name": "drupal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1593651747" == mmh3(base64_py(body)))'
                        ],
                        "name": "blackboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-895963602" == mmh3(base64_py(body)))'
                        ],
                        "name": "jupyter-notebook",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-972810761" == mmh3(base64_py(body)))'
                        ],
                        "name": "hostmonster-web-hosting",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1703788174" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-router-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("225632504" == mmh3(base64_py(body)))'
                        ],
                        "name": "rocket-chat",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1702393021" == mmh3(base64_py(body)))'
                        ],
                        "name": "mofinetwork",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("892542951" == mmh3(base64_py(body)))'
                        ],
                        "name": "zabbix",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("547474373" == mmh3(base64_py(body)))'
                        ],
                        "name": "totolink-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-374235895" == mmh3(base64_py(body)))'
                        ],
                        "name": "ossia-provision-sr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1544230796" == mmh3(base64_py(body)))'
                        ],
                        "name": "cpanel-login",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("517158172" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-router-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("462223993" == mmh3(base64_py(body)))'
                        ],
                        "name": "jeedom-home-automation",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("937999361" == mmh3(base64_py(body)))'
                        ],
                        "name": "jboss-application-server-7",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1991562061" == mmh3(base64_py(body)))'
                        ],
                        "name": "niagara-web-server-tridium",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("812385209" == mmh3(base64_py(body)))'
                        ],
                        "name": "solarwinds-serv-u-ftp-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1142227528" == mmh3(base64_py(body)))'
                        ],
                        "name": "aruba-virtual-controller",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1153950306" == mmh3(base64_py(body)))'
                        ],
                        "name": "dell",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("72005642" == mmh3(base64_py(body)))'
                        ],
                        "name": "remobjects-sdk-remoting-sdk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-484708885" == mmh3(base64_py(body)))'
                        ],
                        "name": "zyxel-zywall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("706602230" == mmh3(base64_py(body)))'
                        ],
                        "name": "visualsvn-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-656811182" == mmh3(base64_py(body)))'
                        ],
                        "name": "jboss",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-332324409" == mmh3(base64_py(body)))'
                        ],
                        "name": "starface-voip-software",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-594256627" == mmh3(base64_py(body)))'
                        ],
                        "name": "netis-network-devices",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-649378830" == mmh3(base64_py(body)))'
                        ],
                        "name": "whm",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("97604680" == mmh3(base64_py(body)))'
                        ],
                        "name": "tandberg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1015932800" == mmh3(base64_py(body)))'
                        ],
                        "name": "ghost-cms",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-194439630" == mmh3(base64_py(body)))'
                        ],
                        "name": "avtech-ip-surveillance-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("129457226" == mmh3(base64_py(body)))'
                        ],
                        "name": "liferay-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-771764544" == mmh3(base64_py(body)))'
                        ],
                        "name": "parallels-plesk-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-617743584" == mmh3(base64_py(body)))'
                        ],
                        "name": "odoo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("77044418" == mmh3(base64_py(body)))'
                        ],
                        "name": "polycom",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("980692677" == mmh3(base64_py(body)))'
                        ],
                        "name": "cake-php",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("476213314" == mmh3(base64_py(body)))'
                        ],
                        "name": "exacq",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("794809961" == mmh3(base64_py(body)))'
                        ],
                        "name": "checkpoint",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1157789622" == mmh3(base64_py(body)))'
                        ],
                        "name": "ubiquiti-unms",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1244636413" == mmh3(base64_py(body)))'
                        ],
                        "name": "cpanel-login",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1985721423" == mmh3(base64_py(body)))'
                        ],
                        "name": "worldclient-for-mdaemon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1124868062" == mmh3(base64_py(body)))'
                        ],
                        "name": "netport-software-dsl",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-335242539" == mmh3(base64_py(body)))'
                        ],
                        "name": "f5-big-ip",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2146763496" == mmh3(base64_py(body)))'
                        ],
                        "name": "mailcow",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1041180225" == mmh3(base64_py(body)))'
                        ],
                        "name": "qnap-nas-virtualization-station",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1319025408" == mmh3(base64_py(body)))'
                        ],
                        "name": "netgear",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("917966895" == mmh3(base64_py(body)))'
                        ],
                        "name": "gogs",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("512590457" == mmh3(base64_py(body)))'
                        ],
                        "name": "trendnet-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1678170702" == mmh3(base64_py(body)))'
                        ],
                        "name": "asustor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1466785234" == mmh3(base64_py(body)))'
                        ],
                        "name": "dahua",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-505448917" == mmh3(base64_py(body)))'
                        ],
                        "name": "discuz",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("255892555" == mmh3(base64_py(body)))'
                        ],
                        "name": "wdcp-cloud-host-management-system",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1627330242" == mmh3(base64_py(body)))'
                        ],
                        "name": "joomla",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1935525788" == mmh3(base64_py(body)))'
                        ],
                        "name": "smartermail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-12700016" == mmh3(base64_py(body)))'
                        ],
                        "name": "seafile",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1770799630" == mmh3(base64_py(body)))'
                        ],
                        "name": "bintec-elmeg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-137295400" == mmh3(base64_py(body)))'
                        ],
                        "name": "netgear-readynas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-195508437" == mmh3(base64_py(body)))'
                        ],
                        "name": "ipecs",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2116540786" == mmh3(base64_py(body)))'
                        ],
                        "name": "bet365",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-38705358" == mmh3(base64_py(body)))'
                        ],
                        "name": "reolink",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-450254253" == mmh3(base64_py(body)))'
                        ],
                        "name": "idera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1630354993" == mmh3(base64_py(body)))'
                        ],
                        "name": "proofpoint",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1678298769" == mmh3(base64_py(body)))'
                        ],
                        "name": "kerio-connect-webmail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-35107086" == mmh3(base64_py(body)))'
                        ],
                        "name": "worldclient-mdaemon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2055322029" == mmh3(base64_py(body)))'
                        ],
                        "name": "realtek",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-692947551" == mmh3(base64_py(body)))'
                        ],
                        "name": "ruijie-networks-login",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1710631084" == mmh3(base64_py(body)))'
                        ],
                        "name": "askey-cable-modem",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("89321398" == mmh3(base64_py(body)))'
                        ],
                        "name": "askey-cable-modem",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("90066852" == mmh3(base64_py(body)))'
                        ],
                        "name": "jaws-web-server-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("768231242" == mmh3(base64_py(body)))'
                        ],
                        "name": "jaws-web-server-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-421986013" == mmh3(base64_py(body)))'
                        ],
                        "name": "homegrown-website-hosting",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("156312019" == mmh3(base64_py(body)))'
                        ],
                        "name": "technicolor-thomson-speedtouch-network-adsl",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-560297467" == mmh3(base64_py(body)))'
                        ],
                        "name": "dvr-korean",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1950415971" == mmh3(base64_py(body)))'
                        ],
                        "name": "joomla",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1842351293" == mmh3(base64_py(body)))'
                        ],
                        "name": "tp-link-network-device",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1433417005" == mmh3(base64_py(body)))'
                        ],
                        "name": "salesforce",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-632070065" == mmh3(base64_py(body)))'
                        ],
                        "name": "apache-haus",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1103599349" == mmh3(base64_py(body)))'
                        ],
                        "name": "untangle",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("224536051" == mmh3(base64_py(body)))'
                        ],
                        "name": "shenzhen-coship-electronics",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1038500535" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-router-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-355305208" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-267431135" == mmh3(base64_py(body)))'
                        ],
                        "name": "kibana",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-759754862" == mmh3(base64_py(body)))'
                        ],
                        "name": "kibana",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1200737715" == mmh3(base64_py(body)))'
                        ],
                        "name": "kibana",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("75230260" == mmh3(base64_py(body)))'
                        ],
                        "name": "kibana",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1668183286" == mmh3(base64_py(body)))'
                        ],
                        "name": "kibana",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("283740897" == mmh3(base64_py(body)))'
                        ],
                        "name": "intelbras-sa",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1424295654" == mmh3(base64_py(body)))'
                        ],
                        "name": "icecast-streaming-media-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1922032523" == mmh3(base64_py(body)))'
                        ],
                        "name": "nec-webpro",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1654229048" == mmh3(base64_py(body)))'
                        ],
                        "name": "vivotek-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1414475558" == mmh3(base64_py(body)))'
                        ],
                        "name": "microsoft-iis",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1697334194" == mmh3(base64_py(body)))'
                        ],
                        "name": "univention-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1424036600" == mmh3(base64_py(body)))'
                        ],
                        "name": "portainer-docker-management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-831826827" == mmh3(base64_py(body)))'
                        ],
                        "name": "nos-router",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-759108386" == mmh3(base64_py(body)))'
                        ],
                        "name": "tongda",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1022206565" == mmh3(base64_py(body)))'
                        ],
                        "name": "crushftp",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1225484776" == mmh3(base64_py(body)))'
                        ],
                        "name": "endian-firewall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-631002664" == mmh3(base64_py(body)))'
                        ],
                        "name": "kerio-control-firewall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2072198544" == mmh3(base64_py(body)))'
                        ],
                        "name": "ferozo-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-466504476" == mmh3(base64_py(body)))'
                        ],
                        "name": "kerio-control-firewall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1251810433" == mmh3(base64_py(body)))'
                        ],
                        "name": "cafe24-korea",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1273982002" == mmh3(base64_py(body)))'
                        ],
                        "name": "mautic-open-source-marketing-automation",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-978656757" == mmh3(base64_py(body)))'
                        ],
                        "name": "netiaspot-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("916642917" == mmh3(base64_py(body)))'
                        ],
                        "name": "multilaser",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("575613323" == mmh3(base64_py(body)))'
                        ],
                        "name": "canvas-lms-learning-management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1726027799" == mmh3(base64_py(body)))'
                        ],
                        "name": "ibm-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-587741716" == mmh3(base64_py(body)))'
                        ],
                        "name": "adb-broadband-spa-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-360566773" == mmh3(base64_py(body)))'
                        ],
                        "name": "arris-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-884776764" == mmh3(base64_py(body)))'
                        ],
                        "name": "huawei-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("929825723" == mmh3(base64_py(body)))'
                        ],
                        "name": "wampserver",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("240136437" == mmh3(base64_py(body)))'
                        ],
                        "name": "seagate-technology-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1911253822" == mmh3(base64_py(body)))'
                        ],
                        "name": "upc-ceska-republica-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-393788031" == mmh3(base64_py(body)))'
                        ],
                        "name": "flussonic-video-streaming",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("366524387" == mmh3(base64_py(body)))'
                        ],
                        "name": "joomla",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("443944613" == mmh3(base64_py(body)))'
                        ],
                        "name": "wampserver",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1953726032" == mmh3(base64_py(body)))'
                        ],
                        "name": "metabase",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2031183903" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("545827989" == mmh3(base64_py(body)))'
                        ],
                        "name": "mobileiron",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("967636089" == mmh3(base64_py(body)))'
                        ],
                        "name": "mobileiron",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("362091310" == mmh3(base64_py(body)))'
                        ],
                        "name": "mobileiron",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2086228042" == mmh3(base64_py(body)))'
                        ],
                        "name": "mobileiron",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1588746893" == mmh3(base64_py(body)))'
                        ],
                        "name": "communigate",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1427976651" == mmh3(base64_py(body)))'
                        ],
                        "name": "zte-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1648531157" == mmh3(base64_py(body)))'
                        ],
                        "name": "infinet-wireless-wanflex-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("938616453" == mmh3(base64_py(body)))'
                        ],
                        "name": "mersive-solstice",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1632780968" == mmh3(base64_py(body)))'
                        ],
                        "name": "université-toulouse-1-capitole",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2068154487" == mmh3(base64_py(body)))'
                        ],
                        "name": "digium-switchvox",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1788112745" == mmh3(base64_py(body)))'
                        ],
                        "name": "powermta-monitoring",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-644617577" == mmh3(base64_py(body)))'
                        ],
                        "name": "smartlang",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1822098181" == mmh3(base64_py(body)))'
                        ],
                        "name": "checkpoint-gaia",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2127152956" == mmh3(base64_py(body)))'
                        ],
                        "name": "mailwizz",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1064742722" == mmh3(base64_py(body)))'
                        ],
                        "name": "rabbitmq",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-693082538" == mmh3(base64_py(body)))'
                        ],
                        "name": "openmediavault-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1941381095" == mmh3(base64_py(body)))'
                        ],
                        "name": "openwrt-luci",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("903086190" == mmh3(base64_py(body)))'
                        ],
                        "name": "honeywell",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("829321644" == mmh3(base64_py(body)))'
                        ],
                        "name": "bomgar-support-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1442789563" == mmh3(base64_py(body)))'
                        ],
                        "name": "nuxt-js",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2140379067" == mmh3(base64_py(body)))'
                        ],
                        "name": "roundcube-webmail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1897829998" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1047213685" == mmh3(base64_py(body)))'
                        ],
                        "name": "netgear-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1485257654" == mmh3(base64_py(body)))'
                        ],
                        "name": "sonarqube",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-299324825" == mmh3(base64_py(body)))'
                        ],
                        "name": "lupus-electronics-xt",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1162730477" == mmh3(base64_py(body)))'
                        ],
                        "name": "vanderbilt-spc",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1268095485" == mmh3(base64_py(body)))'
                        ],
                        "name": "vzpp-plesk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1118684072" == mmh3(base64_py(body)))'
                        ],
                        "name": "baidu",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1616115760" == mmh3(base64_py(body)))'
                        ],
                        "name": "owncloud",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2054889066" == mmh3(base64_py(body)))'
                        ],
                        "name": "sentora",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1333537166" == mmh3(base64_py(body)))'
                        ],
                        "name": "alfresco",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-373674173" == mmh3(base64_py(body)))'
                        ],
                        "name": "digital-keystone-dk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-106646451" == mmh3(base64_py(body)))'
                        ],
                        "name": "wispr-airlan",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1235070469" == mmh3(base64_py(body)))'
                        ],
                        "name": "synology-vpn-plus",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2063428236" == mmh3(base64_py(body)))'
                        ],
                        "name": "sentry",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("15831193" == mmh3(base64_py(body)))'
                        ],
                        "name": "watchguard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-956471263" == mmh3(base64_py(body)))'
                        ],
                        "name": "web-client-pro",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1452159623" == mmh3(base64_py(body)))'
                        ],
                        "name": "tecvoz",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("99432374" == mmh3(base64_py(body)))'
                        ],
                        "name": "mdaemon-remote-administration",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("727253975" == mmh3(base64_py(body)))'
                        ],
                        "name": "paradox-ip-module",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-630493013" == mmh3(base64_py(body)))'
                        ],
                        "name": "dokuwiki",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("552597979" == mmh3(base64_py(body)))'
                        ],
                        "name": "sails",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("774252049" == mmh3(base64_py(body)))'
                        ],
                        "name": "fastpanel-hosting",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-329747115" == mmh3(base64_py(body)))'
                        ],
                        "name": "c-lodop",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1262005940" == mmh3(base64_py(body)))'
                        ],
                        "name": "jamf-pro-login",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("979634648" == mmh3(base64_py(body)))'
                        ],
                        "name": "struxureware-schneider-electric",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("475379699" == mmh3(base64_py(body)))'
                        ],
                        "name": "axcient-replibit-management-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-878891718" == mmh3(base64_py(body)))'
                        ],
                        "name": "twonky-server-media-streaming",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2125083197" == mmh3(base64_py(body)))'
                        ],
                        "name": "windows-azure",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1151675028" == mmh3(base64_py(body)))'
                        ],
                        "name": "isp-manager-web-hosting-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1248917303" == mmh3(base64_py(body)))'
                        ],
                        "name": "jupyterhub",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1908556829" == mmh3(base64_py(body)))'
                        ],
                        "name": "centurylink-modem-gui-login-eg",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1059329877" == mmh3(base64_py(body)))'
                        ],
                        "name": "tecvoz",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1148190371" == mmh3(base64_py(body)))'
                        ],
                        "name": "opnsense",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1467395679" == mmh3(base64_py(body)))'
                        ],
                        "name": "ligowave-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1528414776" == mmh3(base64_py(body)))'
                        ],
                        "name": "rumpus",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2117390767" == mmh3(base64_py(body)))'
                        ],
                        "name": "spiceworks-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1944119648" == mmh3(base64_py(body)))'
                        ],
                        "name": "teamcity",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1748763891" == mmh3(base64_py(body)))'
                        ],
                        "name": "instar-full-hd-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("251106693" == mmh3(base64_py(body)))'
                        ],
                        "name": "gpon-home-gateway",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1779611449" == mmh3(base64_py(body)))'
                        ],
                        "name": "alienvault",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1745552996" == mmh3(base64_py(body)))'
                        ],
                        "name": "arbor-networks",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1275148624" == mmh3(base64_py(body)))'
                        ],
                        "name": "accrisoft",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-178685903" == mmh3(base64_py(body)))'
                        ],
                        "name": "yasni",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-43161126" == mmh3(base64_py(body)))'
                        ],
                        "name": "slack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("671221099" == mmh3(base64_py(body)))'
                        ],
                        "name": "innovaphone",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-10974981" == mmh3(base64_py(body)))'
                        ],
                        "name": "shinobi-cctv",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1274078387" == mmh3(base64_py(body)))'
                        ],
                        "name": "tp-link-network-device",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-336242473" == mmh3(base64_py(body)))'
                        ],
                        "name": "siemens-ozw772",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("882208493" == mmh3(base64_py(body)))'
                        ],
                        "name": "lantronix-spider",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-687783882" == mmh3(base64_py(body)))'
                        ],
                        "name": "claimtime-ramsell-public-health-safety",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-805544463" == mmh3(base64_py(body)))'
                        ],
                        "name": "clawdbot-control-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-590892202" == mmh3(base64_py(body)))'
                        ],
                        "name": "surfilter-ssl-vpn-portal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-50306417" == mmh3(base64_py(body)))'
                        ],
                        "name": "kyocera-printer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("784872924" == mmh3(base64_py(body)))'
                        ],
                        "name": "lucee",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1135165421" == mmh3(base64_py(body)))'
                        ],
                        "name": "ricoh",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("926501571" == mmh3(base64_py(body)))'
                        ],
                        "name": "handle-proxy",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("579239725" == mmh3(base64_py(body)))'
                        ],
                        "name": "metasploit",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-689902428" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-600508822" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("656868270" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2056503929" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1656695885" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("331870709" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1241049726" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("998138196" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("322531336" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-401934945" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-613216179" == mmh3(base64_py(body)))'
                        ],
                        "name": "iomega-nas",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-276759139" == mmh3(base64_py(body)))'
                        ],
                        "name": "chef-automate",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1862132268" == mmh3(base64_py(body)))'
                        ],
                        "name": "gargoyle-router-management-utility",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1738727418" == mmh3(base64_py(body)))'
                        ],
                        "name": "keepitsafe-management-console",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-368490461" == mmh3(base64_py(body)))'
                        ],
                        "name": "entronix-energy-management-platform",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1836828108" == mmh3(base64_py(body)))'
                        ],
                        "name": "openproject",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1775553655" == mmh3(base64_py(body)))'
                        ],
                        "name": "unified-management-console-polycom",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("381100274" == mmh3(base64_py(body)))'
                        ],
                        "name": "moxapass-iologik-remote-ethernet-io-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2124459909" == mmh3(base64_py(body)))'
                        ],
                        "name": "hfs-http-file-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("731374291" == mmh3(base64_py(body)))'
                        ],
                        "name": "hfs-http-file-server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-335153896" == mmh3(base64_py(body)))'
                        ],
                        "name": "traccar-gps-tracking",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("896412703" == mmh3(base64_py(body)))'
                        ],
                        "name": "iw",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("191654058" == mmh3(base64_py(body)))'
                        ],
                        "name": "wordpress-under-construction-icon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-342262483" == mmh3(base64_py(body)))'
                        ],
                        "name": "combivox",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("5542029" == mmh3(base64_py(body)))'
                        ],
                        "name": "netcomwireless-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1552860581" == mmh3(base64_py(body)))'
                        ],
                        "name": "elastic-database",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1174841451" == mmh3(base64_py(body)))'
                        ],
                        "name": "drupal",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1093172228" == mmh3(base64_py(body)))'
                        ],
                        "name": "truvision-nvr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1688698891" == mmh3(base64_py(body)))'
                        ],
                        "name": "spamexperts",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1546574541" == mmh3(base64_py(body)))'
                        ],
                        "name": "sonatype-nexus-repository-manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-256828986" == mmh3(base64_py(body)))'
                        ],
                        "name": "idirect-canada-network-management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1966198264" == mmh3(base64_py(body)))'
                        ],
                        "name": "openerp-now-known-as-odoo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2099342476" == mmh3(base64_py(body)))'
                        ],
                        "name": "pkp-openjournalsystems-public-knowledge-project",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("541087742" == mmh3(base64_py(body)))'
                        ],
                        "name": "liquidfiles",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1906448386" == mmh3(base64_py(body)))'
                        ],
                        "name": "liquidfiles",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-882760066" == mmh3(base64_py(body)))'
                        ],
                        "name": "zyxel-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("16202868" == mmh3(base64_py(body)))'
                        ],
                        "name": "universal-devices-ud",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("987967490" == mmh3(base64_py(body)))'
                        ],
                        "name": "huawei-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-647318973" == mmh3(base64_py(body)))'
                        ],
                        "name": "gm77",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1583478052" == mmh3(base64_py(body)))'
                        ],
                        "name": "okazik",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1969970750" == mmh3(base64_py(body)))'
                        ],
                        "name": "gitea",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1734573358" == mmh3(base64_py(body)))'
                        ],
                        "name": "tc-group",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1589842876" == mmh3(base64_py(body)))'
                        ],
                        "name": "deluge-web-ui",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1822002133" == mmh3(base64_py(body)))'
                        ],
                        "name": "登录-amh",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2006308185" == mmh3(base64_py(body)))'
                        ],
                        "name": "otrs-open-ticket-request-system",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1702769256" == mmh3(base64_py(body)))'
                        ],
                        "name": "bosch-security-systems-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("321591353" == mmh3(base64_py(body)))'
                        ],
                        "name": "node-red",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-923693877" == mmh3(base64_py(body)))'
                        ],
                        "name": "motioneye-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1547576879" == mmh3(base64_py(body)))'
                        ],
                        "name": "saia-burgess-controls-pcd",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1479202414" == mmh3(base64_py(body)))'
                        ],
                        "name": "arcadyan-o2-box-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1081719753" == mmh3(base64_py(body)))'
                        ],
                        "name": "d-link-network",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-166151761" == mmh3(base64_py(body)))'
                        ],
                        "name": "abilis-network-automation",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1231681737" == mmh3(base64_py(body)))'
                        ],
                        "name": "ghost-cms",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("321909464" == mmh3(base64_py(body)))'
                        ],
                        "name": "airwatch",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1153873472" == mmh3(base64_py(body)))'
                        ],
                        "name": "airwatch",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1095915848" == mmh3(base64_py(body)))'
                        ],
                        "name": "airwatch",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("788771792" == mmh3(base64_py(body)))'
                        ],
                        "name": "airwatch",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1863663974" == mmh3(base64_py(body)))'
                        ],
                        "name": "airwatch",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1267819858" == mmh3(base64_py(body)))'
                        ],
                        "name": "keyhelp-keyweb-ag",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("726817668" == mmh3(base64_py(body)))'
                        ],
                        "name": "keyhelp-keyweb-ag",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1474875778" == mmh3(base64_py(body)))'
                        ],
                        "name": "glpi",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("5471989" == mmh3(base64_py(body)))'
                        ],
                        "name": "netcom-technology",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1457536113" == mmh3(base64_py(body)))'
                        ],
                        "name": "cradlepoint",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-736276076" == mmh3(base64_py(body)))'
                        ],
                        "name": "myasp",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1343070146" == mmh3(base64_py(body)))'
                        ],
                        "name": "intelbras-sa",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("538585915" == mmh3(base64_py(body)))'
                        ],
                        "name": "lenel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-625364318" == mmh3(base64_py(body)))'
                        ],
                        "name": "okofen-pellematic",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1117165781" == mmh3(base64_py(body)))'
                        ],
                        "name": "simplehelp-remote-support",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1067420240" == mmh3(base64_py(body)))'
                        ],
                        "name": "graphql",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1465479343" == mmh3(base64_py(body)))'
                        ],
                        "name": "dnn-cms",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1232159009" == mmh3(base64_py(body)))'
                        ],
                        "name": "apple",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1382324298" == mmh3(base64_py(body)))'
                        ],
                        "name": "apple",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1498185948" == mmh3(base64_py(body)))'
                        ],
                        "name": "apple",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("483383992" == mmh3(base64_py(body)))'
                        ],
                        "name": "ispconfig",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1249852061" == mmh3(base64_py(body)))'
                        ],
                        "name": "microsoft-outlook",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("999357577" == mmh3(base64_py(body)))'
                        ],
                        "name": "possibly-dvr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("492290497" == mmh3(base64_py(body)))'
                        ],
                        "name": "possible-ip-camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("400100893" == mmh3(base64_py(body)))'
                        ],
                        "name": "dvr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("983384876" == mmh3(base64_py(body)))'
                        ],
                        "name": "One Identity Password Manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1445519482" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kaseya",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1167011145" == mmh3(base64_py(body)))'
                        ],
                        "name": "OpenSNS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1000719429" == mmh3(base64_py(body)))'
                        ],
                        "name": "SpamSniper",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-41369781" == mmh3(base64_py(body)))'
                        ],
                        "name": "Selea-ANPR-Camera",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-656528622" == mmh3(base64_py(body)))'
                        ],
                        "name": "Selea-CarPlateServer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1650202746" == mmh3(base64_py(body)))'
                        ],
                        "name": "KevinLAB",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-521603470" == mmh3(base64_py(body)))'
                        ],
                        "name": "cisco-ucs",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1250474341" == mmh3(base64_py(body)))'
                        ],
                        "name": "vmware-Workspace-ONE-Access",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("331817455" == mmh3(base64_py(body)))'
                        ],
                        "name": "mspcontrol",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1698328762" == mmh3(base64_py(body)))'
                        ],
                        "name": "Ionic",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1238669242" == mmh3(base64_py(body)))'
                        ],
                        "name": "Tufin-SecureTrack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1398055326" == mmh3(base64_py(body)))'
                        ],
                        "name": "WSO2-api-manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2015389528" == mmh3(base64_py(body)))'
                        ],
                        "name": "Synapse-Mobility",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1614287628" == mmh3(base64_py(body)))'
                        ],
                        "name": "Gradle-enterprise",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2130463260" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kubernetes-Operational-View",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("464587962" == mmh3(base64_py(body)))'
                        ],
                        "name": "Rancher",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("902188936" == mmh3(base64_py(body)))'
                        ],
                        "name": "Emby",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1624375939" == mmh3(base64_py(body)))'
                        ],
                        "name": "Zimbra",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1841044852" == mmh3(base64_py(body)))'
                        ],
                        "name": "ICT Protege WX",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-994319624" == mmh3(base64_py(body)))'
                        ],
                        "name": "Coverity",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1261322577" == mmh3(base64_py(body)))'
                        ],
                        "name": "Aqua Enterprise",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1054477011" == mmh3(base64_py(body)))'
                        ],
                        "name": "Leica RefWorx",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2024949122" == mmh3(base64_py(body)))'
                        ],
                        "name": "Liferay",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1484947000" == mmh3(base64_py(body)))'
                        ],
                        "name": "GoAnyWhere",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1252041730" == mmh3(base64_py(body)))'
                        ],
                        "name": "Vue",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-830586692" == mmh3(base64_py(body)))'
                        ],
                        "name": "Sophos Email Appliance",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("557327884" == mmh3(base64_py(body)))'
                        ],
                        "name": "Temporal Web UI",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("116323821" == mmh3(base64_py(body)))'
                        ],
                        "name": "Springboot Actuators",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("431627549" == mmh3(base64_py(body)))'
                        ],
                        "name": "Snipe-IT",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1893514038" == mmh3(base64_py(body)))'
                        ],
                        "name": "RSSHub",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1101206929" == mmh3(base64_py(body)))'
                        ],
                        "name": "SECUI WAF",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1261322577" == mmh3(base64_py(body)))'
                        ],
                        "name": "Aqua Enterprise",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1379982221" == mmh3(base64_py(body)))'
                        ],
                        "name": "Bamboo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1383463717" == mmh3(base64_py(body)))'
                        ],
                        "name": "Iparapheur",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("419828698" == mmh3(base64_py(body)))'
                        ],
                        "name": "MonstraCMS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("635899646" == mmh3(base64_py(body)))'
                        ],
                        "name": "Silverback MDM",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("999357577" == mmh3(base64_py(body)))'
                        ],
                        "name": "Hikvision",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1781653957" == mmh3(base64_py(body)))'
                        ],
                        "name": "LimeSurvey Survey Software",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("81586312" == mmh3(base64_py(body)))'
                        ],
                        "name": "Jenkins",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1529860313" == mmh3(base64_py(body)))'
                        ],
                        "name": "OSQuery Fleet",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("407286339" == mmh3(base64_py(body)))'
                        ],
                        "name": "OpenTSDB",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-234335289" == mmh3(base64_py(body)))'
                        ],
                        "name": "WebMethod Integration Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1199592666" == mmh3(base64_py(body)))'
                        ],
                        "name": "Structurizr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-670975485" == mmh3(base64_py(body)))'
                        ],
                        "name": "EmqDefault",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("321591353" == mmh3(base64_py(body)))'
                        ],
                        "name": "Node-Red",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2134367771" == mmh3(base64_py(body)))'
                        ],
                        "name": "TimeKeeper",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("11794165" == mmh3(base64_py(body)))'
                        ],
                        "name": "Apollo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1691956220" == mmh3(base64_py(body)))'
                        ],
                        "name": "XXL-JOB",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-655683626" == mmh3(base64_py(body)))'
                        ],
                        "name": "PRTG Network Monitor",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-977323269" == mmh3(base64_py(body)))'
                        ],
                        "name": "Everything Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("450899026" == mmh3(base64_py(body)))'
                        ],
                        "name": "Laravel Horizon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("557327884" == mmh3(base64_py(body)))'
                        ],
                        "name": "Temporal Web UI",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1052926265" == mmh3(base64_py(body)))'
                        ],
                        "name": "File Browser",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("969374472" == mmh3(base64_py(body)))'
                        ],
                        "name": "RethinkDB Administration Console",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("24048806" == mmh3(base64_py(body)))'
                        ],
                        "name": "Doris Dashboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("587330928" == mmh3(base64_py(body)))'
                        ],
                        "name": "Apache Impala",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("786533217" == mmh3(base64_py(body)))'
                        ],
                        "name": "Openstack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("450899026" == mmh3(base64_py(body)))'
                        ],
                        "name": "Jupyter Lab",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1117549627" == mmh3(base64_py(body)))'
                        ],
                        "name": "Fastvue",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1478287554" == mmh3(base64_py(body)))'
                        ],
                        "name": "TeslaMate",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("932345713" == mmh3(base64_py(body)))'
                        ],
                        "name": "Prometheus Promtail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-374133142" == mmh3(base64_py(body)))'
                        ],
                        "name": "Celery Flower",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1373456171" == mmh3(base64_py(body)))'
                        ],
                        "name": "Setup GitHub Enterprise",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1550906681" == mmh3(base64_py(body)))'
                        ],
                        "name": "OpenBMCS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1550906681" == mmh3(base64_py(body)))'
                        ],
                        "name": "OpenBMCS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-347188002" == mmh3(base64_py(body)))'
                        ],
                        "name": "Acuneti360",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2099342476" == mmh3(base64_py(body)))'
                        ],
                        "name": "Open Journal Systems",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("538583492" == mmh3(base64_py(body)))'
                        ],
                        "name": "ClipBucket",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2115208104" == mmh3(base64_py(body)))'
                        ],
                        "name": "Bitrix24",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1575154882" == mmh3(base64_py(body)))'
                        ],
                        "name": "Netsparker Enterprise",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-50306417" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kyocera Printer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1595726841" == mmh3(base64_py(body)))'
                        ],
                        "name": "AceNet AceReporter Report",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1298131932" == mmh3(base64_py(body)))'
                        ],
                        "name": "jshERP",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("965982073" == mmh3(base64_py(body)))'
                        ],
                        "name": "IBM OpenAdmin Tool",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-81573405" == mmh3(base64_py(body)))'
                        ],
                        "name": "iSAMS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-749942143" == mmh3(base64_py(body)))'
                        ],
                        "name": "Sensu by Sumo Logic",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1398055326" == mmh3(base64_py(body)))'
                        ],
                        "name": "WSO2 Management Console",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-850502287" == mmh3(base64_py(body)))'
                        ],
                        "name": "Buddy",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("824580113" == mmh3(base64_py(body)))'
                        ],
                        "name": "T-Up OpenFrame",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1653394551" == mmh3(base64_py(body)))'
                        ],
                        "name": "Dahua Web Service",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-741491222" == mmh3(base64_py(body)))'
                        ],
                        "name": "Horde",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-919788577" == mmh3(base64_py(body)))'
                        ],
                        "name": "Vault",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("464587962" == mmh3(base64_py(body)))'
                        ],
                        "name": "Rancher",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("816588900" == mmh3(base64_py(body)))'
                        ],
                        "name": "ShardingSphere ElasticJob UI",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1264095219" == mmh3(base64_py(body)))'
                        ],
                        "name": "OpenBullet",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1298131932" == mmh3(base64_py(body)))'
                        ],
                        "name": "JshERP Boot",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1262005940" == mmh3(base64_py(body)))'
                        ],
                        "name": "Jamf MDM",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1595726841" == mmh3(base64_py(body)))'
                        ],
                        "name": "AceNet AceReporter Report",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2104916232" == mmh3(base64_py(body)))'
                        ],
                        "name": "Horde Webmail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("540706145" == mmh3(base64_py(body)))'
                        ],
                        "name": "Piwigo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1198579728" == mmh3(base64_py(body)))'
                        ],
                        "name": "DbGate Web Client Management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("305412257" == mmh3(base64_py(body)))'
                        ],
                        "name": "Eventum",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1354079303" == mmh3(base64_py(body)))'
                        ],
                        "name": "Drone CI",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-893681401" == mmh3(base64_py(body)))'
                        ],
                        "name": "Sophos Web Appliance",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("942678640" == mmh3(base64_py(body)))'
                        ],
                        "name": "Proofpoint Protection Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-633108100" == mmh3(base64_py(body)))'
                        ],
                        "name": "Appwrite",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-417785140" == mmh3(base64_py(body)))'
                        ],
                        "name": "Homer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("983734701" == mmh3(base64_py(body)))'
                        ],
                        "name": "Avatier Password Management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1127895693" == mmh3(base64_py(body)))'
                        ],
                        "name": "Zuul",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("957255151" == mmh3(base64_py(body)))'
                        ],
                        "name": "SAS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1199592666" == mmh3(base64_py(body)))'
                        ],
                        "name": "Structurizr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("176427349" == mmh3(base64_py(body)))'
                        ],
                        "name": "UniboPanel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1768726119" == mmh3(base64_py(body)))'
                        ],
                        "name": "Microsoft Exchange Web Service",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("698624197" == mmh3(base64_py(body)))'
                        ],
                        "name": "Redash",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1324930554" == mmh3(base64_py(body)))'
                        ],
                        "name": "Rancher Dashboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("967636089" == mmh3(base64_py(body)))'
                        ],
                        "name": "MobileIron Sentry",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1105083093" == mmh3(base64_py(body)))'
                        ],
                        "name": "Keycloak Admin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("213144638" == mmh3(base64_py(body)))'
                        ],
                        "name": "ProxmoVirtual Environment",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2017604252" == mmh3(base64_py(body)))'
                        ],
                        "name": "Jellyseerr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1249285083" == mmh3(base64_py(body)))'
                        ],
                        "name": "airCube",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("115295460" == mmh3(base64_py(body)))'
                        ],
                        "name": "Strider CD",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("151132309" == mmh3(base64_py(body)))'
                        ],
                        "name": "CloudPanel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2144699833" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kasm",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-633512412" == mmh3(base64_py(body)))'
                        ],
                        "name": "Veeam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1013024216" == mmh3(base64_py(body)))'
                        ],
                        "name": "Dashy",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1889244460" == mmh3(base64_py(body)))'
                        ],
                        "name": "ArcServe",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("933976300" == mmh3(base64_py(body)))'
                        ],
                        "name": "Pichome",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1215318992" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kiteworks PCN",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1691956220" == mmh3(base64_py(body)))'
                        ],
                        "name": "XXLJOB Admin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1970367401" == mmh3(base64_py(body)))'
                        ],
                        "name": "Vidyo Admin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1552322396" == mmh3(base64_py(body)))'
                        ],
                        "name": "Seafile",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2056442365" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kanboard",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("876876147" == mmh3(base64_py(body)))'
                        ],
                        "name": "Virtua Software",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1011076161" == mmh3(base64_py(body)))'
                        ],
                        "name": "ASPECT Control",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("598296063" == mmh3(base64_py(body)))'
                        ],
                        "name": "Navicat On-Prem Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1898583197" == mmh3(base64_py(body)))'
                        ],
                        "name": "OpenVZ Web",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-800060828" == mmh3(base64_py(body)))'
                        ],
                        "name": "OfficeKeeper Admin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1949005079" == mmh3(base64_py(body)))'
                        ],
                        "name": "Modoboa",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("419828698" == mmh3(base64_py(body)))'
                        ],
                        "name": "Monstra Admin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-839356603" == mmh3(base64_py(body)))'
                        ],
                        "name": "Apigee",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-43504595" == mmh3(base64_py(body)))'
                        ],
                        "name": "Immich",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("662709064" == mmh3(base64_py(body)))'
                        ],
                        "name": "MantisBT",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1074357885" == mmh3(base64_py(body)))'
                        ],
                        "name": "WD My Cloud",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1701804003" == mmh3(base64_py(body)))'
                        ],
                        "name": "ServiceNow",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1706783005" == mmh3(base64_py(body)))'
                        ],
                        "name": "Atlantis",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-578216669" == mmh3(base64_py(body)))'
                        ],
                        "name": "RaspberryMatic",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1693580324" == mmh3(base64_py(body)))'
                        ],
                        "name": "Eagle For Apache Kakfa",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("129457226" == mmh3(base64_py(body)))'
                        ],
                        "name": "Liferay",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("889652940" == mmh3(base64_py(body)))'
                        ],
                        "name": "Archibus Web Central",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-629968763" == mmh3(base64_py(body)))'
                        ],
                        "name": "Shell In A Bo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1817615343" == mmh3(base64_py(body)))'
                        ],
                        "name": "Regify",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1416464161" == mmh3(base64_py(body)))'
                        ],
                        "name": "SolarWinds ARM",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1828614783" == mmh3(base64_py(body)))'
                        ],
                        "name": "Dynatrace",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("781922099" == mmh3(base64_py(body)))'
                        ],
                        "name": "Sunbird DCIM",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-379154636" == mmh3(base64_py(body)))'
                        ],
                        "name": "KubeView",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1604363273" == mmh3(base64_py(body)))'
                        ],
                        "name": "HiveManager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1663319756" == mmh3(base64_py(body)))'
                        ],
                        "name": "SAUTER moduWeb Vision",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1090061843" == mmh3(base64_py(body)))'
                        ],
                        "name": "WebTitan Cloud",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("739801466" == mmh3(base64_py(body)))'
                        ],
                        "name": "Cofense Vision",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("24048806" == mmh3(base64_py(body)))'
                        ],
                        "name": "Doris",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1052926265" == mmh3(base64_py(body)))'
                        ],
                        "name": "File Browser",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1343712810" == mmh3(base64_py(body)))'
                        ],
                        "name": "Joget",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-399298961" == mmh3(base64_py(body)))'
                        ],
                        "name": "IBM Maximo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1337147129" == mmh3(base64_py(body)))'
                        ],
                        "name": "IBM WebSphere Application",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-440644339" == mmh3(base64_py(body)))'
                        ],
                        "name": "Zyxel Firewall",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("945408572" == mmh3(base64_py(body)))'
                        ],
                        "name": "Fortinet FortiOS Management Interface",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("945408572" == mmh3(base64_py(body)))'
                        ],
                        "name": "FortiOS Admin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1961736892" == mmh3(base64_py(body)))'
                        ],
                        "name": "DzzOffice",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1961736892" == mmh3(base64_py(body)))'
                        ],
                        "name": "DzzOffice",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1198579728" == mmh3(base64_py(body)))'
                        ],
                        "name": "DbGate Web Client",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("116323821" == mmh3(base64_py(body)))'
                        ],
                        "name": "Spring Boot H2 Database",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-902890504" == mmh3(base64_py(body)))'
                        ],
                        "name": "Universal Media Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1548359600" == mmh3(base64_py(body)))'
                        ],
                        "name": "SOUND4 IMPACTFIRST PULSE Eco",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1950415971" == mmh3(base64_py(body)))'
                        ],
                        "name": "Joomla JLeReview",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2032163853" == mmh3(base64_py(body)))'
                        ],
                        "name": "Jorani Benjamin BALET",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2124459909" == mmh3(base64_py(body)))'
                        ],
                        "name": "HTTP File Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2073748627" == mmh3(base64_py(body)))'
                        ],
                        "name": "MSNSwitch Firmware MNT",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1521640213" == mmh3(base64_py(body)))'
                        ],
                        "name": "HotelDruid Hotel Management Software",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-582931176" == mmh3(base64_py(body)))'
                        ],
                        "name": "NexusPHP",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1274798165" == mmh3(base64_py(body)))'
                        ],
                        "name": "Sophos Mobile managed on-premises",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1398055326" == mmh3(base64_py(body)))'
                        ],
                        "name": "WSO2 Management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1474875778" == mmh3(base64_py(body)))'
                        ],
                        "name": "GLPI",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("217119619" == mmh3(base64_py(body)))'
                        ],
                        "name": "RStudio Connect",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1410071322" == mmh3(base64_py(body)))'
                        ],
                        "name": "SmarterTools SmarterTrack",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1464851260" == mmh3(base64_py(body)))'
                        ],
                        "name": "MCMS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("816588900" == mmh3(base64_py(body)))'
                        ],
                        "name": "Apache ShardingSphere ElasticJob-UI",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-244067125" == mmh3(base64_py(body)))'
                        ],
                        "name": "SolarView Compact",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1499940355" == mmh3(base64_py(body)))'
                        ],
                        "name": "Rukovoditel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1465760059" == mmh3(base64_py(body)))'
                        ],
                        "name": "RaspAP",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1250474341" == mmh3(base64_py(body)))'
                        ],
                        "name": "VMware Workspace ONE Access",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1749354953" == mmh3(base64_py(body)))'
                        ],
                        "name": "Hitachi Pentaho Business Analytics Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1624375939" == mmh3(base64_py(body)))'
                        ],
                        "name": "Zimbra Collaboration Suite",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1250474341" == mmh3(base64_py(body)))'
                        ],
                        "name": "VMware",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1350437236" == mmh3(base64_py(body)))'
                        ],
                        "name": "Wavlink",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("874152924" == mmh3(base64_py(body)))'
                        ],
                        "name": "Likeshop",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1170495932" == mmh3(base64_py(body)))'
                        ],
                        "name": "Fortra GoAnywhere MFT",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1828756398" == mmh3(base64_py(body)))'
                        ],
                        "name": "GoAnywhere",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1484947000" == mmh3(base64_py(body)))'
                        ],
                        "name": "GoAnywhere Web Client",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-74348711" == mmh3(base64_py(body)))'
                        ],
                        "name": "Qlik Sense Enterprise",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2097033750" == mmh3(base64_py(body)))'
                        ],
                        "name": "VertaAI ModelDB",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-165631681" == mmh3(base64_py(body)))'
                        ],
                        "name": "Gibbon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1099097618" == mmh3(base64_py(body)))'
                        ],
                        "name": "Squide",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1247684400" == mmh3(base64_py(body)))'
                        ],
                        "name": "Axigen WebMail",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("362091310" == mmh3(base64_py(body)))'
                        ],
                        "name": "Ivanti Endpoint Manager Mobile (EPMM)",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("440258421" == mmh3(base64_py(body)))'
                        ],
                        "name": "Dolibarr",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("780351152" == mmh3(base64_py(body)))'
                        ],
                        "name": "Microweber",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("163538942" == mmh3(base64_py(body)))'
                        ],
                        "name": "CData RSB Connect",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-50306417" == mmh3(base64_py(body)))'
                        ],
                        "name": "Kyocera TASKalfa printer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-165631681" == mmh3(base64_py(body)))'
                        ],
                        "name": "Gibbon",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1469328760" == mmh3(base64_py(body)))'
                        ],
                        "name": "PMB",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2032163853" == mmh3(base64_py(body)))'
                        ],
                        "name": "Jorani",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2017596142" == mmh3(base64_py(body)))'
                        ],
                        "name": "NocoDB version",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1317621215" == mmh3(base64_py(body)))'
                        ],
                        "name": "D-Link D-View",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1380908726" == mmh3(base64_py(body)))'
                        ],
                        "name": "JeecgBoot",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-47932290" == mmh3(base64_py(body)))'
                        ],
                        "name": "CraftCMS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1199592666" == mmh3(base64_py(body)))'
                        ],
                        "name": "Structurizr on-premises",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1649949475" == mmh3(base64_py(body)))'
                        ],
                        "name": "Label Studio",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1484947000" == mmh3(base64_py(body)))'
                        ],
                        "name": "Fortra GoAnywhere MFT",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1680052984" == mmh3(base64_py(body)))'
                        ],
                        "name": "Web2py URL",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("989289239" == mmh3(base64_py(body)))'
                        ],
                        "name": "MOVEit Transfer",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1540720428" == mmh3(base64_py(body)))'
                        ],
                        "name": "SysAid Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-476299640" == mmh3(base64_py(body)))'
                        ],
                        "name": "Hestiacp",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("463802404" == mmh3(base64_py(body)))'
                        ],
                        "name": "Anyscale Ray",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1903390397" == mmh3(base64_py(body)))'
                        ],
                        "name": "MajorDoMo",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("475145467" == mmh3(base64_py(body)))'
                        ],
                        "name": "Zimbra Collaboration Suite",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1381126564" == mmh3(base64_py(body)))'
                        ],
                        "name": "SonicWall GMS and Analytics Web Services",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2134367771" == mmh3(base64_py(body)))'
                        ],
                        "name": "TimeKeeper by FSMLabs",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2144485375" == mmh3(base64_py(body)))'
                        ],
                        "name": "IceWarp Webmail Server",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("362091310" == mmh3(base64_py(body)))'
                        ],
                        "name": "MobileIron Core",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1189292869" == mmh3(base64_py(body)))'
                        ],
                        "name": "Flatpress",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("463802404" == mmh3(base64_py(body)))'
                        ],
                        "name": "Ray Static File",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1540720428" == mmh3(base64_py(body)))'
                        ],
                        "name": "SysAid Help Desk",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2019488876" == mmh3(base64_py(body)))'
                        ],
                        "name": "Dahua Security",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1414548363" == mmh3(base64_py(body)))'
                        ],
                        "name": "ThinVNC",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("762074255" == mmh3(base64_py(body)))'
                        ],
                        "name": "qdPM",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("876876147" == mmh3(base64_py(body)))'
                        ],
                        "name": "Virtua Software Cobranca",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1606065523" == mmh3(base64_py(body)))'
                        ],
                        "name": "Cachet",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2098066288" == mmh3(base64_py(body)))'
                        ],
                        "name": "GenieACS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("698624197" == mmh3(base64_py(body)))'
                        ],
                        "name": "Redash Setup Configuration",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1582430156" == mmh3(base64_py(body)))'
                        ],
                        "name": "Apache Superset",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("106844876" == mmh3(base64_py(body)))'
                        ],
                        "name": "Revive Adserver",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("999357577" == mmh3(base64_py(body)))'
                        ],
                        "name": "Hikvision IP camera NVR",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("688609340" == mmh3(base64_py(body)))'
                        ],
                        "name": "Agentejo Cockpit",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-186961397" == mmh3(base64_py(body)))'
                        ],
                        "name": "Apache Kylin",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-399298961" == mmh3(base64_py(body)))'
                        ],
                        "name": "IBM Maximo Asset Management",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("419828698" == mmh3(base64_py(body)))'
                        ],
                        "name": "Monstra CMS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1074357885" == mmh3(base64_py(body)))'
                        ],
                        "name": "Western Digital MyCloud NAS",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1599943282" == mmh3(base64_py(body)))'
                        ],
                        "name": "FileCatalyst",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1728967963" == mmh3(base64_py(body)))'
                        ],
                        "name": "Veeam Service Provider Console",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("702863115" == mmh3(base64_py(body)))'
                        ],
                        "name": "mooSocial",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-831756631" == mmh3(base64_py(body)))'
                        ],
                        "name": "n8n",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-130447705" == mmh3(base64_py(body)))'
                        ],
                        "name": "umami",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("307296554" == mmh3(base64_py(body)))'
                        ],
                        "name": "snowflake",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("697691819" == mmh3(base64_py(body)))'
                        ],
                        "name": "yardi-voyager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("2019488876" == mmh3(base64_py(body)))'
                        ],
                        "name": "ic-realtime",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1653394551" == mmh3(base64_py(body)))'
                        ],
                        "name": "lorex",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1862800928" == mmh3(base64_py(body)))'
                        ],
                        "name": "ivanti-traffic-manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1857752096" == mmh3(base64_py(body)))'
                        ],
                        "name": "searxng-search",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-601917817" == mmh3(base64_py(body)))'
                        ],
                        "name": "ip-webcam",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-139383140" == mmh3(base64_py(body)))'
                        ],
                        "name": "schneider-electric",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("953127083" == mmh3(base64_py(body)))'
                        ],
                        "name": "schneider-electric-xflow",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1323738809" == mmh3(base64_py(body)))'
                        ],
                        "name": "sonatype-nexus-repository-manager",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1294748862" == mmh3(base64_py(body)))'
                        ],
                        "name": "fastpanel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1792417660" == mmh3(base64_py(body)))'
                        ],
                        "name": "oracle-rest-data-services",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1051205943" == mmh3(base64_py(body)))'
                        ],
                        "name": "siemens-simatic-s7-1200",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("607493189" == mmh3(base64_py(body)))'
                        ],
                        "name": "teltonika-networks",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1308238121" == mmh3(base64_py(body)))'
                        ],
                        "name": "reolink",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1519363246" == mmh3(base64_py(body)))'
                        ],
                        "name": "n-central",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1761642992" == mmh3(base64_py(body)))'
                        ],
                        "name": "vesta-hosting-control-panel",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1398529462" == mmh3(base64_py(body)))'
                        ],
                        "name": "grandstream",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("96215732" == mmh3(base64_py(body)))'
                        ],
                        "name": "comxchange",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1818779764" == mmh3(base64_py(body)))'
                        ],
                        "name": "etic-telecom",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1548649046" == mmh3(base64_py(body)))'
                        ],
                        "name": "sophos",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("236252842" == mmh3(base64_py(body)))'
                        ],
                        "name": "esl-management-system",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-1939892142" == mmh3(base64_py(body)))'
                        ],
                        "name": "sabnzbd",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1463037400" == mmh3(base64_py(body)))'
                        ],
                        "name": "enghouse-systems-lifesize",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("1506998866" == mmh3(base64_py(body)))'
                        ],
                        "name": "mitsubishi-air-condition",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("487597720" == mmh3(base64_py(body)))'
                        ],
                        "name": "schneider-electric-ecostruxure",
                    },
                    {
                        "type": "dsl",
                        "dsl": [
                            'status_code==200 && ("-2062596654" == mmh3(base64_py(body)))'
                        ],
                        "name": "arista-velocloud",
                    },
                ],
                "e": [{"type": "dsl", "dsl": ["mmh3(base64_py(body))"]}],
            }
        ],
    }
]


import base64
import re

from cruster import HttpRequest, HttpResponse, finding


def _favicon_hash(value):
    data = str(value).encode("latin-1")
    length = len(data)
    h = 0
    for offset in range(0, length - length % 4, 4):
        k = int.from_bytes(data[offset : offset + 4], "little")
        k = (k * 0xCC9E2D51) & 0xFFFFFFFF
        k = ((k << 15) | (k >> 17)) & 0xFFFFFFFF
        k = (k * 0x1B873593) & 0xFFFFFFFF
        h ^= k
        h = ((h << 13) | (h >> 19)) & 0xFFFFFFFF
        h = (h * 5 + 0xE6546B64) & 0xFFFFFFFF
    tail = data[length - length % 4 :]
    k = 0
    if len(tail) == 3:
        k ^= tail[2] << 16
    if len(tail) >= 2:
        k ^= tail[1] << 8
    if tail:
        k ^= tail[0]
        k = (k * 0xCC9E2D51) & 0xFFFFFFFF
        k = ((k << 15) | (k >> 17)) & 0xFFFFFFFF
        k = (k * 0x1B873593) & 0xFFFFFFFF
        h ^= k
    h ^= length
    h ^= h >> 16
    h = (h * 0x85EBCA6B) & 0xFFFFFFFF
    h ^= h >> 13
    h = (h * 0xC2B2AE35) & 0xFFFFFFFF
    h ^= h >> 16
    return h if h < 0x80000000 else h - 0x100000000


def scan(request: HttpRequest, response: HttpResponse):
    target = request.uri.split("://", 1)[-1]
    target = target[target.find("/") :] if "/" in target else "/"
    if target.split("?", 1)[0].split("#", 1)[0] not in (
        "/favicon.ico",
        "/images/favicon.ico",
    ):
        return []
    status = re.match(r"\s*(\d+)", response.status)
    if not status or int(status.group(1)) != 200:
        return []
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    value = str(
        _favicon_hash(base64.encodebytes(body.encode("latin-1")).decode("ascii"))
    )
    evidence = []
    for signature in FAVICON_SIGNATURES[0]["r"][0]["m"]:
        if f'"{value}"' in signature["dsl"][0]:
            evidence.append(signature["name"])
    return [finding("favicon-detect", extracted=evidence + [value])] if evidence else []
