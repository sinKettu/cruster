metadata = {
    "id": "fingerprinthub-web-fingerprints",
    "about": "FingerprintHub Technology Fingerprint (passive Nuclei technology fingerprint)",
    "authors": ["ProjectDiscovery contributors", "Cruster"],
    "references": [
        "https://github.com/projectdiscovery/nuclei-templates/tree/main/http/technologies"
    ],
    "tags": ["technology", "discovery", "nuclei", "passive"],
    "severity": "info",
    "type": "passive"
}
FINGERPRINTS = [
    {
        "id": "fingerprinthub-web-fingerprints",
        "name": "FingerprintHub Technology Fingerprint",
        "r": [
            {
                "p": ["*"],
                "c": "or",
                "m": [
                    {"words": ["typeof(_08cms)"], "type": "word", "name": "08cms"},
                    {
                        "words": ["/custom/groupnewslist.aspx?groupid="],
                        "type": "word",
                        "name": "1caitong",
                    },
                    {
                        "words": ["技术支持：网格（福建）智能科技有限公司"],
                        "type": "word",
                        "name": "21grid",
                    },
                    {
                        "words": ["net263.wm.custom_login.homepage_init"],
                        "type": "word",
                        "name": "263-enterprise-mailbox",
                    },
                    {
                        "words": ['src="/custom_login/js/net263_wm_util.js'],
                        "type": "word",
                        "name": "263-enterprise-mailbox",
                    },
                    {
                        "words": ['<p align="center">请使用263em登陆!</p>'],
                        "type": "word",
                        "name": "263-hrm",
                    },
                    {
                        "words": [
                            '<frame src="/jsp/conference/meetinglist.jsp" name="mainframe"/>'
                        ],
                        "type": "word",
                        "name": "263-meeting",
                    },
                    {
                        "words": [">多可电子档案管理系统</div"],
                        "type": "word",
                        "name": "315soft-filesystem",
                    },
                    {
                        "words": [
                            "35",
                            "images/mail/35pushmail.app.png",
                            "switchingserverpopup",
                        ],
                        "type": "word",
                        "name": "35mail",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'class="user_define_img_btn6" href="http://help.mail.35.com/mailman/81.html'
                        ],
                        "type": "word",
                        "name": "35mail",
                    },
                    {
                        "words": ["360entinst", "关于全网部署360私有云的通知"],
                        "type": "word",
                        "name": "360-enterprise-security",
                        "condition": "and",
                    },
                    {
                        "words": ["天擎"],
                        "type": "word",
                        "name": "360-enterprise-security",
                    },
                    {
                        "words": [
                            'src="/resource/img/login/logo_403.png" alt="360天机"/></a>'
                        ],
                        "type": "word",
                        "name": "360-tianji",
                    },
                    {
                        "words": ['appid":"skylar6'],
                        "type": "word",
                        "name": "360-tianqing",
                    },
                    {
                        "words": ["/task/index/detail?id={item.id}"],
                        "type": "word",
                        "name": "360-tianqing",
                    },
                    {
                        "words": ["已过期或者未授权，购买请联系4008-136-360"],
                        "type": "word",
                        "name": "360-tianqing",
                    },
                    {
                        "words": ["webscan.360.cn/status/pai/hash"],
                        "type": "word",
                        "name": "360-webscan",
                    },
                    {
                        "words": ["src='http://www.365webcall.com/imme1.aspx?"],
                        "type": "word",
                        "name": "365webcall",
                    },
                    {
                        "words": [
                            "href=https://unpkg.com/element-ui/lib/theme-chalk/index.css"
                        ],
                        "type": "word",
                        "name": "365xxy-examing",
                    },
                    {
                        "words": ["<title>云时政在线考试系统</title>"],
                        "type": "word",
                        "name": "365xxy-examing",
                    },
                    {"words": ["powered by 3dcart"], "type": "word", "name": "3dcart"},
                    {
                        "part": "header",
                        "words": ["X-Powered-By: 3dcart"],
                        "type": "word",
                        "name": "3dcart",
                    },
                    {"words": ["3kits</a>"], "type": "word", "name": "3kits-cms"},
                    {
                        "words": ['href="http://www.3kits.com"'],
                        "type": "word",
                        "name": "3kits-cms",
                    },
                    {
                        "words": ["astrocontacts", "suremdm"],
                        "type": "word",
                        "name": "42gears-suremdm",
                        "condition": "and",
                    },
                    {
                        "words": ["chat.53kf.com/company.php", "chat.53kf.com/kf.php"],
                        "type": "word",
                        "name": "53kf",
                        "condition": "and",
                    },
                    {"words": ["powered by 53kf"], "type": "word", "name": "53kf"},
                    {"words": ["tb.53kf.com/code/"], "type": "word", "name": "53kf"},
                    {
                        "words": ['src="http://code.54kefu.net/'],
                        "type": "word",
                        "name": "54-customer-service",
                    },
                    {
                        "words": ['content="我爱考勤云平台'],
                        "type": "word",
                        "name": "5ikq",
                    },
                    {
                        "words": ["我爱考勤云平台</span>"],
                        "type": "word",
                        "name": "5ikq",
                    },
                    {
                        "words": ["/public/js/5kcrm.js"],
                        "type": "word",
                        "name": "5k-crm",
                    },
                    {
                        "words": ["/cgi-bin/webctrl.cgi?action=index_page"],
                        "type": "word",
                        "name": "5vtechnologies-blueangelsoftwaresuite",
                    },
                    {"words": ["powered by 6kbbs"], "type": "word", "name": "6kbbs"},
                    {
                        "words": ['generator" content="6kbbs'],
                        "type": "word",
                        "name": "6kbbs",
                    },
                    {"words": ['content="74cms.com"'], "type": "word", "name": "74cms"},
                    {"words": ['content="74cms.com'], "type": "word", "name": "74cms"},
                    {"words": ['content="骑士cms'], "type": "word", "name": "74cms"},
                    {
                        "words": ['powered by <a href="http://www.74cms.com/"'],
                        "type": "word",
                        "name": "74cms",
                    },
                    {
                        "words": ["/templates/default/css/common.css"],
                        "type": "word",
                        "name": "74cms",
                    },
                    {"words": ["selectjobscategory"], "type": "word", "name": "74cms"},
                    {
                        "words": [
                            '<a href="http://www.78oa.com" target="_blank">78OA办公系统</a>'
                        ],
                        "type": "word",
                        "name": "78oa",
                    },
                    {
                        "words": ["/resource/javascript/system/runtime.min.js"],
                        "type": "word",
                        "name": "78oa",
                    },
                    {"words": ["license.78oa.com"], "type": "word", "name": "78oa"},
                    {
                        "words": ['class="ds_do_action domain_aboutus"'],
                        "type": "word",
                        "name": "7moor-product",
                    },
                    {
                        "words": ["/javascripts/qiniu/qiniu.js"],
                        "type": "word",
                        "name": "7moor-product",
                    },
                    {
                        "part": "header",
                        "words": ["Server: a2b webserver"],
                        "type": "word",
                        "name": "a2b-webserver",
                    },
                    {
                        "words": ['href="scripts/popmodal.css"'],
                        "type": "word",
                        "name": "aakuan-attendance-system",
                    },
                    {
                        "words": ["aakuan.cn"],
                        "type": "word",
                        "name": "aakuan-attendance-system",
                    },
                    {
                        "words": ["aardvark topsites"],
                        "type": "word",
                        "name": "aardvark-topsites",
                    },
                    {
                        "words": ["secured by accellion"],
                        "type": "word",
                        "name": "accellion-secure-file-transfer",
                    },
                    {
                        "words": ['action="/system/login/login.shtml'],
                        "type": "word",
                        "name": "account-manager-exhibition-system",
                    },
                    {
                        "words": ['content="achecker is a web accessibility'],
                        "type": "word",
                        "name": "achecker-web-accessibility-evaluation-tool",
                    },
                    {
                        "words": ["sdiyun.com, all rights reserved"],
                        "type": "word",
                        "name": "acsoft-cloud",
                    },
                    {
                        "words": ["onrememberpasswordclick"],
                        "type": "word",
                        "name": "acsoft-cloud",
                    },
                    {
                        "words": ['<div id="dsitetitle"', "by:lin.zhibin"],
                        "type": "word",
                        "name": "acsoft-reimbursement-system",
                        "condition": "and",
                    },
                    {
                        "words": [
                            "window.external.addfavorite(location.href,document.title);"
                        ],
                        "type": "word",
                        "name": "acsoft-reimbursement-system",
                    },
                    {
                        "words": ['url:"/ucenter/login/loginaction!gettitle.action",'],
                        "type": "word",
                        "name": "act-manager",
                    },
                    {
                        "words": ['<script>location.href="ucenter";</script>'],
                        "type": "word",
                        "name": "act-manager",
                    },
                    {
                        "words": ["powered by activecollab"],
                        "type": "word",
                        "name": "activecollab",
                    },
                    {
                        "words": [
                            '<p id="powered_by"><a href="http://www.activecollab.com/"'
                        ],
                        "type": "word",
                        "name": "activecollab",
                    },
                    {
                        "words": [
                            "awnocachebegin__awnocachebegin__awnocachebegin__awnocachebegin__awnocachebegin"
                        ],
                        "type": "word",
                        "name": "activeweb-content-server",
                    },
                    {
                        "words": ["activeweb cache extension"],
                        "type": "word",
                        "name": "activeweb-content-server",
                    },
                    {
                        "words": ["<acx-root>", "<title>Acunetix"],
                        "type": "word",
                        "name": "acunetix-wvs",
                        "condition": "and",
                    },
                    {
                        "words": ['action="/maxview/manager/login.xhtml'],
                        "type": "word",
                        "name": "adaptec-maxview",
                    },
                    {
                        "words": ['<img src="/img/logo.png" alt="adimoney"/>'],
                        "type": "word",
                        "name": "adimoney",
                    },
                    {
                        "words": [
                            'content="adimoney.com mobile advertisement network. '
                        ],
                        "type": "word",
                        "name": "adimoney",
                    },
                    {
                        "words": ["adiscon gmbh"],
                        "type": "word",
                        "name": "adiscon-loganalyzer",
                    },
                    {
                        "words": ["https://www.adminer.org"],
                        "type": "word",
                        "name": "adminer",
                    },
                    {"words": ["/cfajax/"], "type": "word", "name": "adobe-coldfusion"},
                    {
                        "words": ["<cfscript>"],
                        "type": "word",
                        "name": "adobe-coldfusion",
                    },
                    {
                        "words": ["/common/scripts/showcontent.js"],
                        "type": "word",
                        "name": "adobe-connect",
                    },
                    {"words": ["_jcr_content"], "type": "word", "name": "adobe-cq5"},
                    {
                        "words": ["adobe experience manager"],
                        "type": "word",
                        "name": "adobe-experience-manager",
                    },
                    {
                        "words": [' class="coral-heading coral-heading--1"'],
                        "type": "word",
                        "name": "adobe-experience-manager",
                    },
                    {"words": ["adobe flex"], "type": "word", "name": "adobe-flex"},
                    {
                        "words": ["learn more about flex at http://flex.org"],
                        "type": "word",
                        "name": "adobe-flex",
                    },
                    {
                        "words": ['generator" content="adobe golive'],
                        "type": "word",
                        "name": "adobe-golive",
                    },
                    {
                        "words": ["/skin/frontend/"],
                        "type": "word",
                        "name": "adobe-magento",
                    },
                    {"words": ["blank_img"], "type": "word", "name": "adobe-magento"},
                    {
                        "words": ['generator" content="adobe robohelp'],
                        "type": "word",
                        "name": "adobe-robohelp",
                    },
                    {
                        "words": ['content="tpn,vpn,内网安全,内网控制,主机防护"'],
                        "type": "word",
                        "name": "adt-iam",
                    },
                    {
                        "words": ["powered by aef"],
                        "type": "word",
                        "name": "advanced-electron-forum",
                    },
                    {
                        "words": ["/bw_templete1.dwt"],
                        "type": "word",
                        "name": "advantech-webaccess",
                    },
                    {
                        "words": ["/broadweb/webaccessclientsetup.exe"],
                        "type": "word",
                        "name": "advantech-webaccess",
                    },
                    {
                        "words": ["/broadweb/bwuconfig.asp"],
                        "type": "word",
                        "name": "advantech-webaccess",
                    },
                    {
                        "words": ["remote manage your intelligent systems"],
                        "type": "word",
                        "name": "advantech_wise",
                    },
                    {
                        "words": [
                            "navigator.serviceworker.register('/adviserlogiccache.js')"
                        ],
                        "type": "word",
                        "name": "adviserlogiccli",
                    },
                    {
                        "words": ["afterlogic webmail pro"],
                        "type": "word",
                        "name": "afterlogic-webmail",
                    },
                    {
                        "words": ['class="logo-element">agile-bpm'],
                        "type": "word",
                        "name": "agilebpm",
                    },
                    {
                        "words": ['class="logo-element">bpm'],
                        "type": "word",
                        "name": "agilebpm",
                    },
                    {
                        "words": ["/agora.cgi?product=", "/store/agora.cgi"],
                        "type": "word",
                        "name": "agoracgi",
                        "condition": "and",
                    },
                    {
                        "words": ["trusguard ssl vpn client"],
                        "type": "word",
                        "name": "ahnlab-trusguard-ssl-vpn",
                    },
                    {
                        "words": ["http://www.aidex.de/"],
                        "type": "word",
                        "name": "aidex",
                    },
                    {
                        "words": [
                            '<font class="bottomfont">航天信息股份有限公司 电信行业版'
                        ],
                        "type": "word",
                        "name": "aisino-telecom",
                    },
                    {
                        "words": ['action="/ajenti:auth"', 'src="/ajenti:static/'],
                        "type": "word",
                        "name": "ajenti-server-admin-panel",
                        "condition": "and",
                    },
                    {
                        "words": ["powered by webboard"],
                        "type": "word",
                        "name": "akiva-webboard",
                    },
                    {"words": ["valoriserdiv5"], "type": "word", "name": "alcasar"},
                    {
                        "words": ["/help/en-us/others/ov-cirrus_cookiepolicy.html"],
                        "type": "word",
                        "name": "alcatel_lucent-omnivista-cirrus",
                    },
                    {
                        "words": ["/monitor/css/monitor.css"],
                        "type": "word",
                        "name": "ali-monitoring-system",
                    },
                    {
                        "words": ['href="/monitor/monitoritem/monitoritemlist.htm'],
                        "type": "word",
                        "name": "ali-monitoring-system",
                    },
                    {
                        "words": [
                            "copyright &copy;  dms  all rights reserved （alibaba 数据管理产品）"
                        ],
                        "type": "word",
                        "name": "alibaba-group-dms",
                    },
                    {
                        "words": ['content="tlog 实时数据处理'],
                        "type": "word",
                        "name": "alibaba-group-tlog",
                    },
                    {
                        "words": ['class="legend">rds管理系统</div>'],
                        "type": "word",
                        "name": "aliyun-rds",
                    },
                    {
                        "words": ["cdn.aliyuncs.com"],
                        "type": "word",
                        "name": "aliyuncdn",
                    },
                    {
                        "part": "header",
                        "words": ["Server: aliyunoss"],
                        "type": "word",
                        "name": "aliyunoss",
                    },
                    {
                        "words": ['window.location = "/swp/group/admin";'],
                        "type": "word",
                        "name": "alliance-web-platform",
                    },
                    {
                        "words": ['href="technology_communion.asp'],
                        "type": "word",
                        "name": "alstom-system",
                    },
                    {"words": ['id="dvlogo"'], "type": "word", "name": "am-websystem"},
                    {"words": ["amazeui.min.js"], "type": "word", "name": "amaze-ui"},
                    {"words": ["amazeui.js"], "type": "word", "name": "amaze-ui"},
                    {"words": ["amazeui.css"], "type": "word", "name": "amaze-ui"},
                    {
                        "words": ["北京众恒志信科技"],
                        "type": "word",
                        "name": "ambuf-onlineexam",
                    },
                    {
                        "words": ["<modelname>ami megarac sp</modelname>"],
                        "type": "word",
                        "name": "ami-megarac-sp",
                    },
                    {
                        "words": ["<modelname>ami megarac spx</modelname>"],
                        "type": "word",
                        "name": "ami-megarac-spx",
                    },
                    {
                        "words": ['content="erwin aligam - ealigam@gmail.com'],
                        "type": "word",
                        "name": "anecms",
                    },
                    {
                        "words": [
                            '<form action="" onsubmit="pacs.login.sendpasswordrecoverymail()'
                        ],
                        "type": "word",
                        "name": "animati-pacs",
                    },
                    {
                        "words": [' id="lblname">版权所有：上海安脉计算机科技有限公司'],
                        "type": "word",
                        "name": "anmai-system",
                    },
                    {
                        "words": ['id="lblname1">版权所有：上海安脉计算机科技有限公司'],
                        "type": "word",
                        "name": "anmai-system",
                    },
                    {
                        "words": [
                            '<font color="#000000">上海安脉计算机科技有限公司</font>'
                        ],
                        "type": "word",
                        "name": "anmai-system",
                    },
                    {
                        "words": ['href="http://www.anneca.cz"'],
                        "type": "word",
                        "name": "anneca-intouch-crm",
                    },
                    {
                        "words": [
                            'setcookie("asglanguage",document.form1.planguage.value)'
                        ],
                        "type": "word",
                        "name": "anta-asg",
                    },
                    {
                        "words": ["vcode.aspx"],
                        "type": "word",
                        "name": "aolansoft-studentsystem",
                    },
                    {
                        "words": ["default/apabi.css"],
                        "type": "word",
                        "name": "apabi-digital-resource-platform",
                    },
                    {
                        "words": ['<link href="http://apabi'],
                        "type": "word",
                        "name": "apabi-digital-resource-platform",
                    },
                    {
                        "words": ["<title>Apache ActiveMQ</title>"],
                        "type": "word",
                        "name": "apache-activemq",
                    },
                    {"words": ["Airflow"], "type": "word", "name": "apache-airflow"},
                    {
                        "words": ['src="/static/pin_100.png"'],
                        "type": "word",
                        "name": "apache-airflow",
                    },
                    {
                        "words": ["<span>airflow</span>"],
                        "type": "word",
                        "name": "apache-airflow",
                    },
                    {
                        "words": ['"/licenses/NOTICE.txt"', "<title>Ambari</title>"],
                        "type": "word",
                        "name": "apache-ambari",
                        "condition": "and",
                    },
                    {
                        "words": ["/archiva.js"],
                        "type": "word",
                        "name": "apache-archiva",
                    },
                    {
                        "words": ["/archiva.css"],
                        "type": "word",
                        "name": "apache-archiva",
                    },
                    {
                        "words": ["http://ws.apache.org/axis2"],
                        "type": "word",
                        "name": "apache-axis",
                    },
                    {
                        "words": ["axis2-admin", "axis2-web"],
                        "type": "word",
                        "name": "apache-axis2",
                        "condition": "and",
                    },
                    {
                        "words": ['content="Apache Druid console"'],
                        "type": "word",
                        "name": "apache-druid",
                    },
                    {
                        "part": "header",
                        "words": ["X-Cocoon-Version"],
                        "type": "word",
                        "name": "apache-cocoon",
                    },
                    {
                        "words": ["<title>Apache Flink Web Dashboard</title>"],
                        "type": "word",
                        "name": "apache-flink",
                    },
                    {
                        "words": [
                            '<img alt="apache flink dashboard" src="images/flink-logo.png'
                        ],
                        "type": "word",
                        "name": "apache-flink",
                    },
                    {
                        "words": ['content="apache forrest'],
                        "type": "word",
                        "name": "apache-forrest",
                    },
                    {
                        "words": ['name="forrest'],
                        "type": "word",
                        "name": "apache-forrest",
                    },
                    {
                        "words": ["images/guacamole-logo"],
                        "type": "word",
                        "name": "apache-guacamole",
                    },
                    {
                        "words": [
                            "guacamole - clientless remote desktop",
                            "scripts/guac-ui.js",
                        ],
                        "type": "word",
                        "name": "apache-guacamole",
                        "condition": "and",
                    },
                    {
                        "words": ["/static/hadoop-st.png", "parseHadoopProgress"],
                        "type": "word",
                        "name": "apache-hadoop",
                        "condition": "and",
                    },
                    {
                        "words": ["/static/yarn.css"],
                        "type": "word",
                        "name": "apache-hadoop-yarn",
                    },
                    {
                        "words": ["yarn.dt.plugins.js"],
                        "type": "word",
                        "name": "apache-hadoop-yarn",
                    },
                    {
                        "words": ['href="/apachehaus.ico'],
                        "type": "word",
                        "name": "apache-haus",
                    },
                    {
                        "words": [
                            'copyright &copy; 2008-2017 <a href="http://www.apachehaus.com">the apache haus</a>'
                        ],
                        "type": "word",
                        "name": "apache-haus",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="1;url=kylin">',
                            'href="/kylin/"',
                        ],
                        "type": "word",
                        "name": "apache-kylin",
                        "condition": "or",
                    },
                    {
                        "words": [
                            '<img src="/static/img/mesos_logo.png" alt="apache mesos">'
                        ],
                        "type": "word",
                        "name": "apache-mesos",
                    },
                    {
                        "words": ['<a href="/nifi/">/nifi</a>'],
                        "type": "word",
                        "name": "apache-nifi",
                    },
                    {
                        "words": ["<span>Powered by OFBiz</span>"],
                        "type": "word",
                        "name": "apache-ofbiz",
                    },
                    {
                        "words": ["oozie-console"],
                        "type": "word",
                        "name": "apache-oozie-web-console",
                    },
                    {
                        "words": ['href="/oozie">oozie console'],
                        "type": "word",
                        "name": "apache-oozie-web-console",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: rememberMe=deleteMe"],
                        "type": "word",
                        "name": "apache-shiro",
                    },
                    {
                        "words": ["</i> shiro</li>"],
                        "type": "word",
                        "name": "apache-shiro",
                    },
                    {
                        "words": ["sorry but SkyWalking doesn't work"],
                        "type": "word",
                        "name": "apache-skywalking",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /solr/"],
                        "type": "word",
                        "name": "apache-solr",
                    },
                    {
                        "case-insensitive": True,
                        "part": "header",
                        "words": ['Www-Authenticate: Basic realm="dubbo"'],
                        "type": "word",
                        "name": "apache-dubbo",
                    },
                    {
                        "words": [
                            'content="Struts2 Showcase for Apache Struts Project"'
                        ],
                        "type": "word",
                        "name": "apache-struts",
                    },
                    {
                        "words": ["logo apache unomi"],
                        "type": "word",
                        "name": "apache-unomi",
                    },
                    {
                        "words": ["xmlns:wicket="],
                        "type": "word",
                        "name": "apache-wicket",
                    },
                    {
                        "words": ["/org.apache.wicket."],
                        "type": "word",
                        "name": "apache-wicket",
                    },
                    {
                        "words": [
                            "this object on the apc management web server is protected"
                        ],
                        "type": "word",
                        "name": "apc-management",
                    },
                    {
                        "words": ["cas &#8211; central authentication service"],
                        "type": "word",
                        "name": "apereo-cas",
                    },
                    {
                        "words": ['href="/plug-in/login/fixed/css/login.css"'],
                        "type": "word",
                        "name": "apex-livebpm",
                    },
                    {
                        "part": "header",
                        "words": ["Server: Caddy"],
                        "type": "word",
                        "name": "apilayer-caddy",
                    },
                    {"words": ["powerd by appcms"], "type": "word", "name": "appcms"},
                    {
                        "words": ["appex network corporation"],
                        "type": "word",
                        "name": "appex-lotapp",
                    },
                    {
                        "words": ["/change_lan.php?lanid=en"],
                        "type": "word",
                        "name": "appex-lotapp",
                    },
                    {
                        "words": ["this script was generated by apphp calendar"],
                        "type": "word",
                        "name": "apphp-calendar",
                    },
                    {
                        "words": ["appserv/softicon.gif"],
                        "type": "word",
                        "name": "appserv",
                    },
                    {
                        "words": ["index.php?appservlang=th"],
                        "type": "word",
                        "name": "appserv",
                    },
                    {
                        "words": ["<td>管理apusic应用服务器</td>"],
                        "type": "word",
                        "name": "apusic",
                    },
                    {
                        "part": "server",
                        "words": ["Apusic Application Server"],
                        "type": "word",
                        "name": "apusic",
                    },
                    {
                        "words": ["powered by: arab"],
                        "type": "word",
                        "name": "arab-portal",
                    },
                    {
                        "words": ["argosoft mail server plus for"],
                        "type": "word",
                        "name": "argosoft-mail-server",
                    },
                    {"words": ["an_util.js"], "type": "word", "name": "array-vpn"},
                    {
                        "words": ["www.articlepublisherpro.com"],
                        "type": "word",
                        "name": "articlepublisherpro",
                    },
                    {
                        "words": ['content="article publisher pro'],
                        "type": "word",
                        "name": "articlepublisherpro",
                    },
                    {
                        "part": "header",
                        "words": ["Server: arvancloud"],
                        "type": "word",
                        "name": "arvancloud",
                    },
                    {
                        "words": ["upload/moban/images/style.css"],
                        "type": "word",
                        "name": "asp168-oho",
                    },
                    {
                        "words": ["default.php?mod=article&do=detail&tid"],
                        "type": "word",
                        "name": "asp168-oho",
                    },
                    {"words": ['content="aspcms'], "type": "word", "name": "aspcms"},
                    {
                        "words": ["/inc/aspcms_advjs.asp"],
                        "type": "word",
                        "name": "aspcms",
                    },
                    {
                        "words": ['src="/aspencui/css/appstyles.js'],
                        "type": "word",
                        "name": "aspentech-aspen-infoplus21",
                    },
                    {
                        "words": [
                            "<h2>modify this template to jump-start your asp.net mvc application.</h2>"
                        ],
                        "type": "word",
                        "name": "aspnet-mvc",
                    },
                    {
                        "words": ["asp.net mvc application</p>"],
                        "type": "word",
                        "name": "aspnet-mvc",
                    },
                    {
                        "words": ["httprequestvalidationexception"],
                        "type": "word",
                        "name": "aspnet-requestvalidationmode",
                    },
                    {
                        "words": [
                            "request validation has detected a potentially dangerous client input value"
                        ],
                        "type": "word",
                        "name": "aspnet-requestvalidationmode",
                    },
                    {
                        "words": ["surf the web invisibly using asproxy power"],
                        "type": "word",
                        "name": "asproxy",
                    },
                    {
                        "words": ["btnasproxydisplaybutton"],
                        "type": "word",
                        "name": "asproxy",
                    },
                    {
                        "words": ["/js/_variables_from_backend.js?"],
                        "type": "word",
                        "name": "astaro-command-center",
                    },
                    {
                        "words": ["commandcenter"],
                        "type": "word",
                        "name": "astaro-command-center",
                    },
                    {
                        "words": ["asterisk_rawmanpath"],
                        "type": "word",
                        "name": "asterisk",
                    },
                    {
                        "words": ['href="/smb/css/startup.png"'],
                        "type": "word",
                        "name": "asus-aicloud",
                    },
                    {
                        "words": ["/content/web/theme/skin01/img/p_login_logo01.png"],
                        "type": "word",
                        "name": "atfuture-system",
                    },
                    {"words": ["powered by atmail"], "type": "word", "name": "atmail"},
                    {
                        "words": ["/index.php/mail/auth/processlogin"],
                        "type": "word",
                        "name": "atmail",
                    },
                    {
                        "words": ['<input id="mailserverinput'],
                        "type": "word",
                        "name": "atmail",
                    },
                    {
                        "words": ['content="atutor'],
                        "type": "word",
                        "name": "atutor-elearning",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: atutorid"],
                        "type": "word",
                        "name": "atutor-elearning",
                    },
                    {
                        "words": [
                            "<!-- aurion teal will be used as the login-time default"
                        ],
                        "type": "word",
                        "name": "aurion",
                    },
                    {"words": ["/aurion.js"], "type": "word", "name": "aurion"},
                    {
                        "words": ["h3 bpm suite信息化的最佳实践"],
                        "type": "word",
                        "name": "authine-h3-bpm",
                    },
                    {
                        "words": ['title="autoindex default'],
                        "type": "word",
                        "name": "autoindex-php-script",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: autoindex2"],
                        "type": "word",
                        "name": "autoindex-php-script",
                    },
                    {
                        "words": ["autoindex.sourceforge.net/"],
                        "type": "word",
                        "name": "autoindex-php-script",
                    },
                    {
                        "words": ['href="/_common/lvl5/about/eula.jsp"'],
                        "type": "word",
                        "name": "automatedlogiccorporation-webctrl",
                    },
                    {"words": [".logo-autoset"], "type": "word", "name": "autoset"},
                    {
                        "words": ["index.php?cmd=11"],
                        "type": "word",
                        "name": "auxilium-petratepro",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.avscripts.net/avarcade/'
                        ],
                        "type": "word",
                        "name": "av-arcade",
                    },
                    {
                        "words": [
                            'src="images/avantfax-big.png" border="0" alt="avantfax'
                        ],
                        "type": "word",
                        "name": "avantfax",
                    },
                    {
                        "words": [
                            '<img src="images/avantfax-big.png" border="0" alt="ictfax'
                        ],
                        "type": "word",
                        "name": "avantfax-ictfax",
                    },
                    {
                        "words": ['content="ictfax'],
                        "type": "word",
                        "name": "avantfax-ictfax",
                    },
                    {
                        "words": ["<b>application enablement services&nbsp;</b>"],
                        "type": "word",
                        "name": "avaya-application-enablement-services",
                    },
                    {
                        "words": ["avaya"],
                        "type": "word",
                        "name": "avaya-application-enablement-services",
                    },
                    {
                        "words": ['vmstitle">avaya aura&#8482;&nbsp;utility server'],
                        "type": "word",
                        "name": "avaya-aura-utility-server",
                    },
                    {
                        "words": ["/webhelp/base/utility_toc.htm"],
                        "type": "word",
                        "name": "avaya-aura-utility-server",
                    },
                    {
                        "words": ["avaya aura&reg;&nbsp;utility services"],
                        "type": "word",
                        "name": "avaya-aura-utility-server",
                    },
                    {
                        "words": ["avaya inc. all rights reserved"],
                        "type": "word",
                        "name": "avaya-aura-utility-server",
                    },
                    {
                        "words": [
                            'var newlocation = "https://" + target + "/cgi-bin/common/issue";'
                        ],
                        "type": "word",
                        "name": "avaya-communication-manager",
                    },
                    {
                        "words": ['content="0;url=vsplogin.action'],
                        "type": "word",
                        "name": "avaya-system-platform",
                    },
                    {
                        "words": ["/av732e/setup.exe"],
                        "type": "word",
                        "name": "avtech-video-web-server",
                    },
                    {
                        "words": ["welcome to nginx on amazon ec2!"],
                        "type": "word",
                        "name": "aws-ec2",
                    },
                    {
                        "words": ["<h2>what's next?</h2>"],
                        "type": "word",
                        "name": "aws-elastic-beanstalk",
                    },
                    {
                        "words": ["aws.amazon.com/elasticbeanstalk"],
                        "type": "word",
                        "name": "aws-elastic-beanstalk",
                    },
                    {"words": ['content="axcms.net'], "type": "word", "name": "axcms"},
                    {
                        "words": ["generated by axcms.net"],
                        "type": "word",
                        "name": "axcms",
                    },
                    {
                        "words": ['content="axentra'],
                        "type": "word",
                        "name": "axentra-hipserv",
                    },
                    {
                        "words": ['class="axgate"'],
                        "type": "word",
                        "name": "axgate-sslvpn",
                    },
                    {
                        "words": ["axis2-web/css/axis-style.css"],
                        "type": "word",
                        "name": "axis2-web",
                    },
                    {
                        "part": "header",
                        "words": ["Server: axhttpd"],
                        "type": "word",
                        "name": "axtls-embad-httpd",
                    },
                    {
                        "words": ["/powered-by-b2evolution-150t.gif"],
                        "type": "word",
                        "name": "b2evolution",
                    },
                    {
                        "words": ["powered by b2evolution"],
                        "type": "word",
                        "name": "b2evolution",
                    },
                    {
                        "words": ['content="b2evolution'],
                        "type": "word",
                        "name": "b2evolution",
                    },
                    {
                        "words": ['<div id="bb5-site-wrapper">'],
                        "type": "word",
                        "name": "backbee",
                    },
                    {
                        "words": [
                            "登录密码错误次数超过5次，帐号被锁定。请联系省坏账系统管理员，或发邮件解锁"
                        ],
                        "type": "word",
                        "name": "bad-debt-management-system",
                    },
                    {
                        "words": ['id="yunjiasu_link'],
                        "type": "word",
                        "name": "baidu-subaidu",
                    },
                    {
                        "words": ["/resource/images/cms.ico"],
                        "type": "word",
                        "name": "baishijia-cms",
                    },
                    {
                        "words": ["bim 开发配置与运维控制台"],
                        "type": "word",
                        "name": "bamboocloud-bim",
                    },
                    {
                        "words": ["pm2项目管理系统bs版增强工具.zip"],
                        "type": "word",
                        "name": "bangyong-pm2",
                    },
                    {
                        "words": ["barracuda ssl vpn"],
                        "type": "word",
                        "name": "barracuda-ssl-vpn",
                    },
                    {
                        "words": ['powered by: <a href="http://www.mevin.com/">'],
                        "type": "word",
                        "name": "basic-php-events-lister",
                    },
                    {
                        "words": [
                            "<!-- if you like showing off the fact that your server rocks -->"
                        ],
                        "type": "word",
                        "name": "bbpress",
                    },
                    {
                        "words": ['is proudly powered by <a href="http://bbpress.org'],
                        "type": "word",
                        "name": "bbpress",
                    },
                    {
                        "words": [
                            "beescms",
                            "template/default/images/slides.min.jquery.js",
                        ],
                        "type": "word",
                        "name": "bees_cms",
                        "condition": "and",
                    },
                    {
                        "words": ["/default/images/xslider.js"],
                        "type": "word",
                        "name": "bees_cms",
                    },
                    {
                        "words": ["/default/images/search_btn.gif"],
                        "type": "word",
                        "name": "bees_cms",
                    },
                    {
                        "words": ["powerd by beescms"],
                        "type": "word",
                        "name": "bees_cms",
                    },
                    {
                        "words": ["mx_form/mx_form.php"],
                        "type": "word",
                        "name": "bees_cms",
                    },
                    {
                        "words": ["opac_two"],
                        "type": "word",
                        "name": "beichuang-book-retrieval-system",
                    },
                    {
                        "words": ['href="projectwise.ico'],
                        "type": "word",
                        "name": "bentley-systems-projectwise",
                    },
                    {
                        "words": ["/userfiles/admin/customskin"],
                        "type": "word",
                        "name": "bestsch-ecs",
                    },
                    {
                        "words": ['src="/include/ecsserverapi.js'],
                        "type": "word",
                        "name": "bestsch-ecs",
                    },
                    {
                        "words": ['align="center" class="login_pdm">'],
                        "type": "word",
                        "name": "betasoft-pdm-data-acquisition",
                    },
                    {
                        "words": [
                            "background: no-repeat url(../images/login/pdmdenglu1_28.png);"
                        ],
                        "type": "word",
                        "name": "betasoft-pdm-data-acquisition",
                    },
                    {
                        "words": ["版权所有:郑州蓝视科技有限公司"],
                        "type": "word",
                        "name": "beyeon-iot",
                    },
                    {
                        "words": [
                            "var app_smp_type_name = '门店';var app_grp_type_name = '集团'"
                        ],
                        "type": "word",
                        "name": "beyeon-iot",
                    },
                    {
                        "words": ['bhclientcer:"/modules/web/common/data/bhclient.cer'],
                        "type": "word",
                        "name": "bh-bh5000c",
                    },
                    {
                        "words": ['href="images/bicesoft.css"'],
                        "type": "word",
                        "name": "bicesoft-super-custom-survey-voting-system",
                    },
                    {
                        "words": ["佰思超强自定义问卷调查系统(bicesoft.com)"],
                        "type": "word",
                        "name": "bicesoft-super-custom-survey-voting-system",
                    },
                    {
                        "words": ['class="loginin loginin1"'],
                        "type": "word",
                        "name": "biept-system",
                    },
                    {
                        "words": ["bigdump: staggered mysql dump importer"],
                        "type": "word",
                        "name": "bigdump",
                    },
                    {
                        "words": ["href:'/billtool/querysum'"],
                        "type": "word",
                        "name": "billingtesttool",
                    },
                    {
                        "words": ["/lims/dist/css/font-awesome.min.css"],
                        "type": "word",
                        "name": "bio-lims",
                    },
                    {
                        "words": ["/bds/stylesheets/fds.css"],
                        "type": "word",
                        "name": "biscom-delivery-server",
                    },
                    {
                        "words": ["/bds/includes/fdsjavascript.do"],
                        "type": "word",
                        "name": "biscom-delivery-server",
                    },
                    {
                        "words": ["bit-xxzs", "xmlpzs/webissue.asp"],
                        "type": "word",
                        "name": "bit-service",
                        "condition": "and",
                    },
                    {
                        "words": ["bitbucket.page.login"],
                        "type": "word",
                        "name": "bitbucket",
                    },
                    {
                        "words": [
                            "href='http://www.bithighway.com' target=_blank>北京碧海威科技有限公司<"
                        ],
                        "type": "word",
                        "name": "bithighway-product",
                    },
                    {
                        "part": "header",
                        "words": ["Server: bkhttp"],
                        "type": "word",
                        "name": "bitkeeper",
                    },
                    {
                        "words": ['alt="bitnami redmine stack'],
                        "type": "word",
                        "name": "bitnami-redmine-stack",
                    },
                    {
                        "words": ["bitrix_sm_time_zone"],
                        "type": "word",
                        "name": "bitrix-site-manager",
                    },
                    {
                        "words": ["bx.setcsslist"],
                        "type": "word",
                        "name": "bitrix-site-manager",
                    },
                    {
                        "words": [
                            '<li><a href="/install/certapp_bd.exe">下载证书应用环境</a></li>'
                        ],
                        "type": "word",
                        "name": "bjca",
                    },
                    {
                        "words": ["href=/css/ordercomplaint"],
                        "type": "word",
                        "name": "bjqit-crm",
                    },
                    {
                        "part": "header",
                        "words": ["Server: blackjumbodog"],
                        "type": "word",
                        "name": "blackjumbodog",
                    },
                    {
                        "part": "header",
                        "words": ["Server: blazix java server"],
                        "type": "word",
                        "name": "blazix",
                    },
                    {
                        "words": ["pics/blogengine.ico"],
                        "type": "word",
                        "name": "blogenginenet",
                    },
                    {
                        "words": ["http://www.dotnetblogengine.net"],
                        "type": "word",
                        "name": "blogenginenet",
                    },
                    {"words": ["content='blogger"], "type": "word", "name": "blogger"},
                    {
                        "words": ["powered by blogger"],
                        "type": "word",
                        "name": "blogger",
                    },
                    {
                        "words": ["thank you for using the blueonyx"],
                        "type": "word",
                        "name": "blueonyx",
                    },
                    {
                        "words": ["/biradarserver/web/"],
                        "type": "word",
                        "name": "bluepacific-network-monitoring-system",
                    },
                    {
                        "words": ["/visadmin/viscms/index.do"],
                        "type": "word",
                        "name": "bluepacific-share-content-management-system",
                    },
                    {
                        "words": ['value="copyright (c) 2000, cobalt networks'],
                        "type": "word",
                        "name": "bluequartz",
                    },
                    {
                        "words": ["powered by boastmachine"],
                        "type": "word",
                        "name": "boastmachine",
                    },
                    {
                        "words": ['powered by <a href="http://boastology.com'],
                        "type": "word",
                        "name": "boastmachine",
                    },
                    {
                        "words": ['<span class="footer_t">powered by bossmail</span>'],
                        "type": "word",
                        "name": "bossmail",
                    },
                    {
                        "words": [
                            'href="http://apps.microsoft.com/windows/zh-cn/app/bossmail/24f4bdb3-1bca-467e-9dd9-15a5d278aec6'
                        ],
                        "type": "word",
                        "name": "bossmail",
                    },
                    {
                        "words": ["/public/base/js/plugins/crypto/rsa.js"],
                        "type": "word",
                        "name": "bowen-providence-car-loading-reservation-system",
                    },
                    {
                        "words": [
                            "var bxnstaticresroot='/bxn-static-resource/resources'"
                        ],
                        "type": "word",
                        "name": "boxiao-system",
                    },
                    {
                        "words": ['developed by <a href="http://www.zkdigital.com'],
                        "type": "word",
                        "name": "brewblogger",
                    },
                    {
                        "words": ["education area management support system : amss++"],
                        "type": "word",
                        "name": "bridge5asia-amss",
                    },
                    {
                        "words": ["/statics/js/mdo-angular-cryptography.js"],
                        "type": "word",
                        "name": "bridge5asia-amss",
                    },
                    {
                        "words": ["ispamclient = false"],
                        "type": "word",
                        "name": "broadcom-ca-pam",
                    },
                    {
                        "words": ["/cspm/cleansession.jsp"],
                        "type": "word",
                        "name": "broadcom-ca-pam",
                    },
                    {
                        "words": ['window.location.host + "/agweb"'],
                        "type": "word",
                        "name": "brocade-data-angle-guard-database",
                    },
                    {
                        "words": [
                            '<span class="ui-menuitem-text">about network advisor</span></a>'
                        ],
                        "type": "word",
                        "name": "brocade-network-advisor",
                    },
                    {
                        "words": ["powered by browsercms"],
                        "type": "word",
                        "name": "browsercms",
                    },
                    {
                        "words": ['content="browsercms'],
                        "type": "word",
                        "name": "browsercms",
                    },
                    {
                        "words": ['id="logo" alt=bugfree'],
                        "type": "word",
                        "name": "bugfree",
                    },
                    {
                        "words": ['class="loginbgimage" alt="bugfree'],
                        "type": "word",
                        "name": "bugfree",
                    },
                    {"words": ["enter_bug.cgi"], "type": "word", "name": "bugzilla"},
                    {
                        "words": ["/cgi-bin/bugzilla/"],
                        "type": "word",
                        "name": "bugzilla",
                    },
                    {
                        "words": ["/modalpopup/core-modalpopup.css"],
                        "type": "word",
                        "name": "bulletlink-newspaper-template",
                    },
                    {
                        "words": ["powered by bulletlink"],
                        "type": "word",
                        "name": "bulletlink-newspaper-template",
                    },
                    {
                        "words": ["<title>Bullwark Momentum Series</title>"],
                        "type": "word",
                        "name": "bullwark",
                    },
                    {
                        "words": ['powered by <b><a href="http://www.woltlab.de'],
                        "type": "word",
                        "name": "burning-board-lite",
                    },
                    {
                        "words": ["powered by <b>burning board"],
                        "type": "word",
                        "name": "burning-board-lite",
                    },
                    {
                        "words": ["//获取营业台席pc机 ip地址 mac地址"],
                        "type": "word",
                        "name": "business-paperless-system",
                    },
                    {
                        "words": ['onsubmit="return checksubmit()'],
                        "type": "word",
                        "name": "business-system",
                    },
                    {
                        "words": ["function hiddenpass(e)"],
                        "type": "word",
                        "name": "business-system",
                    },
                    {
                        "words": ['window.location=contextpath+"/work/index.jsp"'],
                        "type": "word",
                        "name": "business-system",
                    },
                    {
                        "words": ["function omiga_window(url)"],
                        "type": "word",
                        "name": "business-system",
                    },
                    {
                        "words": ["images/login_d.png"],
                        "type": "word",
                        "name": "business-system",
                    },
                    {
                        "words": ["function updatapipeline(pipelinename)"],
                        "type": "word",
                        "name": "business-system",
                    },
                    {
                        "words": ["请输入正确的电子邮件地址，如：abc@bxemail.com"],
                        "type": "word",
                        "name": "bxemail",
                    },
                    {
                        "words": ["<h1>关于c-lodop免费和注册授权</h1>"],
                        "type": "word",
                        "name": "c-lodop",
                    },
                    {
                        "words": [
                            'document.getelementbyid("reqid").value==document.getelementbyid("licid").value'
                        ],
                        "type": "word",
                        "name": "c-lodop",
                    },
                    {
                        "words": ["<!-- siteminder encoding"],
                        "type": "word",
                        "name": "ca-siteminder",
                    },
                    {
                        "words": [
                            'alert("系统不存在该用户名，请确认该用户申请了cachecloud权限!");'
                        ],
                        "type": "word",
                        "name": "cachecloud",
                    },
                    {
                        "words": ["https://cachethq.io", "Atom"],
                        "type": "word",
                        "name": "cachethq",
                        "condition": "and",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: cacti="],
                        "type": "word",
                        "name": "cacti",
                    },
                    {
                        "words": ["/plugins/jqueryskin/include/login.css"],
                        "type": "word",
                        "name": "cacti",
                    },
                    {
                        "words": ['powered by <a href="http://www.calendarscript.com'],
                        "type": "word",
                        "name": "calendarscript",
                    },
                    {
                        "words": [
                            'content="camera life',
                            "this site is powered by camera life",
                        ],
                        "type": "word",
                        "name": "cameralife",
                        "condition": "and",
                    },
                    {
                        "words": ['content="campsite'],
                        "type": "word",
                        "name": "campsite",
                    },
                    {
                        "words": ["harbin synjones electronic"],
                        "type": "word",
                        "name": "campus-card-management-system",
                    },
                    {
                        "words": ['document.formpostds.action="xxsearch.action'],
                        "type": "word",
                        "name": "campus-card-management-system",
                    },
                    {
                        "words": ["/shouyeziti.css"],
                        "type": "word",
                        "name": "campus-card-management-system",
                    },
                    {
                        "words": ['var path = "/cassets";'],
                        "type": "word",
                        "name": "cancosoft-asset-management",
                    },
                    {
                        "words": ['src="/cariweb/images/images-new'],
                        "type": "word",
                        "name": "cari-system",
                    },
                    {
                        "words": [
                            'href="/resources/rainmailvpninstaller.exe',
                            ".: <b>rainmail intranet login </b> :.</div>",
                        ],
                        "type": "word",
                        "name": "carizen-rainmail",
                        "condition": "and",
                    },
                    {
                        "words": ["<p>copyright&copy cbss 项目组 自动化测试小组</p>"],
                        "type": "word",
                        "name": "cbss-automated-testing",
                    },
                    {
                        "words": ["</a>登录cbss系统</p>"],
                        "type": "word",
                        "name": "cbss-system",
                    },
                    {
                        "words": ["<li>com.cbss.xss.filter.xssfilter.dofilter"],
                        "type": "word",
                        "name": "cbss-system",
                    },
                    {
                        "words": ['<script src="http://kefu.qycn.com/'],
                        "type": "word",
                        "name": "cc-customer-service",
                    },
                    {
                        "words": ["/static/cdr-stats/js/jquery"],
                        "type": "word",
                        "name": "cdr-stats",
                    },
                    {
                        "words": ['powered by: <a href="http://www.censura.info'],
                        "type": "word",
                        "name": "censura",
                    },
                    {
                        "words": ["new ct.extapp.aboutsystemwindow()"],
                        "type": "word",
                        "name": "centerm",
                    },
                    {
                        "words": ['generator" content="centreon - copyright'],
                        "type": "word",
                        "name": "centreon",
                    },
                    {
                        "words": ['class="ceph-none-found" rv-hide="rbd_pools'],
                        "type": "word",
                        "name": "ceph",
                    },
                    {
                        "words": [
                            "<!-- if you have your own stylesheet for html elements, you can remove the "
                            "cerberus-html.css link"
                        ],
                        "type": "word",
                        "name": "cerberus-helpdesk",
                    },
                    {
                        "words": ['lang="en" ng-app="cerebro'],
                        "type": "word",
                        "name": "cerebro",
                    },
                    {
                        "words": ['<a href="http://www.jmarshall.com/tools/cgiproxy/'],
                        "type": "word",
                        "name": "cgiproxy",
                    },
                    {"words": ["<div id='cgit'>"], "type": "word", "name": "cgit"},
                    {"words": ["href='/cgit.css'/>"], "type": "word", "name": "cgit"},
                    {"words": ["content='cgit"], "type": "word", "name": "cgit"},
                    {
                        "words": ["><script>location='/tplus/';</script></body>"],
                        "type": "word",
                        "name": "chanjet-tplus",
                    },
                    {
                        "words": ["title='cms系统，首选蝉知cms"],
                        "type": "word",
                        "name": "chanzhicms",
                    },
                    {"words": ["chanzhi.js"], "type": "word", "name": "chanzhicms"},
                    {
                        "words": ["poweredby'><a href='http://www.chanzhi.org"],
                        "type": "word",
                        "name": "chanzhicms",
                    },
                    {
                        "words": ["iaus/media/js/login/login.js"],
                        "type": "word",
                        "name": "chelen-system",
                    },
                    {
                        "words": ['window.location="/vmonitor";'],
                        "type": "word",
                        "name": "chenrui-video-security-access-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: cherokee"],
                        "type": "word",
                        "name": "cherokee",
                    },
                    {
                        "words": ['powered by <a href="https://www.chiliproject.org/'],
                        "type": "word",
                        "name": "chiliproject",
                    },
                    {
                        "words": ['content="chiliproject'],
                        "type": "word",
                        "name": "chiliproject",
                    },
                    {
                        "words": ["视翰公司"],
                        "type": "word",
                        "name": "china-shine-video-on-demand",
                    },
                    {
                        "words": ['background="images/cbackground.jpg"'],
                        "type": "word",
                        "name": "china-shine-video-on-demand",
                    },
                    {
                        "words": ["/integrats/gs.sub.systemmanager/dologin/json"],
                        "type": "word",
                        "name": "chinags-cloudlearning",
                    },
                    {
                        "words": ['src="/animation/images/teacher_2.png"'],
                        "type": "word",
                        "name": "chinags-sc",
                    },
                    {
                        "words": ['innerhtml="chinamdm移动终端管理系统'],
                        "type": "word",
                        "name": "chinamdm-mobile-device-management",
                    },
                    {
                        "words": ["justsy/user/searchmenusbyusername/"],
                        "type": "word",
                        "name": "chinamdm-mobile-device-management",
                    },
                    {
                        "words": ['src="images/logo_cotsr.png"'],
                        "type": "word",
                        "name": "chinaorgantransplantresponsesystem",
                    },
                    {
                        "words": ['href="nhmis.css"'],
                        "type": "word",
                        "name": "chinatelecom-guestsupportsystem",
                    },
                    {
                        "words": ["var requiredfieldvalidator2 = document.all "],
                        "type": "word",
                        "name": "chinatelecom-guestsupportsystem",
                    },
                    {
                        "words": ['id="v_login_container"'],
                        "type": "word",
                        "name": "chinatelecomarrearsrecoverymanagementsystem",
                    },
                    {
                        "words": ["<strong>客户宽带地址查询"],
                        "type": "word",
                        "name": "chinatelecomcustomerbroadbandaddressquerysystem",
                    },
                    {
                        "words": ["设备web配置</font"],
                        "type": "word",
                        "name": "chinatelecomequipmentwebconfigurationsystem",
                    },
                    {
                        "words": ["风险治理平台</div>"],
                        "type": "word",
                        "name": "chinatelecomriskmanagementplatform",
                    },
                    {
                        "words": ['document.domain == "eqs.jztj.net"'],
                        "type": "word",
                        "name": "chinatiejun-system",
                    },
                    {
                        "words": [
                            '<a href="http://www.chnitc.com" style="text-decoration:none;color:#fff;">'
                        ],
                        "type": "word",
                        "name": "chnitc-system",
                    },
                    {
                        "words": ["科业开发团队出品"],
                        "type": "word",
                        "name": "chnitc-system",
                    },
                    {
                        "words": ["cicro", 'content="cicro', "cws"],
                        "type": "word",
                        "name": "cicro",
                        "condition": "and",
                    },
                    {
                        "words": ["index.files/cicro_userdefine.css"],
                        "type": "word",
                        "name": "cicro",
                    },
                    {
                        "words": ['powered by <a href="http://www.forperfect.com/'],
                        "type": "word",
                        "name": "cinvoice",
                    },
                    {
                        "words": ["ciphermail email encryption gateway"],
                        "type": "word",
                        "name": "ciphermail-email-encryption-gateway",
                    },
                    {
                        "words": [
                            'window.location.href = "/dlp/admin/user/login.action"'
                        ],
                        "type": "word",
                        "name": "cirrusgate-system",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=/acsadmin" />',
                            "cisco",
                        ],
                        "type": "word",
                        "name": "cisco-acs",
                        "condition": "and",
                    },
                    {
                        "words": ['href="/acsadmin">launch acs'],
                        "type": "word",
                        "name": "cisco-acs",
                    },
                    {
                        "words": ["expressway-e</legend>", "Cisco Expressway-E"],
                        "type": "word",
                        "name": "cisco-expressway",
                    },
                    {
                        "words": ['font-family: "ciscosansthin"'],
                        "type": "word",
                        "name": "cisco-imc-supervisor",
                    },
                    {
                        "words": ["cisco imc supervisor"],
                        "type": "word",
                        "name": "cisco-imc-supervisor",
                    },
                    {
                        "words": ['var g_url_version = "/iox/api/v2"'],
                        "type": "word",
                        "name": "cisco-iox",
                    },
                    {
                        "words": ['<div id="cisco_meeting_application">'],
                        "type": "word",
                        "name": "cisco-meeting-app",
                    },
                    {
                        "words": ["window.location.href = '/monitor';"],
                        "type": "word",
                        "name": "cisco-nexus-data-broker",
                    },
                    {
                        "words": [
                            '<div class="xwtproductname" >cisco prime infrastructure'
                        ],
                        "type": "word",
                        "name": "cisco-prime-infrastructure",
                    },
                    {
                        "words": ["/webacs/lib/xwt/themes/prime/prime-xwt.css"],
                        "type": "word",
                        "name": "cisco-prime-infrastructure",
                    },
                    {
                        "words": ["webacs/welcomeaction.do"],
                        "type": "word",
                        "name": "cisco-prime-infrastructure",
                    },
                    {
                        "part": "header",
                        "words": ["Server: prime"],
                        "type": "word",
                        "name": "cisco-prime-infrastructure",
                    },
                    {
                        "words": ['productname="network registrar'],
                        "type": "word",
                        "name": "cisco-prime-network-registrar",
                    },
                    {"words": ["/ccmadmin/"], "type": "word", "name": "cisco-ucm"},
                    {
                        "words": ["cisco ucs director"],
                        "type": "word",
                        "name": "cisco-ucs-director",
                    },
                    {
                        "words": ['alt="cisco webex meetings server"'],
                        "type": "word",
                        "name": "cisco-webex",
                    },
                    {
                        "words": ["/+CSCOE+/logon.html"],
                        "type": "word",
                        "name": "ciscovpn",
                    },
                    {
                        "words": ["/styles/webcit.css"],
                        "type": "word",
                        "name": "citadel-servers",
                    },
                    {
                        "words": ['<div class="boxlabel">citadel server - powered by'],
                        "type": "word",
                        "name": "citadel-servers",
                    },
                    {
                        "words": ["正在使用腾讯qq帐号登录消防联网系统"],
                        "type": "word",
                        "name": "citec-system",
                    },
                    {
                        "words": ['"/vpn/resources/{lang}"'],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /vpn/index.html"],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ['href="/vpn/images/accessgateway.ico'],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ['class="citrixreceiverlogoaboutbox"'],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ["/vpn/js/gateway_login_view.js?"],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ["cloud.ottoworkfroce.eu/vpn/index.html"],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ["vpn/js/lsgateway_login_view.js"],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ['class="_ctxstxt_netscalergateway"'],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ["receiver/images/common/icon_vpn.ico"],
                        "type": "word",
                        "name": "citrix-access-gateway",
                    },
                    {
                        "words": ['window.location="/citrix/metaframe'],
                        "type": "word",
                        "name": "citrix-metaframe",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: citrix_ns_id"],
                        "type": "word",
                        "name": "citrix-netscaler",
                    },
                    {
                        "words": ["netscape/firefox/opera"],
                        "type": "word",
                        "name": "citrix-netscaler",
                    },
                    {
                        "words": [
                            'href="clients/html5client/src/receiverthirdpartynotices.html"'
                        ],
                        "type": "word",
                        "name": "citrix-receiver",
                    },
                    {
                        "words": [
                            "logonbelt-topshadow",
                            "upgradeavailable-already-installed-separator bar-separator",
                        ],
                        "type": "word",
                        "name": "citrix-receiver",
                        "condition": "and",
                    },
                    {
                        "part": "header",
                        "words": ["Server: citrix web pn server"],
                        "type": "word",
                        "name": "citrix-web-pn-server",
                    },
                    {
                        "words": ["<p/>citrix systems, inc. xcp 1.6.10"],
                        "type": "word",
                        "name": "citrix-xcp",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: xmscookie"],
                        "type": "word",
                        "name": "citrix-xenmobile",
                    },
                    {
                        "words": ["citrix systems, inc. xenserver"],
                        "type": "word",
                        "name": "citrix-xenserver",
                    },
                    {
                        "words": [
                            '<a href="xencentersetup.exe">xencenter installer</a>'
                        ],
                        "type": "word",
                        "name": "citrix-xenserver",
                    },
                    {
                        "words": ['<div class="ciuis-body-content">'],
                        "type": "word",
                        "name": "ciuiscrm",
                    },
                    {
                        "part": "header",
                        "words": ["Server: cl-http"],
                        "type": "word",
                        "name": "cl-http",
                    },
                    {
                        "words": ['target="_blank">claroline</a>'],
                        "type": "word",
                        "name": "claroline",
                    },
                    {
                        "words": ['http://www.claroline.net" rel="copyright'],
                        "type": "word",
                        "name": "claroline",
                    },
                    {
                        "part": "header",
                        "words": ["Server: clearwell"],
                        "type": "word",
                        "name": "clearwell-e-discovery",
                    },
                    {
                        "words": ["powered by clientexec"],
                        "type": "word",
                        "name": "clientexec",
                    },
                    {
                        "words": ["<!-- clipbucket", 'content="clipbucket'],
                        "type": "word",
                        "name": "clipbucket",
                        "condition": "and",
                    },
                    {
                        "words": ["<!-- forged by clipbucket"],
                        "type": "word",
                        "name": "clipbucket",
                    },
                    {
                        "words": ['href="http://clip-bucket.com/">clipbucket'],
                        "type": "word",
                        "name": "clipbucket",
                    },
                    {
                        "words": ["<!--!!!!!!!!!!!!!!!!!!!!!!!!! processing script"],
                        "type": "word",
                        "name": "clipshare",
                    },
                    {
                        "words": ['powered by <a href="http://www.clip-share.com'],
                        "type": "word",
                        "name": "clipshare",
                    },
                    {
                        "words": [
                            'src="/design/common/his.logo.white.svg" alt="his logo'
                        ],
                        "type": "word",
                        "name": "cloodie-his",
                    },
                    {
                        "words": ['href="/design/design/cloodie.css'],
                        "type": "word",
                        "name": "cloodie-his",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: cloudera_manager_sessionid="],
                        "type": "word",
                        "name": "cloudera-manager",
                    },
                    {
                        "words": ['var loginpageurl = "/cmf/login";'],
                        "type": "word",
                        "name": "cloudera-manager",
                    },
                    {
                        "words": ['src="/companyimage/agents/sdk-logo.png"'],
                        "type": "word",
                        "name": "cloudroom-meeting",
                    },
                    {
                        "words": ["/public/vendor/monaco/vs/loader.js"],
                        "type": "word",
                        "name": "cloudwise-dodp",
                    },
                    {
                        "words": ["<font size=2>username ( contatto email )</font>"],
                        "type": "word",
                        "name": "cmailserver",
                    },
                    {
                        "words": ['<input type=checkbox name="saveuserpass'],
                        "type": "word",
                        "name": "cmailserver",
                    },
                    {
                        "words": ['href="/cms4jadmin/login.jsp'],
                        "type": "word",
                        "name": "cms4j",
                    },
                    {
                        "words": [
                            'method="post" name="cms4jsearchform" id="cms4jsearchform'
                        ],
                        "type": "word",
                        "name": "cms4j",
                    },
                    {"words": ['content="cmseasy'], "type": "word", "name": "cmseasy"},
                    {
                        "words": ["/css/cmstop-common.css"],
                        "type": "word",
                        "name": "cmstop",
                    },
                    {
                        "words": ["/js/cmstop-common.js"],
                        "type": "word",
                        "name": "cmstop",
                    },
                    {
                        "words": [
                            '<a class="poweredby" href="http://www.cmstop.com"',
                            "cmstop-list-text.css",
                        ],
                        "type": "word",
                        "name": "cmstop",
                        "condition": "and",
                    },
                    {
                        "words": ['content="开源团主机管理系统'],
                        "type": "word",
                        "name": "cmstuan",
                    },
                    {
                        "words": ["/smrc/resources/default/"],
                        "type": "word",
                        "name": "cndatacom-smsp",
                    },
                    {
                        "words": ["admin@cnoa.cn", "powered by 协众oa"],
                        "type": "word",
                        "name": "cnoa-oa",
                        "condition": "and",
                    },
                    {
                        "words": ["powered by cnoa.cn"],
                        "type": "word",
                        "name": "cnoa-oa",
                    },
                    {
                        "words": ["/oaapp/webobjects/oaapp.woa"],
                        "type": "word",
                        "name": "cnpower-oa",
                    },
                    {
                        "words": ["登录青铜器rdm</b></div>"],
                        "type": "word",
                        "name": "cnrdm-product",
                    },
                    {
                        "words": ["<!--p>cns网络核心服务自动化开通平台系统. "],
                        "type": "word",
                        "name": "cns-corenetwork-services",
                    },
                    {
                        "words": ['<span class="left">cns-app '],
                        "type": "word",
                        "name": "cns-corenetwork-services",
                    },
                    {
                        "words": ["publish by jcms2010"],
                        "type": "word",
                        "name": "cnvp-jcms",
                    },
                    {
                        "words": ['src="/extjs/adapter/ext/ext-base-js'],
                        "type": "word",
                        "name": "cnway-ilims",
                    },
                    {
                        "words": ["/js/allpagefunction.js"],
                        "type": "word",
                        "name": "cnway-ilims",
                    },
                    {
                        "words": [
                            "cockpit/static/login.js",
                            "cockpit/static/login.css",
                        ],
                        "type": "word",
                        "name": "cockpit",
                        "condition": "and",
                    },
                    {
                        "words": ['content="codepush service is hotupdate services'],
                        "type": "word",
                        "name": "codepush-server",
                    },
                    {
                        "words": [
                            "baseurl : 'app',        //配置模块根路径到静态资源根目录。"
                        ],
                        "type": "word",
                        "name": "codesafe",
                    },
                    {
                        "words": ['<param name="startvisu" value="plc_visu">'],
                        "type": "word",
                        "name": "codesys-webvisu",
                    },
                    {
                        "words": ["CodiMD", "hackmd"],
                        "type": "word",
                        "name": "codiad",
                        "condition": "and",
                    },
                    {
                        "words": ["/images/cogent.gif"],
                        "type": "word",
                        "name": "cogent-datahub",
                    },
                    {
                        "words": ["科来 版权所有 保留所有权利"],
                        "type": "word",
                        "name": "colasoft-mdp",
                    },
                    {
                        "words": [
                            '<div id="processtime" time="0.000033" >#processtime</div>'
                        ],
                        "type": "word",
                        "name": "colasoft-mdp",
                    },
                    {
                        "words": ['<meta name="colasoft"'],
                        "type": "word",
                        "name": "colasoft-network-information-comprehensive-detection-and-processing-platform",
                    },
                    {
                        "words": ["科来网络信息综合检测处理平台"],
                        "type": "word",
                        "name": "colasoft-network-information-comprehensive-detection-and-processing-platform",
                    },
                    {
                        "words": [
                            'data-i18n="[html]username">#username&nbsp;&nbsp;</td>'
                        ],
                        "type": "word",
                        "name": "colasoft-tsa",
                    },
                    {"words": ['nfr="true"'], "type": "word", "name": "colasoft-tsa"},
                    {
                        "words": ["广州协商科技有限公司"],
                        "type": "word",
                        "name": "collaborative-management-platform",
                    },
                    {
                        "words": ["/web/submitlogon2.do"],
                        "type": "word",
                        "name": "collaborative-management-platform",
                    },
                    {
                        "words": ["css/hitalklogin.css"],
                        "type": "word",
                        "name": "collaborative-management-platform",
                    },
                    {
                        "words": ["/activex/sinoccshell.cab"],
                        "type": "word",
                        "name": "collaborative-management-platform",
                    },
                    {
                        "words": ["/web/submitlogon2.do "],
                        "type": "word",
                        "name": "collaborative-management-platform",
                    },
                    {
                        "words": ["upgrade/ocx/ccdmsshell.cab#version"],
                        "type": "word",
                        "name": "collectionsystem",
                    },
                    {
                        "words": ['class="s_container_left"'],
                        "type": "word",
                        "name": "collectionsystem",
                    },
                    {
                        "words": ["checkcode.aspx"],
                        "type": "word",
                        "name": "colorfulcube-traffic-management",
                    },
                    {
                        "words": ["cmn/css/common-min.css"],
                        "type": "word",
                        "name": "comcast-business",
                    },
                    {
                        "words": ['type="application/npras'],
                        "type": "word",
                        "name": "comexe-ras",
                    },
                    {
                        "words": ['href="pic/iras.ico', 'href="pic/ras.ico'],
                        "type": "word",
                        "name": "comexe-ras",
                        "condition": "and",
                    },
                    {"words": ["科迈ras"], "type": "word", "name": "comexe-ras"},
                    {
                        "words": ['远程技术支持请求：<a href="http://www.comexe.cn'],
                        "type": "word",
                        "name": "comexe-ras",
                    },
                    {
                        "words": ['href="cmxlogin.php"'],
                        "type": "word",
                        "name": "comexe-ras",
                    },
                    {
                        "words": ['content="commonspot'],
                        "type": "word",
                        "name": "commonspot",
                    },
                    {
                        "part": "header",
                        "words": ["Server: communigatepro"],
                        "type": "word",
                        "name": "communigate-pro",
                    },
                    {
                        "part": "header",
                        "words": ["Server: virata-emweb"],
                        "type": "word",
                        "name": "conexant-emweb",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /login.action?os_destination="],
                        "type": "word",
                        "name": "confluence",
                    },
                    {
                        "words": [
                            'id="com-atlassian-confluence',
                            'name="confluence-base-url"',
                        ],
                        "type": "word",
                        "name": "confluence",
                        "condition": "and",
                    },
                    {
                        "words": [
                            "<h2 align=center>conftool conference administration"
                        ],
                        "type": "word",
                        "name": "conftool",
                    },
                    {
                        "words": [
                            "<a href='http://www.conftool.net'>conference management software"
                        ],
                        "type": "word",
                        "name": "conftool",
                    },
                    {
                        "words": ["javascripts/float.js", "vcxvcxv"],
                        "type": "word",
                        "name": "conking-schoolgroup",
                        "condition": "and",
                    },
                    {
                        "words": ["consul-ui/config/environment"],
                        "type": "word",
                        "name": "consul-hashicorp",
                    },
                    {
                        "words": ["consulhost"],
                        "type": "word",
                        "name": "consul-hashicorp",
                    },
                    {
                        "words": ["consul instance"],
                        "type": "word",
                        "name": "consul-hashicorp",
                    },
                    {
                        "words": ["www.consul.io"],
                        "type": "word",
                        "name": "consul-hashicorp",
                    },
                    {
                        "words": ['content="contentxxl'],
                        "type": "word",
                        "name": "contentxxl",
                    },
                    {
                        "words": ["/coremail/bundle/"],
                        "type": "word",
                        "name": "coremail",
                    },
                    {
                        "words": ['<a href="http://www.coremail.cn" target="_blank">'],
                        "type": "word",
                        "name": "coremail",
                    },
                    {"words": ["coremail/common"], "type": "word", "name": "coremail"},
                    {
                        "words": ["/coremail/common/"],
                        "type": "word",
                        "name": "coremail",
                    },
                    {
                        "words": ['action="/coremail/index.jsp'],
                        "type": "word",
                        "name": "coremail",
                    },
                    {
                        "words": ['fmt_logoalt: "coremail 电子邮件系统'],
                        "type": "word",
                        "name": "coremail",
                    },
                    {
                        "part": "header",
                        "words": ["Server: couchbase"],
                        "type": "word",
                        "name": "couchbase",
                    },
                    {
                        "words": [
                            "href=\"javascript:update_news('board/noticelist.jsp')"
                        ],
                        "type": "word",
                        "name": "creatsoft-safesystem",
                    },
                    {
                        "words": ['href="/corrosion/charts/water-pid.jsp'],
                        "type": "word",
                        "name": "creatsoft-system",
                    },
                    {
                        "words": ['href="/corrosion/charts/ph.jsp"'],
                        "type": "word",
                        "name": "creatsoft-system",
                    },
                    {
                        "words": ['name="source" value="clientbin/cisdasystem.xap"'],
                        "type": "word",
                        "name": "crhms-medical-insurance-decision-support-system",
                    },
                    {
                        "words": ['window.open(url, "中公网医疗信息管理系统", option'],
                        "type": "word",
                        "name": "crhms-medical-insurance-review-system",
                    },
                    {
                        "words": ["中企动力提供技术支持"],
                        "type": "word",
                        "name": "crow-force-portal-cms",
                    },
                    {"words": ["tag_adfo dis_wap"], "type": "word", "name": "cscms"},
                    {"words": ["/js/cscms.js"], "type": "word", "name": "cscms"},
                    {"words": ["/ctop/index.jsp"], "type": "word", "name": "ctop-oa"},
                    {
                        "words": ["/software/jinstall.exe"],
                        "type": "word",
                        "name": "ctop-oa",
                    },
                    {
                        "words": ['src="images/logo-ctop.gif'],
                        "type": "word",
                        "name": "ctop-oa",
                    },
                    {
                        "words": ['src="images/ctop_logo.gif'],
                        "type": "word",
                        "name": "ctop-oa",
                    },
                    {
                        "words": ["r=e&&e.__esmodule?function"],
                        "type": "word",
                        "name": "cuisec-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: cups"],
                        "type": "word",
                        "name": "cups",
                    },
                    {
                        "words": [
                            '<div class="count-down">页面在<em>5</em>秒后自动跳转至您有权限的页面</div>'
                        ],
                        "type": "word",
                        "name": "customer-service-operations-management-system",
                    },
                    {
                        "words": [
                            'href="/login/cwp_theme/original/img/ico/favicon.ico"'
                        ],
                        "type": "word",
                        "name": "cwp-controlpanel",
                    },
                    {
                        "words": [
                            'src="/login/cwp_theme/original/img/new_logo_small.png"'
                        ],
                        "type": "word",
                        "name": "cwp-controlpanel",
                    },
                    {
                        "part": "header",
                        "words": ["Server: cybozu-ws"],
                        "type": "word",
                        "name": "cybozu-garoon",
                    },
                    {
                        "words": ["D-Link Systems, Inc"],
                        "type": "word",
                        "name": "d-link",
                    },
                    {
                        "words": ["url=general/erp/login/"],
                        "type": "word",
                        "name": "dandian-crm",
                    },
                    {
                        "words": ['content="单点crm系统'],
                        "type": "word",
                        "name": "dandian-crm",
                    },
                    {
                        "part": "header",
                        "words": ["Server: darkstat"],
                        "type": "word",
                        "name": "darkstat",
                    },
                    {
                        "words": ['window.location.href="single/empmain2.aspx'],
                        "type": "word",
                        "name": "das-intellitech-c3",
                    },
                    {
                        "words": [
                            'var pagefunc968535468893538dasdaweqwertion = "timeout";'
                        ],
                        "type": "word",
                        "name": "das-usmb-",
                    },
                    {
                        "words": ['src="loadoem?path=login-logo.png'],
                        "type": "word",
                        "name": "das-usmb-",
                    },
                    {
                        "words": ['content="datalife engine'],
                        "type": "word",
                        "name": "datalife-engine",
                    },
                    {
                        "words": ["href='/scada'>datanet scada interface"],
                        "type": "word",
                        "name": "datanet",
                    },
                    {
                        "words": ['$(document).attr("title","我被修改啦.哈哈"'],
                        "type": "word",
                        "name": "daydao-system",
                    },
                    {"words": ["dayrui/statics"], "type": "word", "name": "dayrui-cms"},
                    {
                        "words": [
                            "/statics/admin/global/plugins/font-awesome/css/font-awesome.min.css"
                        ],
                        "type": "word",
                        "name": "dayrui-poscms",
                    },
                    {"words": ['content="DBShop'], "type": "word", "name": "dbshop"},
                    {"words": ['content="dbshop'], "type": "word", "name": "dbshop"},
                    {
                        "words": ["<h1>welcome to nginx on debian!</h1>"],
                        "type": "word",
                        "name": "debian",
                    },
                    {
                        "words": [
                            "<a href=http://www.dedecms.com target='_blank'>Power by DedeCms</a>",
                            "power by dedecms",
                            "http://www.dedecms.com",
                            "/templets/default/style/dedecms.css",
                            "<div><h3>dedecms error warning!</h3>",
                        ],
                        "type": "word",
                        "name": "dedecms",
                        "condition": "or",
                    },
                    {
                        "words": ["Dejavu, the missing Web UI for Elasticsearch"],
                        "type": "word",
                        "name": "dejavu",
                    },
                    {
                        "words": ['class="login_server_default">n1108p-on'],
                        "type": "word",
                        "name": "dell-n1108p-on",
                    },
                    {
                        "words": [
                            'parent.main.location = urlnonst + "?nst=" + top.menu.securitytoken'
                        ],
                        "type": "word",
                        "name": "dell-navisphere-express",
                    },
                    {
                        "words": ["emc corporation"],
                        "type": "word",
                        "name": "dell-networker-management-console",
                    },
                    {
                        "words": ['<img title="open manage" alt="open manage'],
                        "type": "word",
                        "name": "dell-open-manage",
                    },
                    {
                        "words": ['alt="openmanage"'],
                        "type": "word",
                        "name": "dell-openmanage",
                    },
                    {
                        "words": [
                            'src="/oem//data/images/logo.gif"',
                            "url=/servlet/omsalogin?msgstatus='",
                        ],
                        "type": "word",
                        "name": "dell-openmanage",
                        "condition": "and",
                    },
                    {
                        "words": ['window.top.location.href = "dell_login.html'],
                        "type": "word",
                        "name": "dell-openmanage-switch-administrator",
                    },
                    {
                        "words": ["progressgraphicnone"],
                        "type": "word",
                        "name": "dell-openmanage-switch-administrator",
                    },
                    {
                        "words": ["unisphere for sc series"],
                        "type": "word",
                        "name": "dell-unisphere",
                    },
                    {
                        "words": ["用于 sc 系列的 unisphere"],
                        "type": "word",
                        "name": "dell-unisphere",
                    },
                    {
                        "words": ['content="powered by deluxebb'],
                        "type": "word",
                        "name": "deluxebb",
                    },
                    {
                        "words": [
                            '/static/plugins/js/dialog/dialog.js" id="dialog_js"'
                        ],
                        "type": "word",
                        "name": "deshang-dsmall",
                    },
                    {"words": ['content="destoon'], "type": "word", "name": "destoon"},
                    {"words": ["destoon_moduleid"], "type": "word", "name": "destoon"},
                    {
                        "words": ["login to the flexpaper console"],
                        "type": "word",
                        "name": "devaldi-flexpaper",
                    },
                    {
                        "words": ['href="http://flexpaper.devaldi.com/plugins.htm"'],
                        "type": "word",
                        "name": "devaldi-flexpaper",
                    },
                    {"words": ["536870912"], "type": "word", "name": "dfe-scada"},
                    {
                        "words": ['src="/extcomponent/security/image/dhc.png"'],
                        "type": "word",
                        "name": "dhc-oa",
                    },
                    {
                        "words": ["浙江迪安诊断技术股份有限公司"],
                        "type": "word",
                        "name": "dian-diagnostics",
                    },
                    {
                        "words": ['src="/resources/pages/img/logo.svg"'],
                        "type": "word",
                        "name": "dian-diagnostics",
                    },
                    {"words": ["diancms_sitename"], "type": "word", "name": "diancms"},
                    {
                        "words": ["diancms_用户登陆引用"],
                        "type": "word",
                        "name": "diancms",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.dwsurvey.net" style='
                        ],
                        "type": "word",
                        "name": "diaowen-system",
                    },
                    {
                        "words": ['name="tbzcode"'],
                        "type": "word",
                        "name": "differsoft-itsystem",
                    },
                    {
                        "words": ['href="/brands/guardian/favicon.ico"'],
                        "type": "word",
                        "name": "digitalguardian-system",
                    },
                    {
                        "words": ['class="common_footer1030_textfontlink"'],
                        "type": "word",
                        "name": "digiwin-system",
                    },
                    {
                        "words": ["id='cgm' style='background-image"],
                        "type": "word",
                        "name": "dingruan-cgm",
                    },
                    {
                        "words": ['content="Discuz!', "discuz_uid"],
                        "type": "word",
                        "name": "discuz",
                        "condition": "and",
                    },
                    {"words": ['content="discuz'], "type": "word", "name": "discuz"},
                    {
                        "words": ["discuz_uid", "portal.php?mod="],
                        "type": "word",
                        "name": "discuz",
                        "condition": "and",
                    },
                    {"words": ['href="/forum.php?'], "type": "word", "name": "discuz"},
                    {"words": ['id="discuz_tips'], "type": "word", "name": "discuz"},
                    {
                        "words": ['powered by <strong><a href="http://www.discuz.net'],
                        "type": "word",
                        "name": "discuz",
                    },
                    {"words": ["disqus_thread"], "type": "word", "name": "disqus"},
                    {
                        "words": ['<span class="blue bolder">drc</span>'],
                        "type": "word",
                        "name": "distributed-regtech-collaboration-platform",
                    },
                    {"words": ["/js/diyou.js"], "type": "word", "name": "diyou-p2p"},
                    {
                        "words": ['src="/dyweb/dythemes'],
                        "type": "word",
                        "name": "diyou-p2p",
                    },
                    {
                        "words": ["__admin_media_prefix__"],
                        "type": "word",
                        "name": "django",
                    },
                    {
                        "words": ["csrfmiddlewaretoken"],
                        "type": "word",
                        "name": "django",
                    },
                    {
                        "words": ["/css/portfoliomanager/styles_display_page.css"],
                        "type": "word",
                        "name": "dmxready-portfolio-manager",
                    },
                    {
                        "words": ["rememberme_portfoliomanager"],
                        "type": "word",
                        "name": "dmxready-portfolio-manager",
                    },
                    {
                        "words": ["/cgi-bin/dna/password.cgi", "dnalims"],
                        "type": "word",
                        "name": "dnatools-dnalims",
                        "condition": "and",
                    },
                    {
                        "words": ['name="dnp_firewall_redirect'],
                        "type": "word",
                        "name": "dnp-firewall",
                    },
                    {
                        "words": ["<form name=dnp_firewall"],
                        "type": "word",
                        "name": "dnp-firewall",
                    },
                    {
                        "words": ["dnp_firewall_redirect"],
                        "type": "word",
                        "name": "dnp-firewall",
                    },
                    {
                        "words": ["powered by docebo"],
                        "type": "word",
                        "name": "docebolms",
                    },
                    {
                        "words": ['@click.prevent="login" :loading="loginpending"'],
                        "type": "word",
                        "name": "doclever",
                    },
                    {
                        "words": [
                            'href="http://www.docmail.cn/android/app/docmail.apk'
                        ],
                        "type": "word",
                        "name": "docmail-cwindow",
                    },
                    {
                        "words": ['content="北京国信冠群技术有限公司,国信冠群,邮件'],
                        "type": "word",
                        "name": "docmail-cwindow",
                    },
                    {
                        "words": ['<a href="http://www.docmail.cn" target="_blank">'],
                        "type": "word",
                        "name": "docmail-cwindow",
                    },
                    {
                        "words": ['href="/drm/template/css/login.css"'],
                        "type": "word",
                        "name": "document-security-management-system",
                    },
                    {
                        "words": ['action="/drm/login.do"'],
                        "type": "word",
                        "name": "document-security-management-system",
                    },
                    {
                        "words": ['src="/drm/encjs/barrett.js"'],
                        "type": "word",
                        "name": "document-security-management-system",
                    },
                    {
                        "words": ["powered by dokuwiki"],
                        "type": "word",
                        "name": "dokuwiki",
                    },
                    {
                        "words": ['content="dokuwiki'],
                        "type": "word",
                        "name": "dokuwiki",
                    },
                    {
                        "words": ['<div id="dokuwiki'],
                        "type": "word",
                        "name": "dokuwiki",
                    },
                    {
                        "words": ["<title>dolphinscheduler</title>"],
                        "type": "word",
                        "name": "dolphinscheduler",
                    },
                    {
                        "words": ["let node_env = 'true'"],
                        "type": "word",
                        "name": "dolphinscheduler",
                    },
                    {
                        "words": ['powered by <a href="http://dotclear.org/'],
                        "type": "word",
                        "name": "dotclear",
                    },
                    {
                        "words": ["/images/dp_icon.gif"],
                        "type": "word",
                        "name": "dotproject",
                    },
                    {"words": ['content="douphp '], "type": "word", "name": "douphp"},
                    {
                        "words": ['content="dpfax - minifaxserver '],
                        "type": "word",
                        "name": "dpfax",
                    },
                    {
                        "words": [
                            '<img src="images/dpfax-big.png" border="0" alt="dpfax'
                        ],
                        "type": "word",
                        "name": "dpfax",
                    },
                    {
                        "words": ['onsubmit="return sys_submit(this)'],
                        "type": "word",
                        "name": "dptech-umc",
                    },
                    {
                        "words": ['<p class="copyright">dradis'],
                        "type": "word",
                        "name": "dradis-framework",
                    },
                    {
                        "words": ["/studentsign/tologin.di"],
                        "type": "word",
                        "name": "drrui-cloud-office-system",
                    },
                    {
                        "words": ["/user/toupdatepasswordpage.di"],
                        "type": "word",
                        "name": "drrui-cloud-office-system",
                    },
                    {
                        "words": ["powered by drugpak"],
                        "type": "word",
                        "name": "drugpak",
                    },
                    {
                        "words": ["/dplimg/dpstyle.css"],
                        "type": "word",
                        "name": "drugpak",
                    },
                    {"words": ["/misc/drupal.js"], "type": "word", "name": "drupal"},
                    {
                        "words": [
                            'Powered by <a href="https://www.drupal.org">Drupal</a>'
                        ],
                        "type": "word",
                        "name": "drupal",
                    },
                    {"words": ["Drupal.settings"], "type": "word", "name": "drupal"},
                    {
                        "words": ["jquery.extend(drupal.settings"],
                        "type": "word",
                        "name": "drupal",
                    },
                    {
                        "words": ["/sites/default/files/"],
                        "type": "word",
                        "name": "drupal",
                    },
                    {
                        "words": ["/sites/all/modules/"],
                        "type": "word",
                        "name": "drupal",
                    },
                    {"words": ["/sites/all/themes/"], "type": "word", "name": "drupal"},
                    {
                        "part": "header",
                        "words": ["Server: drwebserver"],
                        "type": "word",
                        "name": "drwebantivirus",
                    },
                    {
                        "words": [
                            "/avdesk/includes/system/templates/images/logo_en.png"
                        ],
                        "type": "word",
                        "name": "drwebantivirus",
                    },
                    {"words": ['content="dspace'], "type": "word", "name": "dspace"},
                    {
                        "words": ['<a href="http://www.dspace.org">dspace software'],
                        "type": "word",
                        "name": "dspace",
                    },
                    {
                        "words": ['src="/dsview/images/avocent-logo.png'],
                        "type": "word",
                        "name": "dsview",
                    },
                    {
                        "words": ['href="/dsview/themes/'],
                        "type": "word",
                        "name": "dsview",
                    },
                    {
                        "part": "header",
                        "words": ["Server: avocent dsview"],
                        "type": "word",
                        "name": "dsview",
                    },
                    {
                        "words": ["/dsview/images/favicon.ico"],
                        "type": "word",
                        "name": "dsview",
                    },
                    {
                        "words": ["/dsview/protected/login.do"],
                        "type": "word",
                        "name": "dsview",
                    },
                    {
                        "words": ["window.appname = 'alidts';"],
                        "type": "word",
                        "name": "dts-operation-platform",
                    },
                    {
                        "words": ["duomicms_member", "多米"],
                        "type": "word",
                        "name": "duomicms",
                        "condition": "and",
                    },
                    {"words": ["dvwa/css/login.css"], "type": "word", "name": "dvwa"},
                    {
                        "words": ["dvwa/images/login_logo.png"],
                        "type": "word",
                        "name": "dvwa",
                    },
                    {
                        "part": "header",
                        "words": ["Server: dwhttpd"],
                        "type": "word",
                        "name": "dynaweb-httpd",
                    },
                    {
                        "words": ["dzz/system/scripts/jquery.jstree.min.js"],
                        "type": "word",
                        "name": "dzzoffice-product",
                    },
                    {
                        "words": ["dzz/scripts/dzz_min.js"],
                        "type": "word",
                        "name": "dzzoffice-product",
                    },
                    {
                        "words": ['<a href="http://www.dzzoffice.com"'],
                        "type": "word",
                        "name": "dzzoffice-product",
                    },
                    {
                        "words": ["misc.php?mod=sendmail"],
                        "type": "word",
                        "name": "dzzoffice-product",
                    },
                    {"words": ['content="e-fax '], "type": "word", "name": "e-fax"},
                    {
                        "words": ['method="post" action="/eln3_asp/login.do'],
                        "type": "word",
                        "name": "e-learning",
                    },
                    {
                        "words": [
                            "document.writeln(\"（温馨提示：此处为志高美萍分支机构联系方式，志高美萍总部联系方式请点击<a href='javascript:var"
                        ],
                        "type": "word",
                        "name": "e-link",
                    },
                    {
                        "words": [
                            "lan12-jingbian-hong",
                            "科研管理系统，北京易普拉格科技",
                        ],
                        "type": "word",
                        "name": "e-plugger-srm",
                        "condition": "and",
                    },
                    {
                        "words": ["document.writeln(companymail)"],
                        "type": "word",
                        "name": "e-soonlink-",
                    },
                    {"words": ["志高易联"], "type": "word", "name": "e-soonlink-"},
                    {
                        "words": ["reader/view_abstract.aspx"],
                        "type": "word",
                        "name": "e-tiller",
                    },
                    {"words": ["北京勤云"], "type": "word", "name": "e-tiller"},
                    {
                        "words": ["eagleeye 钉钉答疑群"],
                        "type": "word",
                        "name": "eagleeye",
                    },
                    {
                        "words": ['src="/scripts/eastsimutility.js"'],
                        "type": "word",
                        "name": "east-simulation-account",
                    },
                    {
                        "words": ['document.getelementbyid("hllogininfo").click()'],
                        "type": "word",
                        "name": "east-simulation-nettrmp",
                    },
                    {
                        "words": ["nettrmp登录界面"],
                        "type": "word",
                        "name": "east-simulation-nettrmp",
                    },
                    {
                        "words": [
                            "<span>easted vserver虚拟数据中心系统</span></a></div>"
                        ],
                        "type": "word",
                        "name": "easted-ecloud",
                    },
                    {
                        "part": "header",
                        "words": ["Server: easy file sharing web server"],
                        "type": "word",
                        "name": "easy-file-sharing-web-server",
                    },
                    {
                        "words": ['<h1 class="white">云资源管控平台</h1>'],
                        "type": "word",
                        "name": "easycloud",
                    },
                    {
                        "words": ["/vhost/view/default/style/login.css"],
                        "type": "word",
                        "name": "easypanel",
                    },
                    {
                        "words": ["/css/easyscp.login.css", "content='easyscp"],
                        "type": "word",
                        "name": "easyscp",
                        "condition": "and",
                    },
                    {
                        "words": ['generator" content="easysite'],
                        "type": "word",
                        "name": "easysite",
                    },
                    {
                        "words": ["copyright 2009 by huilan"],
                        "type": "word",
                        "name": "easysite",
                    },
                    {
                        "words": ["_desktopmodules_picturenews"],
                        "type": "word",
                        "name": "easysite",
                    },
                    {
                        "words": ["class='btn btn-ebrigade btn-lg'"],
                        "type": "word",
                        "name": "ebrigade-erp",
                    },
                    {
                        "words": ["<br>欢迎使用e-cash系统"],
                        "type": "word",
                        "name": "ecash-system",
                    },
                    {"words": ["ecwapoa"], "type": "word", "name": "ecwapoa"},
                    {
                        "words": ["<!-- /killlistable.tpl -->"],
                        "type": "word",
                        "name": "edk",
                    },
                    {
                        "words": ["powered by edusoho"],
                        "type": "word",
                        "name": "edusoho-open-source-web-classroom-",
                    },
                    {
                        "words": ['<a href = "http://www.efrontlearning.net'],
                        "type": "word",
                        "name": "efront",
                    },
                    {
                        "words": ['content="egroupware'],
                        "type": "word",
                        "name": "egroupware",
                    },
                    {
                        "words": ['id="topmask"'],
                        "type": "word",
                        "name": "eisoo-anybackup",
                    },
                    {
                        "words": ["res/libs/webuploader/webuploader.css"],
                        "type": "word",
                        "name": "eisoo-anyshare",
                    },
                    {
                        "words": ['src="/res/libs/base64.min.js"'],
                        "type": "word",
                        "name": "eisoo-anyshare",
                    },
                    {
                        "words": ["/java/ektron.js"],
                        "type": "word",
                        "name": "ektron-cms",
                    },
                    {
                        "words": ["<title>Elastic HD Dashboard</title>"],
                        "type": "word",
                        "name": "elastichd-dashboard",
                    },
                    {
                        "words": ['  "tagline" : "You Know, for Search"'],
                        "type": "word",
                        "name": "elasticsearch",
                    },
                    {
                        "words": [
                            'copyright &copy; 2003 - 2017 <a href="http://www.elite-is.com/cmhome.asp'
                        ],
                        "type": "word",
                        "name": "elite_cms",
                    },
                    {"words": ['content="elitius'], "type": "word", "name": "elitius"},
                    {
                        "words": ['target="_blank" title="affiliate'],
                        "type": "word",
                        "name": "elitius",
                    },
                    {
                        "part": "server",
                        "words": ["Mbedthis-Appweb"],
                        "type": "word",
                        "name": "embedthis-appweb",
                    },
                    {
                        "words": ['content="Emby Server"'],
                        "type": "word",
                        "name": "emby",
                    },
                    {
                        "words": ["emc-favicon.ico"],
                        "type": "word",
                        "name": "emc-dd-system-manager",
                    },
                    {
                        "words": ["dd system manager"],
                        "type": "word",
                        "name": "emc-dd-system-manager",
                    },
                    {
                        "words": ["/webtop/index.js"],
                        "type": "word",
                        "name": "emc-documentum-webtop",
                    },
                    {
                        "words": ["/webtop/webtop/theme/documentum/css/webtop.css"],
                        "type": "word",
                        "name": "emc-documentum-webtop",
                    },
                    {
                        "words": ['href="/onefs/styles/onefs.css'],
                        "type": "word",
                        "name": "emc-onefs",
                    },
                    {
                        "words": ['<script src="oemmessage.js'],
                        "type": "word",
                        "name": "emc-unisphere",
                    },
                    {
                        "words": ["emeeting dating software"],
                        "type": "word",
                        "name": "emeeting-online-dating-software",
                    },
                    {
                        "words": ["/_emeetingglobals.js"],
                        "type": "word",
                        "name": "emeeting-online-dating-software",
                    },
                    {
                        "words": ['alert("网络断连或者idu-s没有启动.'],
                        "type": "word",
                        "name": "emerson-environmentalenergymonitoringsystem",
                    },
                    {
                        "words": [
                            'href="/permasense/app/webroot/flot/css/layout.print.ie.css'
                        ],
                        "type": "word",
                        "name": "emerson-permasense",
                    },
                    {
                        "words": ['src="img/xweb-logo.png"'],
                        "type": "word",
                        "name": "emerson-xweb-evo",
                    },
                    {
                        "words": ['src="/css/images/logo_xweb_alpha.png"'],
                        "type": "word",
                        "name": "emerson-xweb-evo",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.phome.net" target="_blank"><strong>empirebak</strong>'
                        ],
                        "type": "word",
                        "name": "empirebak",
                    },
                    {
                        "words": [
                            '<div align="center">(<a href="doc.html" target="_blank">查看帝国备份王说明文档</a>)</div>'
                        ],
                        "type": "word",
                        "name": "empirebak",
                    },
                    {
                        "words": ['href="/web/movielist.rss?tag'],
                        "type": "word",
                        "name": "enigma2",
                    },
                    {"words": ["entercrm"], "type": "word", "name": "entercrm"},
                    {
                        "words": ['txtusername").focus(); //默认焦点'],
                        "type": "word",
                        "name": "enterpriseloginmanagementsystem",
                    },
                    {
                        "words": ["themes/scripts/functionjs.js"],
                        "type": "word",
                        "name": "enterpriseloginmanagementsystem",
                    },
                    {
                        "words": ["/media/images/zkeco16.ico"],
                        "type": "word",
                        "name": "entrance-guard-system",
                    },
                    {
                        "words": ['content="episerver'],
                        "type": "word",
                        "name": "episerver",
                    },
                    {
                        "words": ["/javascript/episerverscriptmanager.js"],
                        "type": "word",
                        "name": "episerver",
                    },
                    {
                        "words": ["epiware - project and document management"],
                        "type": "word",
                        "name": "epiware",
                    },
                    {"words": ['href="eqmail.ico'], "type": "word", "name": "eqmail"},
                    {
                        "words": ['<frame src="/cgi-bin/eqwebmail?empty=1'],
                        "type": "word",
                        "name": "eqmail",
                    },
                    {"words": ["cdgserver3"], "type": "word", "name": "esafenet-dlp"},
                    {
                        "words": ["generated by esotalk"],
                        "type": "word",
                        "name": "esotalk",
                    },
                    {
                        "words": ["powered by esotalk"],
                        "type": "word",
                        "name": "esotalk",
                    },
                    {"words": ["/js/esotalk.js"], "type": "word", "name": "esotalk"},
                    {"words": ["powered by espcms"], "type": "word", "name": "espcms"},
                    {"words": ["infolist_fff"], "type": "word", "name": "espcms"},
                    {
                        "words": ["/templates/default/style/tempates_div.css"],
                        "type": "word",
                        "name": "espcms",
                    },
                    {
                        "words": ["esri/discovery/admin.js"],
                        "type": "word",
                        "name": "esri-arcgis",
                    },
                    {
                        "words": ["powered by esvon"],
                        "type": "word",
                        "name": "esvon-classifieds",
                    },
                    {
                        "words": ['content="esyndicat'],
                        "type": "word",
                        "name": "esyndicat",
                    },
                    {
                        "words": [
                            '<a class="navbar-brand" href="./home">etcd viewer</a>'
                        ],
                        "type": "word",
                        "name": "etcd-viewer",
                    },
                    {
                        "part": "header",
                        "words": ["Server: ethproxy"],
                        "type": "word",
                        "name": "ethproxy",
                    },
                    {
                        "words": ["powered by eticket"],
                        "type": "word",
                        "name": "eticket",
                    },
                    {
                        "words": [
                            '<a href="http://www.eticketsupport.com" target="_blank">'
                        ],
                        "type": "word",
                        "name": "eticket",
                    },
                    {
                        "words": ["/eticket/eticket.css"],
                        "type": "word",
                        "name": "eticket",
                    },
                    {
                        "words": ['<div class="header">登录补天etl系统</div>'],
                        "type": "word",
                        "name": "etl",
                    },
                    {
                        "words": ['link.description = "亿华软件"'],
                        "type": "word",
                        "name": "euesoft-hr",
                    },
                    {
                        "words": ["eureka/css/wro.css"],
                        "type": "word",
                        "name": "eureka-server",
                    },
                    {
                        "words": ["userinfo/userfp.aspx"],
                        "type": "word",
                        "name": "eusestudy",
                    },
                    {"words": ["evercookie.js"], "type": "word", "name": "evercookie"},
                    {
                        "words": ["var ec = new evercookie();"],
                        "type": "word",
                        "name": "evercookie",
                    },
                    {
                        "words": [
                            '<img class="logo" src="/Everything.gif" alt="Everything">'
                        ],
                        "type": "word",
                        "name": "everything",
                    },
                    {
                        "words": ["/ewebeditor.htm?"],
                        "type": "word",
                        "name": "ewebeditor",
                    },
                    {
                        "words": ["ClientDownload.xgi", "NewSoft"],
                        "type": "word",
                        "name": "ewebs",
                        "condition": "and",
                    },
                    {
                        "words": ["/js/xajax05/xajax_js/xajax_core.js"],
                        "type": "word",
                        "name": "ewebs",
                    },
                    {
                        "words": [
                            "<a href='../client/ewebsclientsetup.exe'></a> </td>"
                        ],
                        "type": "word",
                        "name": "ewebs",
                    },
                    {
                        "words": ["易维平台</h1>"],
                        "type": "word",
                        "name": "ewei-plagform",
                    },
                    {
                        "words": ["/examstar_icon.ico"],
                        "type": "word",
                        "name": "examstar",
                    },
                    {
                        "words": [
                            '<div class="content-bottom-text">考试星为您提供方便、高效的考试服务</div>'
                        ],
                        "type": "word",
                        "name": "examstar",
                    },
                    {
                        "words": ['content="exponent content management system'],
                        "type": "word",
                        "name": "exponent-cms",
                    },
                    {
                        "words": ["powered by exponent cms"],
                        "type": "word",
                        "name": "exponent-cms",
                    },
                    {
                        "words": ["setcookie('extmail_username", "欢迎使用extmail"],
                        "type": "word",
                        "name": "extmail",
                        "condition": "and",
                    },
                    {"words": ["/extplorer.ico"], "type": "word", "name": "extplorer"},
                    {
                        "words": ['content="亿邮大容量电子邮件系统，反垃圾邮件网关'],
                        "type": "word",
                        "name": "eyou-anti-spam-mailbox-firewall",
                    },
                    {
                        "words": ['content="亿邮电子邮件系统'],
                        "type": "word",
                        "name": "eyou-mail-system",
                    },
                    {
                        "words": ["/tpl/login/user/images/dbg.png"],
                        "type": "word",
                        "name": "eyou-mail-system",
                    },
                    {
                        "words": [
                            "var loginssl = document.form_login.login_ssl.value;"
                        ],
                        "type": "word",
                        "name": "eyou-mail-system",
                    },
                    {
                        "words": ["powered by eyoucms"],
                        "type": "word",
                        "name": "eyoucms",
                    },
                    {
                        "words": ['name="generator" content="eyoucms'],
                        "type": "word",
                        "name": "eyoucms",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: ezsessioncookie"],
                        "type": "word",
                        "name": "ez-publish",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /defaultroot/portal.jsp?access=oa"],
                        "type": "word",
                        "name": "ezoffice",
                    },
                    {
                        "words": ['content="F5 Networks, Inc."'],
                        "type": "word",
                        "name": "f5-big-ip",
                    },
                    {
                        "words": ['content="f5 networks, inc.'],
                        "type": "word",
                        "name": "f5-bigip",
                    },
                    {
                        "words": ['class="subnav">飞视美</div>'],
                        "type": "word",
                        "name": "facemeeting-meeting",
                    },
                    {
                        "words": ['<h3 class="font-bold">opsplatform</h3>'],
                        "type": "word",
                        "name": "falcon",
                    },
                    {
                        "words": ['textarea class="form-control endpoints'],
                        "type": "word",
                        "name": "falcon",
                    },
                    {
                        "words": ['id="t1">安全、稳定、安全</div>'],
                        "type": "word",
                        "name": "falipu-iot",
                    },
                    {
                        "words": [
                            "/fangmail/cgi/index.cgi",
                            "/fangmail/default/css/em_css.css",
                        ],
                        "type": "word",
                        "name": "fangmail",
                        "condition": "and",
                    },
                    {
                        "words": ['href="http://fpexam.fangpage.com" target='],
                        "type": "word",
                        "name": "fangpage-exam",
                    },
                    {
                        "words": ["/sites/exam/statics/css/login.css"],
                        "type": "word",
                        "name": "fangpage-exam",
                    },
                    {
                        "words": ["/dwr/interface/loginservice.js"],
                        "type": "word",
                        "name": "fanpusoft-construction-work-oa",
                    },
                    {
                        "words": ["app/tpl/fanwe_1/images/lazy_loading.gif"],
                        "type": "word",
                        "name": "fanwe",
                    },
                    {
                        "words": ["index.php?ctl=article_cate"],
                        "type": "word",
                        "name": "fanwe",
                    },
                    {
                        "words": [
                            '<td><font size="-1">&nbsp;</font><p><b><font size="-1">faq admin area</font></b></td>'
                        ],
                        "type": "word",
                        "name": "faq-manager",
                    },
                    {
                        "words": [
                            '<a href="admin/">admin area</a></td></tr></table></body></html>'
                        ],
                        "type": "word",
                        "name": "faq-manager",
                    },
                    {
                        "words": ['content="faq客服机器人'],
                        "type": "word",
                        "name": "faqrobot",
                    },
                    {
                        "words": ["南京云问网络技术有限公司"],
                        "type": "word",
                        "name": "faqrobot",
                    },
                    {
                        "words": ["copyright © fastadmin.net"],
                        "type": "word",
                        "name": "fastadmin-framework",
                    },
                    {
                        "words": ['<a href="/" class="navbar-brand">fastadmin</a>'],
                        "type": "word",
                        "name": "fastadmin-framework",
                    },
                    {
                        "words": ["fastadmin.net"],
                        "type": "word",
                        "name": "fastadmin-framework",
                    },
                    {
                        "words": ["FastAdmin", "fastadmin.net"],
                        "type": "word",
                        "name": "fastadmin-framework",
                        "condition": "and",
                    },
                    {
                        "words": ["js39/flyrise.stopbackspace.js"],
                        "type": "word",
                        "name": "fe-oa",
                    },
                    {
                        "words": ['data-target="#navbar-feifeicms"'],
                        "type": "word",
                        "name": "feifeicms",
                    },
                    {
                        "words": ["/res/vendor/bootstrap-3.3.5/css/bootstrap.min.css"],
                        "type": "word",
                        "name": "femr",
                    },
                    {
                        "words": ["/res/images/login-bg-1.png"],
                        "type": "word",
                        "name": "femr",
                    },
                    {
                        "words": ['href="android/com.apsp.xnmdm-signed.apk"'],
                        "type": "word",
                        "name": "fengyunqifei-firim",
                    },
                    {"words": ['title="festos'], "type": "word", "name": "festos"},
                    {"words": ["css/festos.css"], "type": "word", "name": "festos"},
                    {
                        "part": "header",
                        "words": ["Server: fexsrv"],
                        "type": "word",
                        "name": "fex",
                    },
                    {
                        "words": ['href="mailto:fexmaster@ostc.de'],
                        "type": "word",
                        "name": "fex",
                    },
                    {
                        "words": ['"/lanproxy-config/"'],
                        "type": "word",
                        "name": "ffay-lanproxy",
                    },
                    {
                        "words": ["<!-- fcms-template head.tpl begins"],
                        "type": "word",
                        "name": "fidion-cms",
                    },
                    {
                        "words": ["/fmi/iwp/cgi?-noscript"],
                        "type": "word",
                        "name": "filemaker",
                    },
                    {
                        "part": "header",
                        "words": ["Server: filemakerpro"],
                        "type": "word",
                        "name": "filemakerpro",
                    },
                    {
                        "words": [
                            'content="the fantabulous mechanical eviltwin code machine'
                        ],
                        "type": "word",
                        "name": "filenice",
                    },
                    {
                        "words": ["filenice/filenice.js"],
                        "type": "word",
                        "name": "filenice",
                    },
                    {
                        "words": [
                            '<a href="http://www.gleamtech.com/products/filevista/web-file-manager',
                            "welcome to filevista",
                        ],
                        "type": "word",
                        "name": "filevista",
                        "condition": "and",
                    },
                    {
                        "words": ['name="dbnameddl"'],
                        "type": "word",
                        "name": "findersoft-b9erp",
                    },
                    {
                        "words": ["powered by finecms"],
                        "type": "word",
                        "name": "finecms",
                    },
                    {"words": ["dayrui@gmail.com"], "type": "word", "name": "finecms"},
                    {
                        "words": ['copyright" content="finecms'],
                        "type": "word",
                        "name": "finecms",
                    },
                    {
                        "words": ["/statics/js/dayrui.js"],
                        "type": "word",
                        "name": "finecms",
                    },
                    {
                        "words": ["=fs", "ReportServer"],
                        "type": "word",
                        "name": "finereport",
                        "condition": "and",
                    },
                    {
                        "words": ['content="finereport--web reporting tool"'],
                        "type": "word",
                        "name": "finereport",
                    },
                    {
                        "words": ["<h1>firehose_service_monitoring status</h1>"],
                        "type": "word",
                        "name": "firehose_service_monitoring",
                    },
                    {
                        "words": ["mm_preloadimages('images/bt/bt_login_b.gif"],
                        "type": "word",
                        "name": "firerp-soft",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: fesessionid"],
                        "type": "word",
                        "name": "fisheye",
                    },
                    {"words": ["fisheye "], "type": "word", "name": "fisheye"},
                    {"words": ["fisheye-16.ico"], "type": "word", "name": "fisheye"},
                    {
                        "words": ['<a class="brand" href="/">flower</a>'],
                        "type": "word",
                        "name": "flower-celery-monitoring-tool",
                    },
                    {
                        "words": ['powered by <a href="http://fluxbb.org/'],
                        "type": "word",
                        "name": "fluxbb",
                    },
                    {
                        "words": ["powered by flyspray"],
                        "type": "word",
                        "name": "flyspray",
                    },
                    {
                        "part": "header",
                        "words": ["Server: fnord"],
                        "type": "word",
                        "name": "fnord",
                    },
                    {
                        "words": ["created by dotnetcms", "for foosun"],
                        "type": "word",
                        "name": "foosun",
                        "condition": "and",
                    },
                    {
                        "words": [
                            "powered by www.foosun.net,products:foosun content manage system"
                        ],
                        "type": "word",
                        "name": "foosun",
                    },
                    {
                        "words": ["websense email security gateway"],
                        "type": "word",
                        "name": "forcepoint-websense-email-security-gateway",
                    },
                    {
                        "words": ["aboutus/magicmail.gif"],
                        "type": "word",
                        "name": "formmail",
                    },
                    {"words": ["/formmail.pl"], "type": "word", "name": "formmail"},
                    {
                        "words": [
                            'href="http://www.worldwidemart.com/scripts/formmail.shtml'
                        ],
                        "type": "word",
                        "name": "formmail",
                    },
                    {
                        "words": ['<img src="images/ensilo_logo.png'],
                        "type": "word",
                        "name": "fortinet-ensilo",
                    },
                    {
                        "words": [
                            'top.location=window.location;top.location="/remote/login";'
                        ],
                        "type": "word",
                        "name": "fortinet-fortigate",
                    },
                    {
                        "words": ["fortiguard web filtering"],
                        "type": "word",
                        "name": "fortinet-fortiguard",
                    },
                    {
                        "words": ["/xx/yy/zz/ci/mgpghgpgpfghcdpfggogfgeh"],
                        "type": "word",
                        "name": "fortinet-fortiguard",
                    },
                    {
                        "words": ["/sslvpn/portal.html", "fgt_lang"],
                        "type": "word",
                        "name": "fortinet-sslvpn",
                        "condition": "and",
                    },
                    {
                        "words": ["/newsedit/newsedit/css/login_1.css"],
                        "type": "word",
                        "name": "founder-all-media-editing-system",
                    },
                    {
                        "words": ['src="/portal/img/logo.png"'],
                        "type": "word",
                        "name": "founder-operation-management-and-decision-support-system",
                    },
                    {
                        "words": [
                            'src="/desktop/ui/custom/getimage?img=iphoneview.png"'
                        ],
                        "type": "word",
                        "name": "founder-operation-management-and-decision-support-system",
                    },
                    {
                        "words": ["imgs/fs-black-box.jpg", "fs vpn"],
                        "type": "word",
                        "name": "fourseasonsvpn",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<script src="//cdn.foxycart.com',
                            '<script src="https://cdn.foxycart.com',
                        ],
                        "type": "word",
                        "name": "foxycart",
                    },
                    {"words": ["admincp/css.css"], "type": "word", "name": "fpoll"},
                    {
                        "part": "header",
                        "words": ["Set-Cookie: freakauth"],
                        "type": "word",
                        "name": "freakauth",
                    },
                    {
                        "words": [
                            '<a title="free joomla hosting" href="http://freejoomlas.com'
                        ],
                        "type": "word",
                        "name": "freejoomlas",
                    },
                    {
                        "words": ['title="welcome to freenas'],
                        "type": "word",
                        "name": "freenas",
                    },
                    {
                        "words": ["/images/ui/freenas-logo.png"],
                        "type": "word",
                        "name": "freenas",
                    },
                    {
                        "words": ["powered by: fudforum"],
                        "type": "word",
                        "name": "fudforum",
                    },
                    {
                        "words": ["welcome to netshelter"],
                        "type": "word",
                        "name": "fujitsu-netshelter-vpn",
                    },
                    {
                        "words": ["_fj_sslvpn_login"],
                        "type": "word",
                        "name": "fujitsu-vpn",
                    },
                    {
                        "words": ['onchange="ganglia_form.submit();'],
                        "type": "word",
                        "name": "ganglia",
                    },
                    {
                        "words": ['name="ganglia_form'],
                        "type": "word",
                        "name": "ganglia",
                    },
                    {
                        "words": ["GanttLab-preview.png"],
                        "type": "word",
                        "name": "ganttlab",
                    },
                    {
                        "words": [
                            '<div id="gateone"></div>',
                            "gateone.css",
                            "gateone.js",
                        ],
                        "type": "word",
                        "name": "gate-one",
                        "condition": "and",
                    },
                    {
                        "part": "header",
                        "words": ["Server: GateOne"],
                        "type": "word",
                        "name": "gate-one",
                        "condition": "and",
                    },
                    {
                        "words": ["copyright© genie networks ltd."],
                        "type": "word",
                        "name": "genieatm",
                    },
                    {"words": ["defect 3531"], "type": "word", "name": "genieatm"},
                    {
                        "words": ['powered by <a href="http://geonode.org'],
                        "type": "word",
                        "name": "geonode",
                    },
                    {
                        "words": ['href="/catalogue/opensearch" title="geonode search'],
                        "type": "word",
                        "name": "geonode",
                    },
                    {
                        "words": ["/org.geoserver.web.geoserverbasepage/"],
                        "type": "word",
                        "name": "geoserver",
                    },
                    {
                        "words": ['class="geoserver lebeg'],
                        "type": "word",
                        "name": "geoserver",
                    },
                    {
                        "words": ["\\webapps\\geoserver"],
                        "type": "word",
                        "name": "geoserver",
                    },
                    {
                        "words": ['window.location.replace("web/");'],
                        "type": "word",
                        "name": "geoserver",
                    },
                    {"words": ["geoserver"], "type": "word", "name": "geoserver"},
                    {
                        "words": ["//smarticon.geotrust.com/si.js"],
                        "type": "word",
                        "name": "geotrust-cert",
                    },
                    {
                        "words": ["welcome to geowebcache version"],
                        "type": "word",
                        "name": "geowebcache",
                    },
                    {
                        "words": ['<a href="http://geowebcache.org">geowebcache</a>'],
                        "type": "word",
                        "name": "geowebcache",
                    },
                    {
                        "words": ['static/style/images/tou.png") no-repeat'],
                        "type": "word",
                        "name": "gerpgo-erp",
                    },
                    {
                        "words": ['content="getsimple'],
                        "type": "word",
                        "name": "getsimple-cms",
                    },
                    {
                        "words": ["powered by getsimple"],
                        "type": "word",
                        "name": "getsimple-cms",
                    },
                    {
                        "words": ['src="images/box%20akuntansi%202008.png"'],
                        "type": "word",
                        "name": "gfsoft-akuntansi-2008",
                    },
                    {"words": ["gitbook"], "type": "word", "name": "gitbook"},
                    {
                        "words": ["/assets/common/images/gitbucket.png"],
                        "type": "word",
                        "name": "gitbucket",
                    },
                    {
                        "words": ['content="Gitea - Git with a cup of tea"'],
                        "type": "word",
                        "name": "gitea",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: i_like_gitea"],
                        "type": "word",
                        "name": "gitea",
                    },
                    {
                        "words": ["gon.default_issues_tracker"],
                        "type": "word",
                        "name": "gitlab",
                    },
                    {
                        "words": ['content="gitlab community edition"'],
                        "type": "word",
                        "name": "gitlab",
                    },
                    {"words": ['content="gitlab '], "type": "word", "name": "gitlab"},
                    {
                        "words": ['<a href="https://about.gitlab.com/">about gitlab'],
                        "type": "word",
                        "name": "gitlab",
                    },
                    {
                        "words": ['class="col-sm-7 brand-holder pull-left"'],
                        "type": "word",
                        "name": "gitlab",
                    },
                    {
                        "words": ["powered by gitorious"],
                        "type": "word",
                        "name": "gitorious",
                    },
                    {"words": ["^gitstack/"], "type": "word", "name": "gitstack-code"},
                    {"words": ['content="gitweb'], "type": "word", "name": "gitweb"},
                    {"words": ["/gitweb.css"], "type": "word", "name": "gitweb"},
                    {"words": ["/gitweb.js"], "type": "word", "name": "gitweb"},
                    {
                        "part": "header",
                        "words": ["GlassFish Server Open Source Edition"],
                        "type": "word",
                        "name": "glassfish",
                    },
                    {
                        "words": ["//seal.globalsign.com/siteseal"],
                        "type": "word",
                        "name": "globalsign-cert",
                    },
                    {
                        "words": ['src="/scripts/dd_belatedpng.js'],
                        "type": "word",
                        "name": "glodon-console",
                    },
                    {
                        "words": ['url: "/console/account/logon"'],
                        "type": "word",
                        "name": "glodon-console",
                    },
                    {
                        "words": ['content="glossword'],
                        "type": "word",
                        "name": "glossword",
                    },
                    {
                        "words": ["</span>国迈安全私有云部. <span>all rights reserved"],
                        "type": "word",
                        "name": "gm-electronic-security-document-management-system",
                    },
                    {
                        "words": ["国迈安全私有云部 all rights reserved"],
                        "type": "word",
                        "name": "gm-electronic-security-document-management-system",
                    },
                    {
                        "words": ['by <a href="https://goaccess.io/">goaccess</a>'],
                        "type": "word",
                        "name": "goaccess-log",
                    },
                    {"words": ['content="gogs'], "type": "word", "name": "gogs"},
                    {
                        "words": [
                            '<a class="item" target="_blank" href="https://gogs.io/docs" rel="noreferrer">帮助</a>'
                        ],
                        "type": "word",
                        "name": "gogs",
                    },
                    {
                        "words": ['location.href="homelogin.action'],
                        "type": "word",
                        "name": "golden-dragon-card-ecard-website-query-subsystem",
                    },
                    {
                        "words": [
                            '<div class="tit_b"> 通过管理员分配的密码使用紧急入口。</div>'
                        ],
                        "type": "word",
                        "name": "goldencis-nacp",
                    },
                    {
                        "words": ["onclick=\"change('book');"],
                        "type": "word",
                        "name": "goldlib-library",
                    },
                    {
                        "words": ["/opac/periodicals"],
                        "type": "word",
                        "name": "goldlib-library",
                    },
                    {
                        "words": ['class="jp-searchtabs'],
                        "type": "word",
                        "name": "goldlib-library",
                    },
                    {
                        "words": ["图书", "金盘软件"],
                        "type": "word",
                        "name": "goldlib-library",
                        "condition": "and",
                    },
                    {
                        "words": ["speakintertscarch.aspx"],
                        "type": "word",
                        "name": "goldlibcms",
                    },
                    {
                        "words": ['value="https://mail.sinopec.com/owa/"'],
                        "type": "word",
                        "name": "good10000-tios",
                    },
                    {
                        "words": ["北京万佳信科技有限公司"],
                        "type": "word",
                        "name": "good10000-tios",
                    },
                    {
                        "words": ['option value="enterprise"'],
                        "type": "word",
                        "name": "goodway-integrated-management-information-system",
                    },
                    {
                        "words": ["是否域账户登录"],
                        "type": "word",
                        "name": "goodway-integrated-management-information-system",
                    },
                    {
                        "words": ["www.google.com/talk/service/"],
                        "type": "word",
                        "name": "google-talk-chatback",
                    },
                    {
                        "words": ['alert("菜单层数量和内容层数量不一样!")'],
                        "type": "word",
                        "name": "goonie-internet-public-ppinion-monitoring-system",
                    },
                    {
                        "words": ["网络舆情监控系统"],
                        "type": "word",
                        "name": "goonie-internet-public-ppinion-monitoring-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: gordano"],
                        "type": "word",
                        "name": "gordano-messaging-suite",
                    },
                    {
                        "words": ['href="gforum.cgi?username='],
                        "type": "word",
                        "name": "gossamer-forum",
                    },
                    {
                        "words": ['content="政企订单中心'],
                        "type": "word",
                        "name": "government-and-enterprise-order-center",
                    },
                    {
                        "words": ["Grafana", "login"],
                        "type": "word",
                        "name": "grafana",
                        "condition": "and",
                    },
                    {
                        "words": ["window.grafanabootdata = "],
                        "type": "word",
                        "name": "grafana",
                    },
                    {"words": ["grafana-app"], "type": "word", "name": "grafana"},
                    {
                        "part": "header",
                        "words": ["X-Powered-By: grandnode"],
                        "type": "word",
                        "name": "grandnode",
                    },
                    {
                        "words": [' alert("欢迎使用 【管家婆分销erp)'],
                        "type": "word",
                        "name": "grasp-erp",
                    },
                    {
                        "words": [
                            "org.graylog.plugins.pipelineprocessor.processorplugin"
                        ],
                        "type": "word",
                        "name": "graylog",
                    },
                    {
                        "words": ["/images/gree/gree.gif"],
                        "type": "word",
                        "name": "gree-logistics",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: greencms"],
                        "type": "word",
                        "name": "greencms",
                    },
                    {
                        "words": ['<a href="http://www.gridsite.org/">gridsite'],
                        "type": "word",
                        "name": "gridsite",
                    },
                    {
                        "words": ["gridsite-admin.cgi?cmd"],
                        "type": "word",
                        "name": "gridsite",
                    },
                    {
                        "words": ["powered by group-office"],
                        "type": "word",
                        "name": "group-office",
                    },
                    {
                        "words": ['"theme":"group-office",'],
                        "type": "word",
                        "name": "group-office",
                    },
                    {
                        "words": ['href="http://webmail.zmail300.cn'],
                        "type": "word",
                        "name": "growforce-email",
                    },
                    {
                        "words": ['href="/page/help/mailconfig/config/index.html'],
                        "type": "word",
                        "name": "growforce-email",
                    },
                    {
                        "part": "header",
                        "words": ["Server: gsoap"],
                        "type": "word",
                        "name": "gsoap",
                    },
                    {
                        "words": ['var title    = "预约挂号系统";'],
                        "type": "word",
                        "name": "guahao-appointmentregistrationsystem",
                    },
                    {
                        "words": ['placeholder="请输入凭据"'],
                        "type": "word",
                        "name": "gzcstec-exam",
                    },
                    {
                        "words": ["抄表器驱动tp1100m"],
                        "type": "word",
                        "name": "gzmwiccard-system",
                    },
                    {
                        "words": ["广州全息若海信息科技有限公司"],
                        "type": "word",
                        "name": "gzqxrh-system",
                    },
                    {
                        "words": ['src="/scripts/easyui/jquery.easyui.min.js"'],
                        "type": "word",
                        "name": "gzqxrh-system",
                    },
                    {
                        "words": ['style="vertical-align: middle; cursor: pointer'],
                        "type": "word",
                        "name": "gzqxrh-system",
                    },
                    {
                        "words": ["响应键盘的回车事件"],
                        "type": "word",
                        "name": "gzqxrh-system",
                    },
                    {
                        "words": ["gzsa. all rights reserved</span>"],
                        "type": "word",
                        "name": "gzsa-intranet-security",
                    },
                    {
                        "words": ["login.jsp?jsessionid="],
                        "type": "word",
                        "name": "h2-database",
                    },
                    {"words": ["welcome to h2"], "type": "word", "name": "h2-database"},
                    {"words": ['href="cas.css'], "type": "word", "name": "h3c-cas"},
                    {
                        "words": ["分布式存储管理系统 </p>"],
                        "type": "word",
                        "name": "h3c-cloud",
                    },
                    {"words": ["vstor"], "type": "word", "name": "h3c-cloud"},
                    {
                        "words": ["ER3100", "h3c.com", "login"],
                        "type": "word",
                        "name": "h3c-er3100",
                        "condition": "and",
                    },
                    {
                        "words": ["/video_record.jnlp"],
                        "type": "word",
                        "name": "h3c-hdm",
                    },
                    {
                        "words": [
                            '<iframe frameborder="0" class="framelayout" src="./page/blank.html" name="mainframe" '
                            'id="mainframe'
                        ],
                        "type": "word",
                        "name": "h3c-hdm",
                    },
                    {"words": ["/imc/login.jsf"], "type": "word", "name": "h3c-imc"},
                    {
                        "words": ["imc来宾接入自助管理系统"],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": ["login_logo_h3c.png.jsf"],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": ["com_h3c_imc_usr_usermgr_alluser_overlaydiv"],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": ['action="/imc/login.jsf'],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": [
                            'src="/imc/javax.faces.resource/images/login_logo_h3c.png.jsf?ln=primefaces-imc-new-webui'
                        ],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": ['<span class="cmn_mn_normalfont">h3c 智能管理中心'],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": [
                            "h3c &#26234;&#33021;&#31649;&#29702;&#20013;&#24515;</span>",
                            'src="/imc/faces/extensionresource/com.h3c.imc.component.util.extensionresourceloader/',
                        ],
                        "type": "word",
                        "name": "h3c-imc",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'href="/selfservice/javax.faces.resource/theme.css.xhtml?ln=primefaces-imc-classic-blue"'
                        ],
                        "type": "word",
                        "name": "h3c-imc",
                    },
                    {
                        "words": [
                            "welcome to ssl vpn</h1>",
                            "keep me signed in</span>",
                        ],
                        "type": "word",
                        "name": "h3c-ssl-vpn",
                        "condition": "and",
                    },
                    {
                        "words": ["/wnm/ssl/web/frame/login.html"],
                        "type": "word",
                        "name": "h3c-web-managerment-home",
                    },
                    {
                        "words": ["ER3200", "h3c.com", "home.asp"],
                        "type": "word",
                        "name": "h3cer3200",
                        "condition": "and",
                    },
                    {
                        "words": ["ER6300G2", "h3c.com", "login"],
                        "type": "word",
                        "name": "h3cer6300g2",
                        "condition": "and",
                    },
                    {
                        "words": ["h-ui.js", "h-ui.min.js"],
                        "type": "word",
                        "name": "h_ui",
                        "condition": "and",
                    },
                    {
                        "words": ["/css/h-ui.min.css", "html5shi.js"],
                        "type": "word",
                        "name": "h_ui",
                        "condition": "and",
                    },
                    {
                        "words": ["/css/h-ui.login.css", "/h-ui.admin/"],
                        "type": "word",
                        "name": "h_ui",
                        "condition": "and",
                    },
                    {
                        "words": ["/static/hadoop.css"],
                        "type": "word",
                        "name": "hadoop-administration",
                    },
                    {
                        "words": ['class="navbar-brand">hadoop</div>'],
                        "type": "word",
                        "name": "hadoop-administration",
                    },
                    {
                        "words": ["jhuehdfstreeglobals"],
                        "type": "word",
                        "name": "hadoop-hue",
                    },
                    {
                        "words": [
                            "hue and the hue logo are trademarks of cloudera, inc.",
                            'id="jhuenotify',
                        ],
                        "type": "word",
                        "name": "hadoop-hue",
                        "condition": "and",
                    },
                    {
                        "words": ["haidao.web.general.js"],
                        "type": "word",
                        "name": "haidaoshop",
                    },
                    {"words": ["htvos.js"], "type": "word", "name": "haitian-oa"},
                    {"words": ["images/myjs.js"], "type": "word", "name": "haitian-oa"},
                    {
                        "words": ["mshtml 8.00.6001.19298"],
                        "type": "word",
                        "name": "haitian-oa",
                    },
                    {
                        "words": ['alt="汉码软件logo'],
                        "type": "word",
                        "name": "hanmasoft",
                    },
                    {
                        "words": ['content="汉码软件'],
                        "type": "word",
                        "name": "hanmasoft",
                    },
                    {
                        "words": ['class="win_introduce'],
                        "type": "word",
                        "name": "hanming-video-conferencing",
                    },
                    {
                        "words": ["/resources/fmweb/other/js/login.js"],
                        "type": "word",
                        "name": "hanming-video-conferencing",
                    },
                    {
                        "words": ["hanna图纸服务"],
                        "type": "word",
                        "name": "hanna-drawing-service",
                    },
                    {
                        "words": ["produced by 大汉网络"],
                        "type": "word",
                        "name": "hanweb-system",
                    },
                    {
                        "words": [
                            "<a href='http://www.hanweb.com' style='display:none'>"
                        ],
                        "type": "word",
                        "name": "hanweb-system",
                    },
                    {
                        "words": ["<meta name='generator' content='大汉版通'>"],
                        "type": "word",
                        "name": "hanweb-system",
                    },
                    {
                        "words": ["<meta name='author' content='大汉网络'>"],
                        "type": "word",
                        "name": "hanweb-system",
                    },
                    {
                        "words": ["/jcms_files/jcms"],
                        "type": "word",
                        "name": "hanweb-system",
                    },
                    {
                        "words": ["showsubpage.jsp"],
                        "type": "word",
                        "name": "hanwei-cms",
                    },
                    {
                        "words": ["default value is bootstrapdialog.type_primary"],
                        "type": "word",
                        "name": "hanwei-hazardous-chemicals-enterprise-early-warning-and-prevention-system",
                    },
                    {
                        "words": [
                            "var objheight = document.documentelement.clientheight"
                        ],
                        "type": "word",
                        "name": "hanwei-hazardous-chemicals-enterprise-early-warning-and-prevention-system",
                    },
                    {
                        "words": ["系统需要.net框架2.0，请点击安装!"],
                        "type": "word",
                        "name": "hanwei-integrated-business-platform",
                    },
                    {
                        "words": [
                            'onclick="window.navigate(this.fname);enablesetup();"'
                        ],
                        "type": "word",
                        "name": "hanwei-integrated-business-platform",
                    },
                    {
                        "words": ["东营汉威石油技术开发有限公司"],
                        "type": "word",
                        "name": "hanwei-integrated-business-platform",
                    },
                    {
                        "words": ['content="microsoft visual studio .net 7.1"'],
                        "type": "word",
                        "name": "hanwei-integrated-business-platform",
                    },
                    {
                        "words": [
                            'id="loginpwdcontiner"',
                            'window.location.href="/源头数据资源管理/default/default.aspx"',
                        ],
                        "type": "word",
                        "name": "hanwei-integrated-business-platform",
                        "condition": "and",
                    },
                    {
                        "words": ['directlink = "programstartup.application"'],
                        "type": "word",
                        "name": "hanwei-integrated-business-platform",
                    },
                    {
                        "words": ['<img src="images/zshlogo.jpg" />'],
                        "type": "word",
                        "name": "hanwei-mobile-platform",
                    },
                    {
                        "words": ["信任汉威的开发证书"],
                        "type": "word",
                        "name": "hanwei-mobile-platform",
                    },
                    {
                        "words": ['background="images/centerfax0.gif"'],
                        "type": "word",
                        "name": "hanyuan-wanfa-fax-server",
                    },
                    {
                        "words": ["background-image:url(images/bgfax0.jpg);"],
                        "type": "word",
                        "name": "hanyuan-wanfa-fax-server",
                    },
                    {
                        "words": ["images/common/logina_1.gif"],
                        "type": "word",
                        "name": "haoshitong-cloud-conference",
                    },
                    {
                        "words": ['content="fsmeeting'],
                        "type": "word",
                        "name": "haoshitong-cloud-conference",
                    },
                    {
                        "words": ['type="hidden" id="app.index.configsuclogin'],
                        "type": "word",
                        "name": "haoshitong-cloud-conference",
                    },
                    {
                        "words": ["statistics report for haproxy"],
                        "type": "word",
                        "name": "haproxy-report",
                    },
                    {
                        "words": ["<title>Harbor</title>"],
                        "type": "word",
                        "name": "harbor",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=/master-status"/>'
                        ],
                        "type": "word",
                        "name": "hbase",
                    },
                    {
                        "words": ["labbuilder 实验室信息管理系统"],
                        "type": "word",
                        "name": "hbjr-labbuilder",
                    },
                    {
                        "words": ['href="/chat/sametime192x192.png'],
                        "type": "word",
                        "name": "hcl-sametime",
                    },
                    {"words": ['content="hdwiki'], "type": "word", "name": "hdwiki"},
                    {
                        "words": [
                            "http://kaiyuan.hudong.com?hf=hdwiki_copyright_kaiyuan"
                        ],
                        "type": "word",
                        "name": "hdwiki",
                    },
                    {
                        "words": ["window.location.replace('/bin/hdnet.dll/login')"],
                        "type": "word",
                        "name": "heading-e-commerce-platform",
                    },
                    {
                        "words": ["location.href='/otter'"],
                        "type": "word",
                        "name": "heading-web-server",
                    },
                    {
                        "words": ['href="/templates/everythingisok/index.css"'],
                        "type": "word",
                        "name": "hejiaoa",
                    },
                    {
                        "words": ['target="_blank">freehelpdesk.org'],
                        "type": "word",
                        "name": "help-desk-software",
                    },
                    {
                        "words": ['document.cookie="4pl07username="+arraystr'],
                        "type": "word",
                        "name": "hercules-logistics",
                    },
                    {"words": ["hesk_javascript.js"], "type": "word", "name": "hesk"},
                    {"words": ["hesk_style.css"], "type": "word", "name": "hesk"},
                    {
                        "words": ['powered by <a href="http://www.hesk.com'],
                        "type": "word",
                        "name": "hesk",
                    },
                    {
                        "words": ['powered by <a href="https://www.hesk.com'],
                        "type": "word",
                        "name": "hesk",
                    },
                    {
                        "words": ['href="/web/mainmenu/images/favicon.ico">'],
                        "type": "word",
                        "name": "hetongkj",
                    },
                    {
                        "words": ['alt="ravendb"'],
                        "type": "word",
                        "name": "hibernating-rhinos-ravendb",
                    },
                    {"words": ['content="hiki'], "type": "word", "name": "hiki"},
                    {"words": ["/hiki_base.css"], "type": "word", "name": "hiki"},
                    {
                        "words": ['by <a href="http://hikiwiki.org/'],
                        "type": "word",
                        "name": "hiki",
                    },
                    {
                        "words": ["大数据诊断工具</strong>"],
                        "type": "word",
                        "name": "hikvision-bigdatadiagnosis",
                    },
                    {
                        "words": ["嗨看云视频</p>"],
                        "type": "word",
                        "name": "hikvision-cloudvideo",
                    },
                    {
                        "words": ['window.location.href.indexof("/index/carfile/"'],
                        "type": "word",
                        "name": "hikvision-intelligentsafeguardsystems",
                    },
                    {
                        "words": ["<!--警示提示处-->"],
                        "type": "word",
                        "name": "hikvision-ivms",
                    },
                    {
                        "words": ['<h1 class="logo">安防综合管理平台</h1>'],
                        "type": "word",
                        "name": "hikvision-ivms",
                    },
                    {
                        "words": ["杭州海康威视系统技术有限公司 版权所有"],
                        "type": "word",
                        "name": "hikvision-ivms",
                    },
                    {"words": ["serviceip"], "type": "word", "name": "hikvision-ivms"},
                    {
                        "words": ['src="/portal/common/js/commonvar.js'],
                        "type": "word",
                        "name": "hikvision-ivms-8700",
                    },
                    {
                        "words": ["hikvision v2.3控件网页demo"],
                        "type": "word",
                        "name": "hikvision-v23-control",
                    },
                    {
                        "words": ["杭州海康威视数字技术股份有限公司"],
                        "type": "word",
                        "name": "hikvision-v23-control",
                    },
                    {
                        "words": ["if(m_bdvrcontrol.stoptalk())"],
                        "type": "word",
                        "name": "hikvision-v23-control",
                    },
                    {
                        "part": "header",
                        "words": ["Server: face-webs"],
                        "type": "word",
                        "name": "hikvision-videocload",
                    },
                    {
                        "words": ['href="resources/login-all.css"'],
                        "type": "word",
                        "name": "hillstone-hsa",
                    },
                    {
                        "words": ["'hillstone stoneos software version "],
                        "type": "word",
                        "name": "hillstone-stoneos",
                    },
                    {
                        "words": ["gb_root_dir", "maincontent.css"],
                        "type": "word",
                        "name": "hims-hotel-cloud-computing-service",
                        "condition": "and",
                    },
                    {
                        "words": ["hims酒店云计算服务"],
                        "type": "word",
                        "name": "hims-hotel-cloud-computing-service",
                    },
                    {
                        "words": ["images/newlogin_01.jpg"],
                        "type": "word",
                        "name": "hintsoft-pubwin2015",
                    },
                    {
                        "words": ['src="left.jpg"', 'src="up.jpg"'],
                        "type": "word",
                        "name": "hisense-business-management-platform",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<legend><img src="../../content/images/hisense.bmp" style="height:20px; '
                            'padding-left:-10px"/>webpos登录</legend>'
                        ],
                        "type": "word",
                        "name": "hisense-webpos",
                    },
                    {
                        "words": ["content/images/hisense.bmp"],
                        "type": "word",
                        "name": "hisense-webpos",
                    },
                    {
                        "words": [
                            'action="login.pl" method="post"  onsubmit="encryptpasswd()'
                        ],
                        "type": "word",
                        "name": "hispider-router",
                    },
                    {
                        "words": ["__gwt_historyframe"],
                        "type": "word",
                        "name": "hitachi-maintenance-utility",
                    },
                    {
                        "words": ['src="/cgismryset/smryset.cgi/clk"'],
                        "type": "word",
                        "name": "hitachi-virtual-storage-platform",
                    },
                    {
                        "words": ['content="hivemail'],
                        "type": "word",
                        "name": "hivemail",
                    },
                    {
                        "words": ['src="/images/hcm/copyright.gif"'],
                        "type": "word",
                        "name": "hjsoft-hcm",
                    },
                    {
                        "words": [
                            'src="/images/hcm/themes/default/login/login_banner2.png?v=12334"'
                        ],
                        "type": "word",
                        "name": "hjsoft-hcm",
                    },
                    {
                        "words": ['src="/general/sys/hjaxmanage.js"'],
                        "type": "word",
                        "name": "hjsoft-hcm",
                    },
                    {
                        "words": [
                            'href="http://www.hnjycy.com" target="_blank">沃科网<'
                        ],
                        "type": "word",
                        "name": "hnjycy",
                    },
                    {
                        "words": ['resource="title_sub"'],
                        "type": "word",
                        "name": "hollysys-mes",
                    },
                    {
                        "words": ["<h2>blog comments</h2>"],
                        "type": "word",
                        "name": "honeypot",
                    },
                    {
                        "words": ['color="black" size="5">intermec easylan'],
                        "type": "word",
                        "name": "honeywell-intermec-easylan",
                    },
                    {
                        "words": ["<title>考核评测系统</title>"],
                        "type": "word",
                        "name": "hoperun-hr",
                    },
                    {"words": ["_setHordeTitle"], "type": "word", "name": "horde"},
                    {
                        "words": ["imp: copyright 2001-2009 the horde project"],
                        "type": "word",
                        "name": "horde",
                    },
                    {
                        "words": ['name="hstapp/config/environment"'],
                        "type": "word",
                        "name": "hortonworks-smartsense-tool",
                    },
                    {
                        "words": ["医院物资供应商b2b平台"],
                        "type": "word",
                        "name": "hospital-material-supplier-b2b-platform",
                    },
                    {
                        "words": [
                            "href=./static/css/app.edb681c84a53277f9336fc297ebca96e.css"
                        ],
                        "type": "word",
                        "name": "host-security-and-management-system",
                    },
                    {
                        "words": ['powered by <a href="http://hostbillapp.com'],
                        "type": "word",
                        "name": "hostbill",
                    },
                    {"words": ["<strong>hostbill"], "type": "word", "name": "hostbill"},
                    {"words": ["/house5-style1/"], "type": "word", "name": "house5"},
                    {
                        "words": ["3com - officeconnect vpn firewall"],
                        "type": "word",
                        "name": "hp-3com-officeconnect-vpn-firewall",
                    },
                    {
                        "words": ['href="http://www.hp.com/go/ilo'],
                        "type": "word",
                        "name": "hp-ilo",
                    },
                    {"words": ["js/ilo.js"], "type": "word", "name": "hp-ilo"},
                    {
                        "words": ["sitescope login"],
                        "type": "word",
                        "name": "hp-sitescope",
                    },
                    {
                        "part": "header",
                        "words": ["Server: compaqhttpserver"],
                        "type": "word",
                        "name": "hp-system-management",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: compaq-hmmd"],
                        "type": "word",
                        "name": "hp-system-management",
                    },
                    {
                        "words": ["name='mx_hidden' src=\"common/hiddenframe.html"],
                        "type": "word",
                        "name": "hp-virtual-connect-manager",
                    },
                    {
                        "words": [
                            "id='psssdiv'",
                            "src='kss_inc/js/jquery.1.3.2.pack.js'",
                        ],
                        "type": "word",
                        "name": "hphu-system",
                        "condition": "and",
                    },
                    {
                        "words": ["<b>httpfs service</b"],
                        "type": "word",
                        "name": "httpfs",
                    },
                    {
                        "words": ['content="专业的web医学影像浏览器'],
                        "type": "word",
                        "name": "huaease-medication",
                    },
                    {
                        "words": ["mdmserver"],
                        "type": "word",
                        "name": "huawei-anyoffice",
                    },
                    {
                        "words": ['onclick="changelanguage'],
                        "type": "word",
                        "name": "huawei-anyoffice",
                    },
                    {
                        "words": ["cancelchangepwbtn"],
                        "type": "word",
                        "name": "huawei-anyoffice",
                    },
                    {
                        "words": ['<form id="jvform" action="admin/business/login'],
                        "type": "word",
                        "name": "huawei-anyoffice",
                    },
                    {
                        "words": ['src="assets/conn-service/admin/'],
                        "type": "word",
                        "name": "huawei-anyoffice",
                    },
                    {
                        "words": ["75718c9a-f029-11d1-a1ac-00c04fb6c223"],
                        "type": "word",
                        "name": "huawei-auth-server",
                    },
                    {
                        "words": [
                            "location.replace('login.action?_='+ new date().gettime());"
                        ],
                        "type": "word",
                        "name": "huawei-esight",
                    },
                    {
                        "words": ['<body onload="gotologin()">'],
                        "type": "word",
                        "name": "huawei-esight",
                    },
                    {
                        "words": ["esight_login_copy_right_font"],
                        "type": "word",
                        "name": "huawei-esight",
                    },
                    {
                        "words": ["href=/webui/default/img/logo.ico", "huawei"],
                        "type": "word",
                        "name": "huawei-fusioncloud-desktop",
                        "condition": "and",
                    },
                    {
                        "words": ["resources/themes/images/logo/favicon.ico"],
                        "type": "word",
                        "name": "huawei-fusioncompute",
                    },
                    {
                        "words": ["/omsportal/"],
                        "type": "word",
                        "name": "huawei-fusioncompute",
                    },
                    {
                        "words": ["hidden_frame.html"],
                        "type": "word",
                        "name": "huawei-inner-web",
                    },
                    {
                        "words": ["var iever = scriptenginemajorversion()"],
                        "type": "word",
                        "name": "huawei-ivs",
                    },
                    {
                        "words": ["/netopen/theme/css/inframe.css"],
                        "type": "word",
                        "name": "huawei-netopen",
                    },
                    {
                        "words": [
                            'var apppath = "/app/hw/"',
                            'href="./logo/&logo&.ico"',
                        ],
                        "type": "word",
                        "name": "huawei-ssl-vpn",
                        "condition": "and",
                    },
                    {
                        "words": ["oncompleted(hresult,perrorobject, pasynccontext)"],
                        "type": "word",
                        "name": "huawei-vpn",
                    },
                    {
                        "words": ["Script/SmcScript.js?version="],
                        "type": "word",
                        "name": "huaweismc",
                    },
                    {
                        "words": ["js.hubspot.com/analytics"],
                        "type": "word",
                        "name": "hubspot",
                    },
                    {
                        "words": ["/hwface/script/logincheck.js"],
                        "type": "word",
                        "name": "hw99-checking",
                    },
                    {
                        "words": ['content="护卫神·主机大师 前台管理登录"'],
                        "type": "word",
                        "name": "hws-host",
                    },
                    {
                        "part": "header",
                        "words": ["Server: hybrid cluster"],
                        "type": "word",
                        "name": "hybrid-cluster",
                    },
                    {
                        "words": ['src="/images/hyscm.jpg'],
                        "type": "word",
                        "name": "hycas-system",
                    },
                    {
                        "words": [',"chassis_name":'],
                        "type": "word",
                        "name": "ibm-chassis-management",
                    },
                    {
                        "part": "header",
                        "words": ["Server: ibm_cics_transaction_server"],
                        "type": "word",
                        "name": "ibm-cics-transaction-server",
                    },
                    {
                        "words": [
                            "/cgi-bin/cognos.cgi",
                            "cognos &#26159; international business machines corp",
                        ],
                        "type": "word",
                        "name": "ibm-cognos",
                        "condition": "and",
                    },
                    {
                        "part": "header",
                        "words": ["Server: goserve"],
                        "type": "word",
                        "name": "ibm-goserve",
                    },
                    {
                        "words": ['<frame src="/preloginmonitor/welcome.jsp"/>'],
                        "type": "word",
                        "name": "ibm-hmc",
                    },
                    {
                        "words": ["IBM HTTP Server", "Support"],
                        "type": "word",
                        "name": "ibm-http-server",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=/designs/imm/noscript/noscript_en.php" />'
                        ],
                        "type": "word",
                        "name": "ibm-imm",
                    },
                    {
                        "words": ["ibm.stg.inlinemessage.messagetypes.msg_critical"],
                        "type": "word",
                        "name": "ibm-imm",
                    },
                    {"words": ["/ibmdojo/"], "type": "word", "name": "ibm-imm"},
                    {
                        "words": ['action="/names.nsf?login" name="_dominoform'],
                        "type": "word",
                        "name": "ibm-lotus",
                    },
                    {"words": ["软标科技"], "type": "word", "name": "ibm-lotus"},
                    {
                        "words": ["domcfg.nsf", "login.nsf"],
                        "type": "word",
                        "name": "ibm-lotus",
                        "condition": "and",
                    },
                    {
                        "words": ["esoaisapp/login.jsp", "main.nsf"],
                        "type": "word",
                        "name": "ibm-lotus",
                        "condition": "and",
                    },
                    {
                        "part": "header",
                        "words": ["Server: lotus-domino/"],
                        "type": "word",
                        "name": "ibm-lotus-domino",
                    },
                    {
                        "words": ['alt="lotus inotes login screen'],
                        "type": "word",
                        "name": "ibm-lotus-inotes",
                    },
                    {
                        "words": ['src="sametime/avtest.js"'],
                        "type": "word",
                        "name": "ibm-lotus-sametime",
                    },
                    {
                        "words": ['href="sametime/meetingcenter-moz.css"'],
                        "type": "word",
                        "name": "ibm-lotus-sametime",
                    },
                    {
                        "words": ['class="sametimemeetingsbuttontransparent"'],
                        "type": "word",
                        "name": "ibm-lotus-sametime",
                    },
                    {
                        "words": ["sametime/themes/images/blank.gif"],
                        "type": "word",
                        "name": "ibm-lotus-sametime",
                    },
                    {
                        "words": ['<option value="merge pacs">merge pacs</option>'],
                        "type": "word",
                        "name": "ibm-merge-pacs",
                    },
                    {
                        "words": ["/platform/framework/logout/logout.action"],
                        "type": "word",
                        "name": "ibm-spectrum-computing",
                    },
                    {
                        "words": ["ssoclient_"],
                        "type": "word",
                        "name": "ibm-spectrum-computing",
                    },
                    {
                        "words": ["banner/tivoli/tv_icbanner.html"],
                        "type": "word",
                        "name": "ibm-tivoli",
                    },
                    {
                        "words": ["tivoli netview uses an open source web server"],
                        "type": "word",
                        "name": "ibm-tivoli",
                    },
                    {
                        "words": [
                            "<!--- do not translate or modify any part of the hidden parameter(s) --->"
                        ],
                        "type": "word",
                        "name": "ibm-tivoli-access-manager",
                    },
                    {
                        "words": [
                            'var warningstring = "<b>warning:</b> to maintain your login session, make sure that your '
                            'browser is configured to accept cookies.";'
                        ],
                        "type": "word",
                        "name": "ibm-tivoli-access-manager",
                    },
                    {
                        "words": [
                            'http-equiv="refresh" content="0; url=/main_login.htm"'
                        ],
                        "type": "word",
                        "name": "ibm-ts3310",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: watchfiresessionid"],
                        "type": "word",
                        "name": "ibm-watchfire",
                    },
                    {
                        "words": ["/admin-bin/webexec/wte.html"],
                        "type": "word",
                        "name": "ibm-web-traffic-express-caching-proxy",
                    },
                    {
                        "part": "header",
                        "words": ["Server: webseal"],
                        "type": "word",
                        "name": "ibm-webseal",
                    },
                    {"words": ["websphere"], "type": "word", "name": "ibm-websphere"},
                    {
                        "words": ["com.ibm.websphere.ihs.doc"],
                        "type": "word",
                        "name": "ibm-websphere",
                    },
                    {
                        "words": ['content="websphere application server'],
                        "type": "word",
                        "name": "ibm-websphere",
                    },
                    {
                        "words": ['class="oat oneui"'],
                        "type": "word",
                        "name": "ibm_openadmin_tool",
                    },
                    {
                        "words": ["author:lvzhaohua"],
                        "type": "word",
                        "name": "ibot-cloud",
                    },
                    {
                        "words": ["var img_obj = document.getelementbyid('showing');"],
                        "type": "word",
                        "name": "icall-cms",
                    },
                    {
                        "words": ['var s3_app_address="https://gyj.icbc.com.cn"'],
                        "type": "word",
                        "name": "icbc-gyj",
                    },
                    {
                        "words": ["/clipboard/zeroclipboard.min"],
                        "type": "word",
                        "name": "idcos-cloudboot",
                    },
                    {
                        "part": "header",
                        "words": ["Server: ideawebserver"],
                        "type": "word",
                        "name": "ideawebserver",
                    },
                    {
                        "words": ["copyrightpt12"],
                        "type": "word",
                        "name": "ieslab-scada",
                    },
                    {
                        "words": ["青岛积成电子有限公司"],
                        "type": "word",
                        "name": "ieslab-scada",
                    },
                    {
                        "words": [
                            'href="http://www.igenus.org/" target="_blank">',
                            "igenus webmail system",
                        ],
                        "type": "word",
                        "name": "igenus-webmail",
                        "condition": "and",
                    },
                    {
                        "words": ['content="lucky-tech iguard'],
                        "type": "word",
                        "name": "iguard-security-system",
                    },
                    {
                        "words": ['content="ikonboard'],
                        "type": "word",
                        "name": "ikonboard",
                    },
                    {
                        "words": ['powered by <a href="http://www.ikonboard.com'],
                        "type": "word",
                        "name": "ikonboard",
                    },
                    {
                        "words": [
                            "<strong>we're sorry but ikuai cloud platform doesn't "
                        ],
                        "type": "word",
                        "name": "ikuai8-cloud",
                    },
                    {
                        "words": [
                            '<iframe name="content"  src="index_middle.html" frameborder="auto'
                        ],
                        "type": "word",
                        "name": "ilas",
                    },
                    {
                        "words": [
                            '<select id="selprovince"   onchange="getcity(this.options[this.selectedindex].value)">'
                        ],
                        "type": "word",
                        "name": "ilas",
                    },
                    {
                        "words": ["logo_freeboxos"],
                        "type": "word",
                        "name": "iliad-freeboxos",
                    },
                    {
                        "words": ['content="imageview'],
                        "type": "word",
                        "name": "imageview",
                    },
                    {
                        "words": ["by jorge schrauwen"],
                        "type": "word",
                        "name": "imageview",
                    },
                    {
                        "words": ['href="http://www.blackdot.be" title="blackdot.be'],
                        "type": "word",
                        "name": "imageview",
                    },
                    {
                        "words": ['href="http://www.imgallery.zor.pl"><b>imgallery'],
                        "type": "word",
                        "name": "imgallery",
                    },
                    {
                        "words": ['content="impresspages cms'],
                        "type": "word",
                        "name": "impresspages-cms",
                    },
                    {
                        "words": ['content="druid indexer coordinator console'],
                        "type": "word",
                        "name": "indexer-coordinator",
                    },
                    {
                        "words": ["wafportal/wafportal.nocache.js"],
                        "type": "word",
                        "name": "indusguard-waf",
                    },
                    {
                        "words": ['class="influxdb-version"'],
                        "type": "word",
                        "name": "influxdata-influxdb",
                    },
                    {
                        "words": ["influxdb"],
                        "type": "word",
                        "name": "influxdata-influxdb",
                    },
                    {
                        "words": ["client_check/js/global.js"],
                        "type": "word",
                        "name": "infogo-imc",
                    },
                    {
                        "words": ["/masterview.css"],
                        "type": "word",
                        "name": "infomaster",
                    },
                    {"words": ["/masterview.js"], "type": "word", "name": "infomaster"},
                    {
                        "words": ["/masterview/mpleftnavstyle/panelbar.mpifma.css"],
                        "type": "word",
                        "name": "infomaster",
                    },
                    {
                        "words": [
                            '<input type="submit" name="cmdsubmit" value=" 登 录 " '
                            'onclick="javascript:webform_dopostbackwithoptions(new '
                            "webform_postbackoptions(&quot;cmdsubmit&quot;, &quot;&quot;, true, &quot;&quot;, "
                            '&quot;&quot;, false, false))" id="cmdsubmit" class="colorbutton'
                        ],
                        "type": "word",
                        "name": "infopro-system",
                    },
                    {
                        "words": ['action="/adminconsole/loginsubmit.do'],
                        "type": "word",
                        "name": "informatica-powercenter",
                    },
                    {
                        "part": "header",
                        "words": ["Server: informatica"],
                        "type": "word",
                        "name": "informatica-powercenter",
                    },
                    {
                        "words": ['content="informatics'],
                        "type": "word",
                        "name": "informatics-cms",
                    },
                    {
                        "words": ['placeholder="ad域账号 / 系统账号"'],
                        "type": "word",
                        "name": "information-operation-and-maintenance-support-system",
                    },
                    {
                        "words": ["ccaq_kf@unisk.cn"],
                        "type": "word",
                        "name": "information-security-integrated-management-platform",
                    },
                    {
                        "words": ["china utrust infortech co,.ltd"],
                        "type": "word",
                        "name": "infosec-utrust",
                    },
                    {
                        "words": [
                            '<div class="main_supporterbar">',
                            'class="main_loginbar',
                        ],
                        "type": "word",
                        "name": "infowarelab-system-management-center",
                        "condition": "and",
                    },
                    {
                        "words": ["images/intro/lin_bottom_nocr.gif"],
                        "type": "word",
                        "name": "innotube-manager",
                    },
                    {"words": ['id="ino-body"'], "type": "word", "name": "inoerp"},
                    {
                        "words": ["inspinia", 'name="password'],
                        "type": "word",
                        "name": "inspinia",
                        "condition": "and",
                    },
                    {
                        "words": ["onlinequery/querylist.aspx"],
                        "type": "word",
                        "name": "inspur-ec-government-approval-platform",
                    },
                    {
                        "words": ["langchao.ecgap.outportal"],
                        "type": "word",
                        "name": "inspur-ec-government-approval-platform",
                    },
                    {
                        "words": ['class="easyui-layout'],
                        "type": "word",
                        "name": "inspur-incloud-sphere",
                    },
                    {
                        "words": ["images/ewuser_title.jpg"],
                        "type": "word",
                        "name": "installationqualitymanagementsystem",
                    },
                    {
                        "words": ["rmsie = /(msie\\s|trident.*rv:)([\\w.]+)/i"],
                        "type": "word",
                        "name": "integrating-century-epbp-management-platform",
                    },
                    {
                        "words": ["match = rmsie.exec(window.navigator.useragent"],
                        "type": "word",
                        "name": "integrating-century-epbp-management-platform",
                    },
                    {
                        "words": ["厦门立智通讯科技有限公司 版权所有"],
                        "type": "word",
                        "name": "intelligence-parking-integrated-management-platform",
                    },
                    {
                        "words": ["handlexpapplycontact"],
                        "type": "word",
                        "name": "intelligent-cloud",
                    },
                    {
                        "words": ["交互式虚拟船舶展示系统</a>"],
                        "type": "word",
                        "name": "interactivevirtualshipdisplaysystem",
                    },
                    {
                        "part": "header",
                        "words": ["Server: internet cluster manager"],
                        "type": "word",
                        "name": "internet-cluster-manager",
                    },
                    {
                        "words": ['content="interred'],
                        "type": "word",
                        "name": "interred",
                    },
                    {
                        "words": ["created with interred"],
                        "type": "word",
                        "name": "interred",
                    },
                    {"words": ["ipb.vars"], "type": "word", "name": "invision-ipboard"},
                    {
                        "words": ['powered by <a href="http://www.invisionboard.com'],
                        "type": "word",
                        "name": "invision-powerboard",
                    },
                    {
                        "words": [
                            '<a href="https://www.ioa.cn/official/download.html" target="_blank">爱办公app</a>'
                        ],
                        "type": "word",
                        "name": "ioa",
                    },
                    {
                        "words": ['id="foot_version">厦门容能科技有限公司'],
                        "type": "word",
                        "name": "ioa",
                    },
                    {
                        "words": ["<!-- ipcop logo row -->"],
                        "type": "word",
                        "name": "ipcop-firewall",
                    },
                    {
                        "words": ["href='https://sourceforge.net/projects/ipcop/"],
                        "type": "word",
                        "name": "ipcop-firewall",
                    },
                    {
                        "words": ["href='http://sf.net/projects/ipcop/"],
                        "type": "word",
                        "name": "ipcop-firewall",
                    },
                    {
                        "words": ["/login/lpec/qrcode.html"],
                        "type": "word",
                        "name": "ipec-ipms",
                    },
                    {"words": ["powered by ipeer"], "type": "word", "name": "ipeer"},
                    {"words": ["/css/ipeer.css"], "type": "word", "name": "ipeer"},
                    {
                        "words": [
                            "onchange=\"is_empty('#txtusername','#lblemptyname')"
                        ],
                        "type": "word",
                        "name": "ipguard-system",
                    },
                    {
                        "words": ["myicalusername"],
                        "type": "word",
                        "name": "ipswitch-imailserver",
                    },
                    {
                        "part": "header",
                        "words": ["Server: ipswitch-imail"],
                        "type": "word",
                        "name": "ipswitch-imailserver",
                    },
                    {
                        "words": ['src="/static/img/allstar.png"'],
                        "type": "word",
                        "name": "irainone-parkingsystem",
                    },
                    {"words": ["iredadmin"], "type": "word", "name": "iredmail"},
                    {
                        "words": [
                            'powered by <a href="http://www.iscripts.com/reservelogic/'
                        ],
                        "type": "word",
                        "name": "iscripts-reservelogic",
                    },
                    {
                        "words": ["powered by: support center"],
                        "type": "word",
                        "name": "isolsoft-support-center",
                    },
                    {
                        "words": ['powered by <a href="http://www.ispconfig.org'],
                        "type": "word",
                        "name": "ispconfig",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: ispcp"],
                        "type": "word",
                        "name": "ispcp-omega",
                    },
                    {
                        "words": ["var c_name = 'jsessionid';"],
                        "type": "word",
                        "name": "isunor-order-management-system",
                    },
                    {"words": ["/enableq.css"], "type": "word", "name": "itenable"},
                    {
                        "words": ["js/checkquestion.js.php"],
                        "type": "word",
                        "name": "itenable",
                    },
                    {
                        "words": ["/images/enableq.ico"],
                        "type": "word",
                        "name": "itenable",
                    },
                    {"words": ["_skinpath"], "type": "word", "name": "iwebshop"},
                    {"words": ["_themepath"], "type": "word", "name": "iwebshop"},
                    {
                        "words": ["_weburl", 'class="pro_title">iwebshop支付测试'],
                        "type": "word",
                        "name": "iwebshop",
                        "condition": "and",
                    },
                    {
                        "words": ["/jooyea/images/sns_idea1.jpg"],
                        "type": "word",
                        "name": "iwebsns",
                    },
                    {
                        "words": ["/jooyea/images/snslogo.gif"],
                        "type": "word",
                        "name": "iwebsns",
                    },
                    {
                        "part": "header",
                        "words": ["Server: jabberd"],
                        "type": "word",
                        "name": "jabberd",
                    },
                    {
                        "words": ['alt="the jakarta project'],
                        "type": "word",
                        "name": "jakarta-project",
                    },
                    {
                        "words": ['<a href="http://jakarta.apache.org/">'],
                        "type": "word",
                        "name": "jakarta-project",
                    },
                    {
                        "words": ['powered by <a href="http://www.jasig.org/cas'],
                        "type": "word",
                        "name": "jasig-cas",
                    },
                    {"words": ["易族智汇javashop"], "type": "word", "name": "javashop"},
                    {
                        "words": ['<a href="http://jboss.org">'],
                        "type": "word",
                        "name": "jboss",
                    },
                    {"words": ["jboss.css"], "type": "word", "name": "jboss"},
                    {
                        "words": ["manage this jboss as instance"],
                        "type": "word",
                        "name": "jboss-as",
                    },
                    {
                        "words": [
                            "<h3>your jboss enterprise application platform is running.</h3>"
                        ],
                        "type": "word",
                        "name": "jboss-eap",
                    },
                    {
                        "words": ["JeecgBoot", "polyfill_"],
                        "type": "word",
                        "name": "jeecgboot",
                        "condition": "and",
                    },
                    {
                        "words": ['"/cnf/r/cms/common.js"'],
                        "type": "word",
                        "name": "jeecms",
                    },
                    {"words": ["jeesite.css"], "type": "word", "name": "jeesite"},
                    {"words": ["jeesite.js"], "type": "word", "name": "jeesite"},
                    {"words": ["jeesite.com"], "type": "word", "name": "jeesite"},
                    {
                        "part": "header",
                        "words": ["Set-Cookie: jeesite.session.id="],
                        "type": "word",
                        "name": "jeesite",
                    },
                    {
                        "words": ["<title>Jellyfin</title>", 'content="Jellyfin"'],
                        "type": "word",
                        "name": "jellyfin",
                        "condition": "and",
                    },
                    {
                        "words": ["hudson.model.Hudson.Administer"],
                        "type": "word",
                        "name": "jenkins",
                    },
                    {
                        "words": ["jenkins-agent-protocols"],
                        "type": "word",
                        "name": "jenkins",
                    },
                    {"words": ["[Jenkins]"], "type": "word", "name": "jenkins"},
                    {
                        "words": ["jh-la3600"],
                        "type": "word",
                        "name": "jianhengxinan-jh-las",
                    },
                    {
                        "words": ["建恒信安日志审计系统"],
                        "type": "word",
                        "name": "jianhengxinan-jh-las",
                    },
                    {
                        "words": ['content="jiaozhichu'],
                        "type": "word",
                        "name": "jiaozhichu-online-test-system",
                    },
                    {
                        "words": ['href="/ksxt/h5/images/jiaozhichu.ico'],
                        "type": "word",
                        "name": "jiaozhichu-online-test-system",
                    },
                    {
                        "words": ["tougao/misc.js"],
                        "type": "word",
                        "name": "jienohan-journal",
                    },
                    {
                        "part": "header",
                        "words": ["Server: jigsaw"],
                        "type": "word",
                        "name": "jigsaw",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /secure/SetupMode!default.jspa"],
                        "type": "word",
                        "name": "jira",
                    },
                    {"words": ["jira.webresources"], "type": "word", "name": "jira"},
                    {"words": ["ams-build-number"], "type": "word", "name": "jira"},
                    {"words": ["com.atlassian.jira"], "type": "word", "name": "jira"},
                    {
                        "words": [
                            "location.href='/cgi-bin/cgiproxy.exe?action=start';"
                        ],
                        "type": "word",
                        "name": "jit-web-connector",
                    },
                    {"words": ["九思软件"], "type": "word", "name": "jiusi-oa"},
                    {"words": ["/jive-icons.css"], "type": "word", "name": "jive-sbs"},
                    {
                        "words": ['class="jive-body-formpage'],
                        "type": "word",
                        "name": "jive-sbs",
                    },
                    {
                        "words": ["logintable", "selectcss", "toptitleimg"],
                        "type": "word",
                        "name": "jloa",
                        "condition": "and",
                    },
                    {
                        "words": ["jlwcs", "京伦建站系统 "],
                        "type": "word",
                        "name": "jltech",
                        "condition": "and",
                    },
                    {
                        "words": ['src="../../doc/config/shxmjgptapp.png"'],
                        "type": "word",
                        "name": "jnsh-system",
                    },
                    {
                        "words": ["北京久其软件股份有限公司 版权所有"],
                        "type": "word",
                        "name": "join-cheer-general-financial-system",
                    },
                    {
                        "words": ["/netrep/intf", "/netrep/message2/"],
                        "type": "word",
                        "name": "join-cheer-general-financial-system",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0";url="../netrep">'
                        ],
                        "type": "word",
                        "name": "join-cheer-general-financial-system",
                    },
                    {
                        "words": ["../netrep", "jqci"],
                        "type": "word",
                        "name": "join_cheer-report",
                        "condition": "and",
                    },
                    {
                        "words": ["<h1>富通天下erp</h1>"],
                        "type": "word",
                        "name": "joinf-erp",
                    },
                    {
                        "words": ['<meta name="keywords" content="joomla, Joomla" />'],
                        "type": "word",
                        "name": "joomla",
                    },
                    {"words": ['content="joomla'], "type": "word", "name": "joomla"},
                    {
                        "words": ["/media/system/js/core.js"],
                        "type": "word",
                        "name": "joomla",
                    },
                    {
                        "words": ["/media/system/js/mootools-core.js"],
                        "type": "word",
                        "name": "joomla",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: jsf"],
                        "type": "word",
                        "name": "jsf",
                    },
                    {
                        "words": ["- Powered by Jspxcms", "template/"],
                        "type": "word",
                        "name": "jspxcms",
                        "condition": "and",
                    },
                    {"words": ['content="jstorm'], "type": "word", "name": "jstorm"},
                    {
                        "words": ['content="仪化产品质量查询系统"'],
                        "type": "word",
                        "name": "jsyhit-system",
                    },
                    {
                        "words": ['action="login.aspx" id="ctl00"'],
                        "type": "word",
                        "name": "juhe-uams",
                    },
                    {
                        "words": ["background-color: #4a93be;"],
                        "type": "word",
                        "name": "juhe-uams",
                    },
                    {
                        "words": [
                            "JumpServer 开源堡垒机",
                            "JumpServer Open Source Bastion Host",
                        ],
                        "type": "word",
                        "name": "jumpserver",
                        "condition": "or",
                    },
                    {
                        "words": [
                            '<input type="password" class="form-control" name="password" placeholder="密码" required="">',
                            "csrfmiddlewaretoken",
                        ],
                        "type": "word",
                        "name": "jumpserver-fortres-machine",
                        "condition": "and",
                    },
                    {
                        "words": ["/stylesheet/juniper.css"],
                        "type": "word",
                        "name": "juniper-hdr",
                    },
                    {"words": ["/hdr_logo.gif"], "type": "word", "name": "juniper-hdr"},
                    {
                        "words": [
                            "juniper networks vpn",
                            "junos pulse secure access service",
                        ],
                        "type": "word",
                        "name": "juniper-vpn",
                        "condition": "and",
                    },
                    {
                        "words": ['<div id="ipython-main-app" class="container">'],
                        "type": "word",
                        "name": "jupyter-notebook",
                    },
                    {
                        "words": [
                            '<div id="ipython_notebook" class="nav navbar-brand">'
                        ],
                        "type": "word",
                        "name": "jupyter-notebook",
                    },
                    {
                        "words": ["<title>Jupyter Notebook</title>"],
                        "type": "word",
                        "name": "jupyter-notebook",
                    },
                    {
                        "words": [
                            "var _usermenusurl = '/appcenter/functions/getusermenu'"
                        ],
                        "type": "word",
                        "name": "jurassic-application-management-system",
                    },
                    {
                        "words": ["jurassic"],
                        "type": "word",
                        "name": "jurassic-application-management-system",
                    },
                    {
                        "words": ['id="jxt-popup-wrapper'],
                        "type": "word",
                        "name": "jxt-consulting",
                    },
                    {
                        "words": ["powered by jxt consulting"],
                        "type": "word",
                        "name": "jxt-consulting",
                    },
                    {
                        "words": ['content="jymusic音乐管理系统'],
                        "type": "word",
                        "name": "jymusic",
                    },
                    {
                        "words": ["public/static/libs/jymusic/css"],
                        "type": "word",
                        "name": "jymusic",
                    },
                    {
                        "words": [
                            "vassets/images/2af62f58ee2baf495c9b3a9a1c30ce03-favicon.png"
                        ],
                        "type": "word",
                        "name": "kafka-manager",
                    },
                    {
                        "words": ["css/cluster-viz.css"],
                        "type": "word",
                        "name": "kafkaoffsetmonitor",
                    },
                    {"words": ["powered by kaibb"], "type": "word", "name": "kaibb"},
                    {
                        "words": ['content="forum powered by kaibb'],
                        "type": "word",
                        "name": "kaibb",
                    },
                    {
                        "words": ['src="/kjoa/sheep/login/login.js'],
                        "type": "word",
                        "name": "kaijiang-oa",
                    },
                    {
                        "part": "header",
                        "words": ["Server: kamailio"],
                        "type": "word",
                        "name": "kamailio-sip-server",
                    },
                    {
                        "words": ["http://cf.kampyle.com/k_button.js"],
                        "type": "word",
                        "name": "kampyle",
                    },
                    {
                        "words": ["start kampyle feedback form button"],
                        "type": "word",
                        "name": "kampyle",
                    },
                    {
                        "part": "header",
                        "words": ["Server: karrigell"],
                        "type": "word",
                        "name": "karrigell",
                    },
                    {
                        "words": ["kaspersky secure mail gateway"],
                        "type": "word",
                        "name": "kaspersky-secure-mail-gateway",
                    },
                    {
                        "words": ["powered by kayako esupport"],
                        "type": "word",
                        "name": "kayako-supportsuite",
                    },
                    {
                        "words": ["help desk software by kayako esupport"],
                        "type": "word",
                        "name": "kayako-supportsuite",
                    },
                    {
                        "part": "header",
                        "words": ["Server: keil-eweb"],
                        "type": "word",
                        "name": "keil-embedded-web-server",
                    },
                    {
                        "words": ['href="/favicon.ico?kenna"'],
                        "type": "word",
                        "name": "kenna-system",
                    },
                    {
                        "words": ['class="kenna sessions new"'],
                        "type": "word",
                        "name": "kenna-system",
                    },
                    {
                        "words": ["/cmspages/getresource.ashx", 'content="kentico cms'],
                        "type": "word",
                        "name": "kentico-cms",
                        "condition": "and",
                    },
                    {"words": ["kentico"], "type": "word", "name": "kentico-cms"},
                    {
                        "part": "header",
                        "words": ["Server: kerio connect"],
                        "type": "word",
                        "name": "kerio-connect",
                    },
                    {
                        "part": "header",
                        "words": ["Server: kerio mailserver"],
                        "type": "word",
                        "name": "kerio-mailserver",
                    },
                    {
                        "part": "header",
                        "words": ["Server: kerio_webstar"],
                        "type": "word",
                        "name": "kerio-webstar",
                    },
                    {
                        "part": "header",
                        "words": ["Server: webstar"],
                        "type": "word",
                        "name": "kerio-webstar",
                    },
                    {
                        "part": "header",
                        "words": ["Server: kfwebserver"],
                        "type": "word",
                        "name": "keyfocus-webserver",
                    },
                    {
                        "part": "header",
                        "words": ["kbn-name: kibana"],
                        "type": "word",
                        "name": "kibana",
                    },
                    {"words": ["kindeditor.js"], "type": "word", "name": "kindeditor"},
                    {
                        "words": ["kindeditor.ready"],
                        "type": "word",
                        "name": "kindeditor",
                    },
                    {
                        "words": ["k.create", "kindeditor-min.js"],
                        "type": "word",
                        "name": "kindeditor",
                        "condition": "and",
                    },
                    {
                        "words": ["Kingdee.EntryRole", "loginKDLogo"],
                        "type": "word",
                        "name": "kingdee",
                        "condition": "and",
                    },
                    {
                        "words": ["金蝶国际软件集团有限公司版权所有"],
                        "type": "word",
                        "name": "kingdee",
                    },
                    {
                        "words": ['var formidafterlogin = \'"bos_mainconsolesutra"'],
                        "type": "word",
                        "name": "kingdee",
                    },
                    {
                        "words": ['class="kd-div-loading-ct"'],
                        "type": "word",
                        "name": "kingdee",
                    },
                    {"words": ["logo-kingdee.png"], "type": "word", "name": "kingdee"},
                    {"words": ["eassessionid"], "type": "word", "name": "kingdee"},
                    {"words": ["/eassso/common"], "type": "word", "name": "kingdee"},
                    {
                        "words": ["setkingoencypt.jsp"],
                        "type": "word",
                        "name": "kingosoft",
                    },
                    {
                        "words": ["/jkingo.js", "kingosoft"],
                        "type": "word",
                        "name": "kingosoft",
                        "condition": "and",
                    },
                    {"words": ["青果"], "type": "word", "name": "kingosoft"},
                    {
                        "words": ['class="title">关于全网部署金山毒霸企业版'],
                        "type": "word",
                        "name": "kingsoft-duba-enterprise",
                    },
                    {"words": ["kissy.ready"], "type": "word", "name": "kissy"},
                    {"words": ["kissy-min.js"], "type": "word", "name": "kissy"},
                    {"words": ["kissy.js"], "type": "word", "name": "kissy"},
                    {"words": ["powered by kleeja"], "type": "word", "name": "kleeja"},
                    {
                        "words": ['src="/img/hypervm-logo.gif'],
                        "type": "word",
                        "name": "kloxo-single-server",
                    },
                    {
                        "words": ["/htmllib/js/preop.js"],
                        "type": "word",
                        "name": "kloxo-single-server",
                    },
                    {
                        "part": "header",
                        "words": ["Server: the knopflerfish http server"],
                        "type": "word",
                        "name": "knopflerfish-http-server",
                    },
                    {
                        "part": "header",
                        "words": ["Server: koala web server"],
                        "type": "word",
                        "name": "koala-web-server",
                    },
                    {
                        "words": ["/common/loading_simple.gif"],
                        "type": "word",
                        "name": "kodcloud-system",
                    },
                    {"words": ['content="koha '], "type": "word", "name": "koha"},
                    {
                        "words": ['name="koha_login_context'],
                        "type": "word",
                        "name": "koha",
                    },
                    {"words": ['content="kolab'], "type": "word", "name": "kolab"},
                    {"words": [">kordil edms"], "type": "word", "name": "kordil-edms"},
                    {
                        "words": ["欢迎使用 kouton ctbs advanced web client 系统!"],
                        "type": "word",
                        "name": "kouton-ctbs",
                    },
                    {
                        "words": ['src="resource/javascript/jkpm6.datetime.js'],
                        "type": "word",
                        "name": "kuaipu-m6",
                    },
                    {
                        "words": ['content="北京开维创科技有限公司'],
                        "type": "word",
                        "name": "kwcnet-vis",
                    },
                    {
                        "words": ['powered by <a href="http://www.kxmail.net'],
                        "type": "word",
                        "name": "kxmail",
                    },
                    {
                        "words": ["/systemfunction.pack.js"],
                        "type": "word",
                        "name": "kxmail",
                    },
                    {"words": ["lo_computername"], "type": "word", "name": "kxmail"},
                    {
                        "words": [
                            "<meta name='author' content='http://www.kyanmedia.com'>"
                        ],
                        "type": "word",
                        "name": "kyan-design",
                    },
                    {"words": ["goodbook-tabcont"], "type": "word", "name": "kyxscms"},
                    {
                        "words": ['<script src="/public/static/layer/layer.js">'],
                        "type": "word",
                        "name": "kyxscms",
                    },
                    {
                        "words": ['href="/template/home/default_web/css/style.css"'],
                        "type": "word",
                        "name": "kyxscms",
                    },
                    {
                        "part": "header",
                        "words": ["Server: labview"],
                        "type": "word",
                        "name": "labview",
                    },
                    {"words": ["landmark"], "type": "word", "name": "landmark-dus"},
                    {
                        "words": ['href="/landmark.admin.web_deploy/'],
                        "type": "word",
                        "name": "landmark-dus",
                    },
                    {
                        "words": ["<strong>恭喜", "LANMP", "wdlinux.cn", "本页可删除"],
                        "type": "word",
                        "name": "lanmp",
                        "condition": "and",
                    },
                    {
                        "words": ["<font>兰眼下一代威胁感知系统</font>"],
                        "type": "word",
                        "name": "lanyeye",
                    },
                    {
                        "words": ["/skin/admin/img/login/laneye.png"],
                        "type": "word",
                        "name": "lanyeye",
                    },
                    {
                        "words": ["/public/js/util/xg_oyang.js"],
                        "type": "word",
                        "name": "laobanmail-visualhost",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: laravel_session="],
                        "type": "word",
                        "name": "laravel",
                    },
                    {
                        "words": ["vendor/laravel-admin/"],
                        "type": "word",
                        "name": "laravel-admin",
                    },
                    {
                        "words": ["欢迎登录laravel-admin</p>"],
                        "type": "word",
                        "name": "laravel-admin",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: laravel_session"],
                        "type": "word",
                        "name": "laravel-framework",
                    },
                    {
                        "part": "header",
                        "words": ["Server: lasso"],
                        "type": "word",
                        "name": "lasso-web-data-engine",
                    },
                    {"words": ['title="laysns"'], "type": "word", "name": "laysns-cms"},
                    {
                        "words": ['href="http://www.laysns.com/"> laysns'],
                        "type": "word",
                        "name": "laysns-cms",
                    },
                    {
                        "words": ["<title>secondary_nodes</title>"],
                        "type": "word",
                        "name": "lc000-system",
                    },
                    {
                        "words": ['name="author" content="leanote,蚂蚁笔记"'],
                        "type": "word",
                        "name": "leanote-notepad",
                    },
                    {
                        "words": ['content="力软敏捷开发框架，是一个web可视化开发平台'],
                        "type": "word",
                        "name": "learun-system",
                    },
                    {
                        "words": ["j_setcon j_sub_new j_padt30 j_padb30"],
                        "type": "word",
                        "name": "led-control-software",
                    },
                    {
                        "words": ["<!-- 记录当前电视墙的序号 end-->"],
                        "type": "word",
                        "name": "led-control-software",
                    },
                    {
                        "words": ["lemis.web_app_name"],
                        "type": "word",
                        "name": "lemis-management-system",
                    },
                    {
                        "words": ["client/android/bin/lenovobox.apk"],
                        "type": "word",
                        "name": "lenovo-enterprise-network-disk",
                    },
                    {
                        "words": ['alt="联想企业网盘android客户端下载"'],
                        "type": "word",
                        "name": "lenovo-enterprise-network-disk",
                    },
                    {
                        "words": ["thinkserver"],
                        "type": "word",
                        "name": "lenovo-thinkserver",
                    },
                    {
                        "words": ["language/switchover\"+'/'+current_language", "登录"],
                        "type": "word",
                        "name": "lepus",
                        "condition": "and",
                    },
                    {
                        "words": ["letodms free document management system"],
                        "type": "word",
                        "name": "letodms",
                    },
                    {
                        "words": [
                            'action="/datacenter/authentication/login.do" method="post'
                        ],
                        "type": "word",
                        "name": "lezhixing",
                    },
                    {
                        "words": [
                            'location.href=contextpath+"/login/password/password.jsp'
                        ],
                        "type": "word",
                        "name": "lezhixing",
                    },
                    {
                        "words": [
                            "/static/thirdparty/jquery/",
                            'var contextpath = "/datacenter',
                        ],
                        "type": "word",
                        "name": "lezhixing",
                        "condition": "and",
                    },
                    {"words": ["/lfradius/"], "type": "word", "name": "lflflf"},
                    {
                        "words": ['src="/ssw/js/user/index.js"'],
                        "type": "word",
                        "name": "lg-supersign",
                    },
                    {
                        "words": ['<div class="header_opac_logo">'],
                        "type": "word",
                        "name": "libsys-library",
                    },
                    {
                        "part": "header",
                        "words": ["Server: libwww-perl-daemon"],
                        "type": "word",
                        "name": "libwww-perl-daemon",
                    },
                    {
                        "words": ["powered by liferay portal"],
                        "type": "word",
                        "name": "liferay",
                    },
                    {"words": ["liferay.aui"], "type": "word", "name": "liferay"},
                    {
                        "words": ["liferay.currenturl"],
                        "type": "word",
                        "name": "liferay",
                    },
                    {
                        "words": ["/lifesizecontrol/asp/index.html"],
                        "type": "word",
                        "name": "lifesize-control",
                    },
                    {
                        "words": ['content="lifetype'],
                        "type": "word",
                        "name": "lifetype",
                    },
                    {
                        "words": ['title="install lifetype'],
                        "type": "word",
                        "name": "lifetype",
                    },
                    {
                        "words": [
                            'content="limesurvey',
                            'href="http://www.limesurvey.org" target="_blank',
                        ],
                        "type": "word",
                        "name": "lime-survey",
                        "condition": "and",
                    },
                    {
                        "words": ["powered by the listserv email list manager"],
                        "type": "word",
                        "name": "listserv",
                    },
                    {
                        "words": ["<title>welcome to listserv"],
                        "type": "word",
                        "name": "listserv",
                    },
                    {
                        "words": ["thank you for using livezilla!"],
                        "type": "word",
                        "name": "livezilla",
                    },
                    {
                        "words": ['content="livezilla'],
                        "type": "word",
                        "name": "livezilla",
                    },
                    {
                        "words": ['href="http://www.livezilla.net" target="_blank'],
                        "type": "word",
                        "name": "livezilla",
                    },
                    {
                        "words": [
                            "livezilla is a registered trademark of livezilla gmbh</div>"
                        ],
                        "type": "word",
                        "name": "livezilla",
                    },
                    {
                        "words": ["/lksys_windowcontrolscript.js"],
                        "type": "word",
                        "name": "lkpoweroa",
                    },
                    {
                        "words": ['onload="lksys_pubmaxwin()'],
                        "type": "word",
                        "name": "lkpoweroa",
                    },
                    {
                        "words": [
                            "id=\"lkblogin\" href=\"javascript:__dopostback('lkblogin','')"
                        ],
                        "type": "word",
                        "name": "lkpoweroa",
                    },
                    {
                        "words": ["identityvalidator"],
                        "type": "word",
                        "name": "lkpoweroa",
                    },
                    {"words": ["hhctrlmax"], "type": "word", "name": "lkpoweroa"},
                    {"words": ["lnmp一键安装包"], "type": "word", "name": "lnmp"},
                    {
                        "words": [
                            "Locomoco WebServer - <span style='color:blue'>RUN ( SUCCESS )"
                        ],
                        "type": "word",
                        "name": "locomoco",
                    },
                    {
                        "words": ["window.location = '/ui/modules/login/'"],
                        "type": "word",
                        "name": "logsign-siem",
                    },
                    {
                        "words": ['content="loopup"', "machine:"],
                        "type": "word",
                        "name": "loopup-meeting",
                        "condition": "and",
                    },
                    {
                        "words": ['src="http://gate.looyu.com/'],
                        "type": "word",
                        "name": "looyu-oms",
                    },
                    {
                        "words": ["北京华夏创新科技有限公司"],
                        "type": "word",
                        "name": "lotwan-web-accelerator",
                    },
                    {
                        "words": ["/loyaa/common.lib.js"],
                        "type": "word",
                        "name": "loyaa-information-automatic-editing-system",
                    },
                    {
                        "words": ['href="/eproc/assets/application.css'],
                        "type": "word",
                        "name": "lpse",
                    },
                    {
                        "words": ["/luci-static/openwrt.org/cascade.css"],
                        "type": "word",
                        "name": "luci",
                    },
                    {
                        "words": ["luci - lua configuration interface"],
                        "type": "word",
                        "name": "luci",
                    },
                    {"words": ["powered by luci"], "type": "word", "name": "luci"},
                    {
                        "words": ['action="/cgi-bin/luci">'],
                        "type": "word",
                        "name": "luci",
                    },
                    {
                        "words": ['href="/cgi-bin/luci"></a>'],
                        "type": "word",
                        "name": "luci",
                    },
                    {
                        "words": [
                            '<head> <meta http-equiv="refresh" content="0; url=/cgi-bin/luci" /> </head>'
                        ],
                        "type": "word",
                        "name": "luci",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: lussumo vanilla"],
                        "type": "word",
                        "name": "lussumo-vanilla",
                    },
                    {"words": ['content="luxcal'], "type": "word", "name": "luxcal"},
                    {"words": ["class='footlb'>lux"], "type": "word", "name": "luxcal"},
                    {
                        "words": ['content="roel buining'],
                        "type": "word",
                        "name": "luxcal",
                    },
                    {
                        "part": "header",
                        "words": ["Server: rdserver"],
                        "type": "word",
                        "name": "m2soft-rdserver",
                    },
                    {
                        "part": "header",
                        "words": ["Server: machttp"],
                        "type": "word",
                        "name": "machttp",
                    },
                    {
                        "words": ["/aboutus/magicmail.gif"],
                        "type": "word",
                        "name": "magicmail",
                    },
                    {
                        "words": ["journalx/authorlogon.action?mag_id"],
                        "type": "word",
                        "name": "magtech-journalx",
                    },
                    {
                        "words": ["this site is powered by mahara"],
                        "type": "word",
                        "name": "mahara",
                    },
                    {
                        "words": ['id="powered-by"><a href="http://mahara.org/'],
                        "type": "word",
                        "name": "mahara",
                    },
                    {
                        "words": ['{"namedfieldempty":'],
                        "type": "word",
                        "name": "mahara",
                    },
                    {
                        "words": ["<!-- loginpanel_shell_table -->"],
                        "type": "word",
                        "name": "mailenable",
                    },
                    {
                        "words": ["/jdwm/cgi/login.cgi?login"],
                        "type": "word",
                        "name": "mailer",
                    },
                    {
                        "words": ["window.open('http://www.hechen.com')"],
                        "type": "word",
                        "name": "mailgard-webmail",
                    },
                    {
                        "words": ["/images/mailman", "delivered by mailman"],
                        "type": "word",
                        "name": "mailman",
                        "condition": "and",
                    },
                    {
                        "words": ["<b>mailsite <em>express"],
                        "type": "word",
                        "name": "mailsite-express",
                    },
                    {
                        "words": ['onsubmit="openexpress(document.expresslogin)'],
                        "type": "word",
                        "name": "mailsiteexpress",
                    },
                    {
                        "words": ["rockliffe systems, inc."],
                        "type": "word",
                        "name": "mailsiteexpress",
                    },
                    {
                        "words": ["Powered by MallBuilde"],
                        "type": "word",
                        "name": "mallbuilder",
                    },
                    {
                        "words": ['content="MallBuilder'],
                        "type": "word",
                        "name": "mallbuilder",
                    },
                    {"words": ['content="mambo'], "type": "word", "name": "mambo-cms"},
                    {"words": ['<div id="mambo'], "type": "word", "name": "mambo-cms"},
                    {
                        "words": [
                            'content="基于vue2 + element ui 的后台管理系统解决方案'
                        ],
                        "type": "word",
                        "name": "manage-system",
                    },
                    {
                        "words": ['title="manageengine admanager plus'],
                        "type": "word",
                        "name": "manageengine-adaudit",
                    },
                    {
                        "words": ['href="images/adssp_favicon.ico'],
                        "type": "word",
                        "name": "manageengine-adselfservice",
                    },
                    {
                        "words": ["small_status_box"],
                        "type": "word",
                        "name": "manageengine-adselfservice",
                    },
                    {
                        "words": ['src="adsf/js/'],
                        "type": "word",
                        "name": "manageengine-adselfservice",
                    },
                    {
                        "words": ["manageengine"],
                        "type": "word",
                        "name": "manageengine-adselfservice",
                    },
                    {
                        "words": ["/appmanager.js"],
                        "type": "word",
                        "name": "manageengine-applications-manager",
                    },
                    {
                        "words": ['class="footerf2">manageengine assetexplorer'],
                        "type": "word",
                        "name": "manageengine-assetexplorer",
                    },
                    {
                        "words": ["/deviceexpert.js"],
                        "type": "word",
                        "name": "manageengine-deviceexpert",
                    },
                    {
                        "words": ["password manager pro does not allow"],
                        "type": "word",
                        "name": "manageengine-deviceexpert",
                    },
                    {
                        "words": ["selectpasswordpolicy"],
                        "type": "word",
                        "name": "manageengine-deviceexpert",
                    },
                    {
                        "words": ["changepasswordpolicy"],
                        "type": "word",
                        "name": "manageengine-deviceexpert",
                    },
                    {
                        "words": [
                            "the complete network monitoring software from manageengine"
                        ],
                        "type": "word",
                        "name": "manageengine-opmanager",
                    },
                    {
                        "words": [
                            '<a href="http://www.manageengine.com/products/opmanager/index.html" target='
                        ],
                        "type": "word",
                        "name": "manageengine-opmanager",
                    },
                    {
                        "words": [",'manageengine servicedesk plus',"],
                        "type": "word",
                        "name": "manageengine-servicedesk",
                    },
                    {
                        "words": ["北京天源迪科信息技术有限公司"],
                        "type": "word",
                        "name": "management-platform",
                    },
                    {
                        "words": ["casloginview", "i-verfiy"],
                        "type": "word",
                        "name": "management-platform",
                        "condition": "and",
                    },
                    {
                        "words": ["powered by mantis bugtracker"],
                        "type": "word",
                        "name": "mantisbt",
                    },
                    {
                        "words": ['alt="mantis bugtracker'],
                        "type": "word",
                        "name": "mantisbt",
                    },
                    {
                        "words": ["<h2>曼陀罗医疗</h2>"],
                        "type": "word",
                        "name": "mantuoluo-medication",
                    },
                    {"words": ["pwd yahei16"], "type": "word", "name": "maop-oa"},
                    {"words": ["www.oooa.cn"], "type": "word", "name": "maop-oa"},
                    {
                        "words": [
                            'href="http://www.oooa.cn">重庆猫扑网络科技有限公司</a>'
                        ],
                        "type": "word",
                        "name": "maop-oa",
                    },
                    {
                        "words": ['href="img/marathon-favicon.ico'],
                        "type": "word",
                        "name": "marathon",
                    },
                    {
                        "words": ['data-redirect="account/systemconfiguration.aspx"'],
                        "type": "word",
                        "name": "marcopacs-product",
                    },
                    {
                        "words": ["海事选船系统</el-col>"],
                        "type": "word",
                        "name": "maritimeselectionsystem",
                    },
                    {
                        "words": ["action='/mas_security_check'>"],
                        "type": "word",
                        "name": "mas-mobile-agent-serve",
                    },
                    {
                        "words": [
                            'if(!isnotnull(document.forms[0].filepath.value, "证书文件"))'
                        ],
                        "type": "word",
                        "name": "mas-mobile-agent-serve",
                    },
                    {
                        "part": "header",
                        "words": ["Server: mashery proxy"],
                        "type": "word",
                        "name": "mashery-proxy",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: html::mason"],
                        "type": "word",
                        "name": "mason",
                    },
                    {
                        "part": "header",
                        "words": ["Server: mathopd"],
                        "type": "word",
                        "name": "mathopd",
                    },
                    {
                        "words": ["maticsoftsns"],
                        "type": "word",
                        "name": "maticsoft-sns",
                    },
                    {"words": ["maticsoft"], "type": "word", "name": "maticsoft-sns"},
                    {"words": ["/areas/sns/"], "type": "word", "name": "maticsoft-sns"},
                    {
                        "part": "header",
                        "words": ["Server: mbedthis-appweb"],
                        "type": "word",
                        "name": "mbedthis-appweb",
                    },
                    {
                        "words": ["intrushield", "intruvert"],
                        "type": "word",
                        "name": "mcafee-intrushield",
                        "condition": "and",
                    },
                    {
                        "words": ["/worldclient.dll?view=main"],
                        "type": "word",
                        "name": "mdaemon-email-server",
                    },
                    {
                        "words": ["mdaemon "],
                        "type": "word",
                        "name": "mdaemon-email-server",
                    },
                    {
                        "words": ["<strong>mdaemon/worldclient"],
                        "type": "word",
                        "name": "mdaemon-email-server",
                    },
                    {
                        "words": ["obj.reserve = strreserve"],
                        "type": "word",
                        "name": "mechat-irc",
                    },
                    {
                        "words": ["/meiqia.js"],
                        "type": "word",
                        "name": "mechat-online-service",
                    },
                    {
                        "words": ["meetingplaza http tunneling"],
                        "type": "word",
                        "name": "meetingplaza",
                    },
                    {
                        "words": [
                            '<h1 class="logo">欢迎使用 <span class="logo_icon">meis</span> 医疗信息管理系统</h1>'
                        ],
                        "type": "word",
                        "name": "meis-medicine",
                    },
                    {
                        "words": ['action="trackerlogin.aspx'],
                        "type": "word",
                        "name": "meitrack",
                    },
                    {
                        "words": ["_trackermain_gtvtseries"],
                        "type": "word",
                        "name": "meitrack",
                    },
                    {
                        "words": ['title="mercurial" style='],
                        "type": "word",
                        "name": "mercurial",
                    },
                    {
                        "words": ["MessageSolution Enterprise"],
                        "type": "word",
                        "name": "message-solution",
                    },
                    {
                        "words": ["metasploit", "r7bottom-strip"],
                        "type": "word",
                        "name": "metasploit",
                        "condition": "and",
                    },
                    {
                        "words": ["content='dcl.metaview.web.client'"],
                        "type": "word",
                        "name": "metaswitch-networks-metaview-web",
                    },
                    {
                        "words": ['src="dcl.metaview.web.client.nocache.js">'],
                        "type": "word",
                        "name": "metaswitch-networks-metaview-web",
                    },
                    {
                        "words": ["<title>MeterSphere"],
                        "type": "word",
                        "name": "metersphere",
                    },
                    {"words": ['content="metinfo'], "type": "word", "name": "metinfo"},
                    {
                        "words": ["powered_by_metinfo"],
                        "type": "word",
                        "name": "metinfo",
                    },
                    {
                        "words": ["/images/css/metinfo.css"],
                        "type": "word",
                        "name": "metinfo",
                    },
                    {
                        "words": ["metronic. admin dashboard template."],
                        "type": "word",
                        "name": "metronic-admin-theme-framework",
                    },
                    {
                        "words": ['content="mgb opensource guestbook'],
                        "type": "word",
                        "name": "mgb-opensource-guestbook",
                    },
                    {
                        "words": ['title="mgb homepage'],
                        "type": "word",
                        "name": "mgb-opensource-guestbook",
                    },
                    {
                        "words": ['class="empty_inner'],
                        "type": "word",
                        "name": "mibew-messenger",
                    },
                    {
                        "words": ['class="flink">mibew messenger'],
                        "type": "word",
                        "name": "mibew-messenger",
                    },
                    {
                        "words": ["米酷影视 版权所有"],
                        "type": "word",
                        "name": "micool-management-system",
                    },
                    {
                        "words": [
                            'name="keywords" content="电影,视频大全,在线高清电影,付费电影,免费电影,剧集,电影,在线观看,vip高清电影直播"'
                        ],
                        "type": "word",
                        "name": "micool-management-system",
                    },
                    {
                        "words": ["bplay.php?play="],
                        "type": "word",
                        "name": "micool-management-system",
                    },
                    {
                        "words": [
                            "<h3>micro focus open enterprise server 提供市场中的最佳网络、文件和打印服务。</h3>"
                        ],
                        "type": "word",
                        "name": "micro-focus-open-enterprise-server",
                    },
                    {
                        "words": ["/tpl/home/weimeng/common/css/"],
                        "type": "word",
                        "name": "micro-portal",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /owa/", "Server: Microsoft"],
                        "type": "word",
                        "name": "microsoft-exchange",
                        "condition": "and",
                    },
                    {
                        "words": ["<!-- owapage = asp.auth_logon_aspx"],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": ["/exchweb/bin/auth/owalogon.asp"],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": ["/exchweb/bin/auth/owalogon.asp?url="],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": ['href="/owa/auth/'],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": [
                            'window.location.replace("/owa/" + window.location.hash);</script></head><body></body>'
                        ],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": ['<meta http-equiv="refresh" contect="0;url=/owa">'],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": ["themes/resources/segoeui-semibold.ttf"],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": [
                            '<div class="signinheader">outlook</div>',
                            "owalogocontainer",
                        ],
                        "type": "word",
                        "name": "microsoft-exchange",
                        "condition": "and",
                    },
                    {
                        "words": ["/owa/", "owapage = asp.auth_logon_aspx"],
                        "type": "word",
                        "name": "microsoft-exchange",
                        "condition": "and",
                    },
                    {
                        "words": ["/owa/", "showpasswordcheck"],
                        "type": "word",
                        "name": "microsoft-exchange",
                        "condition": "and",
                    },
                    {
                        "words": ["outlook"],
                        "type": "word",
                        "name": "microsoft-exchange",
                    },
                    {
                        "words": [
                            "the isa server denied the specified uniform resource locator"
                        ],
                        "type": "word",
                        "name": "microsoft-isa-server",
                    },
                    {
                        "words": [
                            "the server denied the specified uniform resource locator (url). contact the server "
                            "administrator"
                        ],
                        "type": "word",
                        "name": "microsoft-isa-server",
                    },
                    {
                        "words": ['content="copyright (c) microsoft corporation'],
                        "type": "word",
                        "name": "microsoft-remote-web-workplace",
                    },
                    {
                        "words": ['action="logon.aspx?'],
                        "type": "word",
                        "name": "microsoft-remote-web-workplace",
                    },
                    {
                        "words": ['content="microsoft sharepoint'],
                        "type": "word",
                        "name": "microsoft-sharepoint",
                    },
                    {
                        "words": ['content="sharepoint team'],
                        "type": "word",
                        "name": "microsoft-sharepoint",
                    },
                    {
                        "words": ['id="msowebpartpage_postbacksource'],
                        "type": "word",
                        "name": "microsoft-sharepoint",
                    },
                    {
                        "case-insensitive": True,
                        "words": ["get microsoft silverlight"],
                        "type": "word",
                        "name": "microsoft-silverlight",
                    },
                    {
                        "words": [
                            'var reachclientproductname = "skype for business web 应用"'
                        ],
                        "type": "word",
                        "name": "microsoft-skype-for-business",
                    },
                    {
                        "words": ['"generator" content="Microweber" />'],
                        "type": "word",
                        "name": "microweber",
                    },
                    {
                        "words": [
                            'http://www.mihalism.com/product/mmh/">mihalism multi host'
                        ],
                        "type": "word",
                        "name": "mihalism-multi-host",
                    },
                    {
                        "words": ["powered by mihalism multi host"],
                        "type": "word",
                        "name": "mihalism-multi-host",
                    },
                    {
                        "words": ['content="mihalism multi host'],
                        "type": "word",
                        "name": "mihalism-multi-host",
                    },
                    {
                        "part": "header",
                        "words": ["Server: mikrotik httpproxy"],
                        "type": "word",
                        "name": "mikrotik-httpproxy",
                    },
                    {
                        "words": ["eth\u200a—\u200atotal speed:"],
                        "type": "word",
                        "name": "minergate-claymore-miner",
                    },
                    {
                        "words": ["eth: gpu0"],
                        "type": "word",
                        "name": "minergate-claymore-miner",
                    },
                    {
                        "words": ['href="imges/zgsh.ico"'],
                        "type": "word",
                        "name": "mingdekeji-system",
                    },
                    {
                        "words": ["荆州明德科技"],
                        "type": "word",
                        "name": "mingdekeji-system",
                    },
                    {
                        "words": ['<img id="erp" src="/_imgs/login/zs_erp.png'],
                        "type": "word",
                        "name": "mingyuanyun-sales",
                    },
                    {
                        "words": ['value="明源售楼管理系统v5.0"'],
                        "type": "word",
                        "name": "mingyuanyun-sales",
                    },
                    {
                        "words": ["<!--minibb copyright link"],
                        "type": "word",
                        "name": "minibb",
                    },
                    {
                        "words": ['powered by <a href="http://www.minibb.'],
                        "type": "word",
                        "name": "minibb",
                    },
                    {
                        "words": ["<title>MinIO Browser</title>"],
                        "type": "word",
                        "name": "minio",
                    },
                    {
                        "words": ['href="/minio/loader.css"'],
                        "type": "word",
                        "name": "minio-browser",
                    },
                    {
                        "words": ['alt="mission control application shield'],
                        "type": "word",
                        "name": "mission-control-application-shield",
                    },
                    {
                        "words": ["深圳市深海捷科技有限公司"],
                        "type": "word",
                        "name": "mixcall-seat-management-center",
                    },
                    {
                        "words": ["/admin/modules/admin/statics/images/"],
                        "type": "word",
                        "name": "mixcall-seat-management-center",
                    },
                    {
                        "part": "header",
                        "words": ["Server: mjniohttpdaemon"],
                        "type": "word",
                        "name": "mjniohttpdaemon",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: mjniohttpdsessionid"],
                        "type": "word",
                        "name": "mjniohttpdaemon",
                    },
                    {
                        "words": ["北京数字天堂信息科技有限责任公司"],
                        "type": "word",
                        "name": "mkey",
                    },
                    {
                        "words": ['content="mkportal'],
                        "type": "word",
                        "name": "mkportal",
                    },
                    {
                        "words": ['target="_blank">mkportal</a>'],
                        "type": "word",
                        "name": "mkportal",
                    },
                    {
                        "words": ["window.location.href = '/ui/html/login.html';"],
                        "type": "word",
                        "name": "mobile-office-system",
                    },
                    {
                        "words": [
                            "click here for more information about mobilityguard"
                        ],
                        "type": "word",
                        "name": "mobilityguard",
                    },
                    {"words": ["modlogan.css"], "type": "word", "name": "modlogan"},
                    {"words": [">modlogan"], "type": "word", "name": "modlogan"},
                    {
                        "words": ["this error was generated by mod_security"],
                        "type": "word",
                        "name": "modsecurity",
                    },
                    {
                        "words": ['class="meeting movision"'],
                        "type": "word",
                        "name": "momeeting-movision",
                    },
                    {
                        "words": ['document.title="登录-摩云视讯"'],
                        "type": "word",
                        "name": "momeeting-movision",
                    },
                    {
                        "words": ["<!-- 科达视讯云 摩云视讯 电信有区别 -->"],
                        "type": "word",
                        "name": "momeeting-movision",
                    },
                    {
                        "words": [
                            "It looks like you are trying to access MongoDB over HTTP on the native driver port."
                        ],
                        "type": "word",
                        "name": "mongodb",
                    },
                    {
                        "words": ["Mongo Express", "mongo-express-logo.png"],
                        "type": "word",
                        "name": "mongoexpress",
                        "condition": "and",
                    },
                    {
                        "words": ['content="鹰眼盒子监控中心'],
                        "type": "word",
                        "name": "monitoring-center",
                    },
                    {
                        "words": ['title="moodle" href="http://moodle.org/'],
                        "type": "word",
                        "name": "moodle",
                    },
                    {"words": ['content="moodle'], "type": "word", "name": "moodle"},
                    {"words": ['m.str = {"moodle":'], "type": "word", "name": "moodle"},
                    {
                        "words": ["mfs.cgi", "under-goal files"],
                        "type": "word",
                        "name": "moosefs",
                        "condition": "and",
                    },
                    {
                        "words": ["movable type version"],
                        "type": "word",
                        "name": "movable-type",
                    },
                    {
                        "words": ['content="movable type'],
                        "type": "word",
                        "name": "movable-type",
                    },
                    {
                        "words": ['rel="generator">movable type'],
                        "type": "word",
                        "name": "movable-type",
                    },
                    {
                        "words": ['href="http://oss.oetiker.ch/mrtg/'],
                        "type": "word",
                        "name": "mrtg",
                    },
                    {
                        "words": [
                            "/mrtg-l.png",
                            'command line is easier to read using "view page properties" of your browser',
                        ],
                        "type": "word",
                        "name": "mrtg",
                        "condition": "and",
                    },
                    {
                        "words": ['action="i.cgi'],
                        "type": "word",
                        "name": "ms-mfc-httpsvr",
                    },
                    {
                        "words": ["powered by mshift&reg"],
                        "type": "word",
                        "name": "mshift",
                    },
                    {
                        "words": ["$('.sign-btn').addclass(\"completion\");"],
                        "type": "word",
                        "name": "msvod-mediamanager",
                    },
                    {
                        "words": ["static/js/meisicms.js"],
                        "type": "word",
                        "name": "msvod-mediamanager",
                    },
                    {
                        "words": ['content="魅思cms'],
                        "type": "word",
                        "name": "msvod-mediamanager",
                    },
                    {
                        "words": ["<label>打开m18 app， 扫描二维码</label>"],
                        "type": "word",
                        "name": "multiabnle-m18",
                    },
                    {
                        "words": ["auto-generated by munin"],
                        "type": "word",
                        "name": "munin",
                    },
                    {"words": ["munin-month.html"], "type": "word", "name": "munin"},
                    {"words": ['content="mvmmall'], "type": "word", "name": "mvmmall"},
                    {
                        "words": ['content="www.mvmmall.cn"'],
                        "type": "word",
                        "name": "mvmmall",
                    },
                    {
                        "words": ['powered by <a href="http://www.mybboard.com'],
                        "type": "word",
                        "name": "mybb",
                    },
                    {
                        "words": [
                            "<!-- mybb is free software developed and maintained"
                        ],
                        "type": "word",
                        "name": "mybb",
                    },
                    {
                        "words": ["visibility of the mybb copyright at any time"],
                        "type": "word",
                        "name": "mybb",
                    },
                    {
                        "words": ['onchange="mybb.changelanguage();'],
                        "type": "word",
                        "name": "mybb",
                    },
                    {
                        "words": ["powered by my little forum"],
                        "type": "word",
                        "name": "mylittleforum",
                    },
                    {
                        "words": ["index.php?mode=js_defaults"],
                        "type": "word",
                        "name": "mylittleforum",
                    },
                    {
                        "words": ["<!-- begin: menu footer don't change anything"],
                        "type": "word",
                        "name": "myre-php",
                    },
                    {
                        "words": ["<b><u>my last viewed</u></b>"],
                        "type": "word",
                        "name": "myre-php",
                    },
                    {
                        "words": ["if(window.__myscadaversion)"],
                        "type": "word",
                        "name": "myscada-hmi",
                    },
                    {
                        "words": ['<select class="sqlcombo" name="tablecombo'],
                        "type": "word",
                        "name": "mysqldumper",
                    },
                    {
                        "words": ['href="mysql.cgi?do=top_level_op'],
                        "type": "word",
                        "name": "mysqlman",
                    },
                    {
                        "words": ['size="1">mysqlman'],
                        "type": "word",
                        "name": "mysqlman",
                    },
                    {
                        "words": ["href='mwftp/common/mwftp.css"],
                        "type": "word",
                        "name": "mywebftp",
                    },
                    {
                        "words": ["mwftp/common/mwftp.js"],
                        "type": "word",
                        "name": "mywebftp",
                    },
                    {"words": [">mywebftp</a>"], "type": "word", "name": "mywebftp"},
                    {
                        "words": ['target="_blank" href="http://mywebsql.net'],
                        "type": "word",
                        "name": "mywebsql",
                    },
                    {
                        "words": ['method="post" action="" name="dbform" '],
                        "type": "word",
                        "name": "mywebsql",
                    },
                    {
                        "words": ["static/style/cpm_style.css"],
                        "type": "word",
                        "name": "n2ws",
                    },
                    {
                        "words": ['class="nabble" id="nabble'],
                        "type": "word",
                        "name": "nabble",
                    },
                    {"words": [">nabble</a>"], "type": "word", "name": "nabble"},
                    {
                        "words": ["<title>Nacos</title>"],
                        "type": "word",
                        "name": "nacos",
                    },
                    {
                        "words": [
                            "click the link below to get started using nagios xi."
                        ],
                        "type": "word",
                        "name": "nagios-xi",
                    },
                    {
                        "words": ["/nagiosxi/images/favicon.ico"],
                        "type": "word",
                        "name": "nagios-xi",
                    },
                    {
                        "words": ['id="ctl00_contentplaceholder1_txthospcode"'],
                        "type": "word",
                        "name": "nalong-system",
                    },
                    {
                        "words": ['http://www.namazu.org/">namazu'],
                        "type": "word",
                        "name": "namazu",
                    },
                    {"words": ['content="neo4j'], "type": "word", "name": "neo4j"},
                    {
                        "words": ['ng-show="neo4j.enterpriseedition'],
                        "type": "word",
                        "name": "neo4j",
                    },
                    {
                        "words": ['play-topic="neo4j-sync'],
                        "type": "word",
                        "name": "neo4j",
                    },
                    {
                        "words": ["{{ neo4j.version | neo4jdeveloperdoc }}/"],
                        "type": "word",
                        "name": "neo4j",
                    },
                    {
                        "words": ["<!-- net2ftp version"],
                        "type": "word",
                        "name": "net2ftp",
                    },
                    {
                        "words": ["<!-- end of net2ftp login form"],
                        "type": "word",
                        "name": "net2ftp",
                    },
                    {
                        "words": ['href="http://www.net2ftp.com">net2ftp</a>'],
                        "type": "word",
                        "name": "net2ftp",
                    },
                    {"words": ['content="net2ftp'], "type": "word", "name": "net2ftp"},
                    {
                        "part": "header",
                        "words": ["Server: data ontap"],
                        "type": "word",
                        "name": "netapp-data-ontap",
                    },
                    {
                        "part": "header",
                        "words": ["Server: netcache appliance"],
                        "type": "word",
                        "name": "netapp-netcache-appliance",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.netartmedia.net/realestate'
                        ],
                        "type": "word",
                        "name": "netartmedia-real-estate-portal",
                    },
                    {
                        "words": ['src="image/loginauthorize.png"'],
                        "type": "word",
                        "name": "netauth",
                    },
                    {
                        "words": [
                            "onmouseover=\"mm_swapimage('image1','','image/loginok_after.gif',1)\""
                        ],
                        "type": "word",
                        "name": "netauth",
                    },
                    {
                        "words": ['href="/netbotz.css'],
                        "type": "word",
                        "name": "netbotz-network-monitoring-device",
                    },
                    {
                        "words": ['href="http://www.netbotz.com" target'],
                        "type": "word",
                        "name": "netbotz-network-monitoring-device",
                    },
                    {
                        "part": "header",
                        "words": ["Server: netbox"],
                        "type": "word",
                        "name": "netbox",
                    },
                    {"words": ['href="runecrm.ico"'], "type": "word", "name": "netcrm"},
                    {"words": ["power by netdoit"], "type": "word", "name": "netdoit"},
                    {
                        "words": [
                            'align="center"><a href="http://www.net-doit.com" target="_blank'
                        ],
                        "type": "word",
                        "name": "netdoit",
                    },
                    {
                        "words": ["frmvalidator"],
                        "type": "word",
                        "name": "netease-enterprise-mailbox",
                    },
                    {
                        "words": [
                            '<span class="warn">请您从网易企业邮箱用户登录页登录</span>'
                        ],
                        "type": "word",
                        "name": "netease-enterprise-mailbox",
                    },
                    {
                        "words": [
                            'content="网易企业邮箱',
                            'src="http://mimg.qiye.163.com/',
                        ],
                        "type": "word",
                        "name": "netease-enterprise-mailbox",
                        "condition": "and",
                    },
                    {
                        "words": ["netflow analyzer"],
                        "type": "word",
                        "name": "netflow-analyzer-zoho-traffic-management",
                    },
                    {
                        "words": ["zoho"],
                        "type": "word",
                        "name": "netflow-analyzer-zoho-traffic-management",
                    },
                    {
                        "words": ['href="/netflow/css/netflow.css'],
                        "type": "word",
                        "name": "netflow-analyzer-zoho-traffic-management",
                    },
                    {
                        "words": ["gigabit dual wan ssl vpn firewall fvs336gv3</div>"],
                        "type": "word",
                        "name": "netgear-prosafe-vpn-firewall-fvs336gv3",
                    },
                    {
                        "words": ["var mywindows = ext.create"],
                        "type": "word",
                        "name": "netmizer-log-management-system",
                    },
                    {
                        "words": ['window.location.href="main.html";'],
                        "type": "word",
                        "name": "netmizer-log-management-system",
                    },
                    {
                        "words": ['版权所有 <a href="http://www.netpas.cc'],
                        "type": "word",
                        "name": "netpas-traffic-management-system",
                    },
                    {
                        "words": ['href="/sys/images/tree.css" title="netpilot'],
                        "type": "word",
                        "name": "netpilot",
                    },
                    {
                        "part": "header",
                        "words": ["Server: netport software"],
                        "type": "word",
                        "name": "netport",
                    },
                    {"words": ["netposa"], "type": "word", "name": "netposa-system"},
                    {
                        "part": "header",
                        "words": ["Server: netpresenz"],
                        "type": "word",
                        "name": "netpresenz",
                    },
                    {
                        "words": ['action="nquser.php'],
                        "type": "word",
                        "name": "netquery",
                    },
                    {
                        "words": ['href="nqadmin.php'],
                        "type": "word",
                        "name": "netquery",
                    },
                    {
                        "part": "header",
                        "words": ["Server: netscape-fasttrack"],
                        "type": "word",
                        "name": "netscape-fasttrack",
                    },
                    {
                        "words": ['var newpath = "/common/ngeniusdirect.jsp'],
                        "type": "word",
                        "name": "netscout-ngeniusone",
                    },
                    {
                        "words": ["vpn", "netshare"],
                        "type": "word",
                        "name": "netshare-vpn",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'window.location.href = "/appframe/login_v2/login.jsp";'
                        ],
                        "type": "word",
                        "name": "netsoft-eida",
                    },
                    {
                        "words": ['href="/images/common/favicon.ico"'],
                        "type": "word",
                        "name": "netsoft-eida",
                    },
                    {
                        "words": ["netsweepersmbtextatbottomofloginscreen"],
                        "type": "word",
                        "name": "netsweeper",
                    },
                    {
                        "words": ['href="http://www.poweredbynetsweeper.com"><img '],
                        "type": "word",
                        "name": "netsweeper",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: webadmin="],
                        "type": "word",
                        "name": "netsweeper",
                    },
                    {
                        "part": "header",
                        "words": ["Server: nettalk-webserver"],
                        "type": "word",
                        "name": "nettalk-webserver",
                    },
                    {
                        "words": ["/cgi/dbabble.cgi"],
                        "type": "word",
                        "name": "netwin-dbabble",
                    },
                    {
                        "words": [">dbabble online help</a>"],
                        "type": "word",
                        "name": "netwin-dbabble",
                    },
                    {
                        "words": ['action="/dbabble'],
                        "type": "word",
                        "name": "netwin-dbabble",
                    },
                    {
                        "words": [
                            'href="/about_mail.htm">about surgemail',
                            "surgemail welcome page",
                        ],
                        "type": "word",
                        "name": "netwin-surgemail",
                        "condition": "and",
                    },
                    {
                        "words": [">network tracker<"],
                        "type": "word",
                        "name": "network-tracker",
                    },
                    {
                        "words": ["<b>网络资源综合支撑辅助平台</b>"],
                        "type": "word",
                        "name": "networkresourcesauxiliaryplatform",
                    },
                    {
                        "words": ["广州高度软件有限公司版权所有"],
                        "type": "word",
                        "name": "netzone-webcache",
                    },
                    {
                        "words": ['action="/ntars/loginaction.do'],
                        "type": "word",
                        "name": "neusoft-neteye-bitsaver",
                    },
                    {
                        "words": ['href="/ntars/css/'],
                        "type": "word",
                        "name": "neusoft-neteye-bitsaver",
                    },
                    {
                        "words": ["senginx-robot-mitigation"],
                        "type": "word",
                        "name": "neusoft-senginx",
                    },
                    {
                        "words": [
                            "ecdomain/unieap/ria/unieap/themes/earth/common/unieap.css"
                        ],
                        "type": "word",
                        "name": "neusoft-uniportal",
                    },
                    {
                        "words": ["var data = formatparams(params.data)"],
                        "type": "word",
                        "name": "new-rock-ip-pbx-management-system",
                    },
                    {
                        "words": [
                            '<img id="createcheckcode" src="login/picturecheckcode" name="check_code" '
                            'ng-click="reloadcheckcode()'
                        ],
                        "type": "word",
                        "name": "news",
                    },
                    {
                        "words": ['ng-disabled="!loginform.$valid"'],
                        "type": "word",
                        "name": "news",
                    },
                    {"words": ['name="group_sn"'], "type": "word", "name": "newton"},
                    {
                        "words": ["/nexpose-dark.min.css"],
                        "type": "word",
                        "name": "nexpose-security-console",
                    },
                    {
                        "words": ["nexposeccpassword"],
                        "type": "word",
                        "name": "nexpose-security-console",
                    },
                    {
                        "words": ["/nexpose-features.js"],
                        "type": "word",
                        "name": "nexpose-security-console",
                    },
                    {
                        "words": ["nextcloud</a> – 给您所有数据一个安全的家"],
                        "type": "word",
                        "name": "nextcloud-product",
                    },
                    {
                        "words": ["<title>Nexus Repository Manager</title>"],
                        "type": "word",
                        "name": "nexus-repository-manager",
                    },
                    {
                        "words": [" nexus repository manager"],
                        "type": "word",
                        "name": "nexus-repository-manager",
                    },
                    {
                        "words": ["progressmessage('initializing ...')"],
                        "type": "word",
                        "name": "nexus-repository-manager",
                    },
                    {
                        "words": ['src="/diam4/inc/lang/fr/lang.min.js"'],
                        "type": "word",
                        "name": "ngi-diam4",
                    },
                    {
                        "words": ['<div id="passwordwrapper" class="fieldwrapper">'],
                        "type": "word",
                        "name": "ngi-gxd5-pacs",
                    },
                    {
                        "words": ["/images/nitc1.png", "nitc web marketing service"],
                        "type": "word",
                        "name": "nitc-cms",
                        "condition": "and",
                    },
                    {
                        "words": ["interesting ports on", "starting nmap"],
                        "type": "word",
                        "name": "nmap-log",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'class="blackfont">诺姆四达人力资源测评咨询服务有限公司'
                        ],
                        "type": "word",
                        "name": "normstar-hr",
                    },
                    {
                        "words": [
                            '<h2 style="margin-left: 0px;">norton cloud connect</h2>'
                        ],
                        "type": "word",
                        "name": "norton-cloud-connect",
                    },
                    {
                        "words": ["<!-- start novell services"],
                        "type": "word",
                        "name": "novell-groupwise",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: njscn="],
                        "type": "word",
                        "name": "novell-groupwise",
                    },
                    {
                        "words": ['code="nwshealth.class'],
                        "type": "word",
                        "name": "novell-netware",
                    },
                    {
                        "part": "header",
                        "words": ["Server: netware http stack"],
                        "type": "word",
                        "name": "novell-netware",
                    },
                    {
                        "words": [
                            "<!-- this is all just a super-duper redirect to the welcome page"
                        ],
                        "type": "word",
                        "name": "novell-open-enterprise-server",
                    },
                    {
                        "words": [
                            'href="http://www.novell.com/products/openenterpriseserver'
                        ],
                        "type": "word",
                        "name": "novell-open-enterprise-server",
                    },
                    {
                        "words": ['title="novell sentinel log manager'],
                        "type": "word",
                        "name": "novell-sentinel-log-manager",
                    },
                    {
                        "words": ['content="0;url=/novelllogmanager">'],
                        "type": "word",
                        "name": "novell-sentinel-log-manager",
                    },
                    {
                        "words": ["/zenworks/js/dojo"],
                        "type": "word",
                        "name": "novell-zenworks",
                    },
                    {
                        "words": ["managementzonename"],
                        "type": "word",
                        "name": "novell-zenworks",
                    },
                    {
                        "words": ['id="novnc-control-bar'],
                        "type": "word",
                        "name": "novnc",
                    },
                    {
                        "words": ["novnc example: simple"],
                        "type": "word",
                        "name": "novnc",
                    },
                    {
                        "words": ['content="nowayer"'],
                        "type": "word",
                        "name": "nowayer-ftp",
                    },
                    {
                        "words": ['content="n点虚拟主机管理系统'],
                        "type": "word",
                        "name": "npoint",
                    },
                    {"words": ["js/ajax_x.js"], "type": "word", "name": "npoint"},
                    {
                        "words": ["/inc/usercode.asp?npoint="],
                        "type": "word",
                        "name": "npoint",
                    },
                    {
                        "words": ["ehang", "login", "nps"],
                        "type": "word",
                        "name": "nps",
                        "condition": "and",
                    },
                    {
                        "words": ["/nsfocus_bvs.css"],
                        "type": "word",
                        "name": "nsfocus-bvs",
                    },
                    {
                        "words": ["/login_logo_espc_zh_cn.png"],
                        "type": "word",
                        "name": "nsfocus-enterprise-security-center",
                    },
                    {
                        "words": ["/logo_login_nsfocus.png"],
                        "type": "word",
                        "name": "nsfocus-vpn",
                    },
                    {
                        "words": ["<h2>nsoc大数据分析系统</h2>"],
                        "type": "word",
                        "name": "nsoc-bigdata",
                    },
                    {
                        "words": ['src="/nfw/static/framework/images/views.png'],
                        "type": "word",
                        "name": "nsoc-bigdata",
                    },
                    {
                        "words": ["<b>nsoc云安全解决方案"],
                        "type": "word",
                        "name": "nsoc-bigdata",
                    },
                    {
                        "words": [
                            'skysz.fw.index.validatecodenew = "/loginaction/validatecodenew.html'
                        ],
                        "type": "word",
                        "name": "nstc-software",
                    },
                    {
                        "words": ["var base_path = '/stormweb';"],
                        "type": "word",
                        "name": "nstrong-itmaster",
                    },
                    {
                        "words": ["netstrong"],
                        "type": "word",
                        "name": "nstrong-itmaster",
                    },
                    {
                        "words": ["global traffic statistics"],
                        "type": "word",
                        "name": "ntop",
                    },
                    {"words": ["ntopmenuid"], "type": "word", "name": "ntop"},
                    {"words": ["/css/ntopng.css"], "type": "word", "name": "ntop"},
                    {
                        "words": ["<font color=lightgray>generated by ntopng"],
                        "type": "word",
                        "name": "ntopng",
                    },
                    {
                        "words": [
                            '<a href=http://www.ntop.org><img src="/img/logo.png"></a>'
                        ],
                        "type": "word",
                        "name": "ntopng",
                    },
                    {
                        "words": ['href="/css/ntopng.css'],
                        "type": "word",
                        "name": "ntopng",
                    },
                    {
                        "words": ["safecom mobile print"],
                        "type": "word",
                        "name": "nuance-safecom",
                    },
                    {
                        "words": ['target="_blank">nucleus cms'],
                        "type": "word",
                        "name": "nucleus-cms",
                    },
                    {
                        "words": ["nucleus_lf_name"],
                        "type": "word",
                        "name": "nucleus-cms",
                    },
                    {
                        "words": ['title="nucleus" href="http://nucleuscms.org/'],
                        "type": "word",
                        "name": "nucleus-cms",
                    },
                    {
                        "words": [
                            "<div style='display: none;' id=\"setstatusiconstatediv\"></div>"
                        ],
                        "type": "word",
                        "name": "nvidia-mellanox-mlnx-os",
                    },
                    {
                        "words": ["/mlx-default.css"],
                        "type": "word",
                        "name": "nvidia-mellanox-mlnx-os",
                    },
                    {
                        "words": ["'/admin/launch?script="],
                        "type": "word",
                        "name": "nvidia-mellanox-mlnx-os",
                    },
                    {
                        "words": ["url=/admin/launch?script="],
                        "type": "word",
                        "name": "nvidia-mellanox-mlnx-os",
                    },
                    {
                        "words": ['home_obm.png" alt="obm'],
                        "type": "word",
                        "name": "obm",
                    },
                    {
                        "words": ['href="images/observium-icon.png"'],
                        "type": "word",
                        "name": "observium",
                    },
                    {
                        "words": ["江苏欧索软件有限公司"],
                        "type": "word",
                        "name": "oceansoft",
                    },
                    {
                        "words": ["/ocensoftcomm.js"],
                        "type": "word",
                        "name": "oceansoft",
                    },
                    {
                        "words": ['技术支持：<a href="http://www.oceansoft.com.cn/">'],
                        "type": "word",
                        "name": "oceansoft",
                    },
                    {
                        "words": [
                            "aspx/casecenter/acasecenter.aspx?pagetype=sxcx&casetype=sscs&casename=",
                            'href="/e/action/listinfo/?',
                        ],
                        "type": "word",
                        "name": "oceansoft",
                        "condition": "and",
                    },
                    {"words": ["江苏欧索"], "type": "word", "name": "oceansoft"},
                    {
                        "words": ["css/ocsreports.css"],
                        "type": "word",
                        "name": "ocs-inventory-ng",
                    },
                    {
                        "words": ['content="爱施德移动渠道管理系统'],
                        "type": "word",
                        "name": "ocss",
                    },
                    {
                        "words": ['<option value="age_sys">代理商内部员工</option>'],
                        "type": "word",
                        "name": "ocss",
                    },
                    {"words": ["爱施德 aisidi.com"], "type": "word", "name": "ocss"},
                    {
                        "words": ['value="connect to octopussy'],
                        "type": "word",
                        "name": "octopussy",
                    },
                    {
                        "words": ['align="center" >octopussy'],
                        "type": "word",
                        "name": "octopussy",
                    },
                    {
                        "words": ["var generatedviewurlelementid"],
                        "type": "word",
                        "name": "office-web-apps",
                    },
                    {
                        "words": ['title="homepage of open source lms olat'],
                        "type": "word",
                        "name": "olat",
                    },
                    {"words": ['content="olat'], "type": "word", "name": "olat"},
                    {
                        "words": ['content="olat 是一个学习内容管理系统 (lcms).'],
                        "type": "word",
                        "name": "olat",
                    },
                    {
                        "words": ['content="olat - online learning and training'],
                        "type": "word",
                        "name": "olat",
                    },
                    {
                        "words": ['o_info.uriprefix="/olat/dmz/";'],
                        "type": "word",
                        "name": "olat",
                    },
                    {"words": ['href="/olat/raw/'], "type": "word", "name": "olat"},
                    {
                        "words": [
                            "<p><a href=\"http://www.omeeting.com\" target='_blank'>powered by"
                        ],
                        "type": "word",
                        "name": "om-video-conferencing",
                    },
                    {
                        "words": ['content="om视频会议'],
                        "type": "word",
                        "name": "om-video-conferencing",
                    },
                    {
                        "words": ["onclick=\"gotomeeting('/gotomeeting.php"],
                        "type": "word",
                        "name": "om-video-conferencing",
                    },
                    {"words": ["OneThink管理平台"], "type": "word", "name": "onethink"},
                    {"words": ["onethink.css"], "type": "word", "name": "onethink"},
                    {
                        "words": ['content="online grades'],
                        "type": "word",
                        "name": "online-grades",
                    },
                    {
                        "words": ['<div class="extras" id="password_extras">', "crm"],
                        "type": "word",
                        "name": "onminutes-crm",
                        "condition": "and",
                    },
                    {
                        "words": [
                            "/monitor/application/views/themes/default/css/default/common.css"
                        ],
                        "type": "word",
                        "name": "op5-monitor",
                    },
                    {
                        "words": ['action="/cgi-bin/rptbirthday.pl'],
                        "type": "word",
                        "name": "open-admin-for-schools",
                    },
                    {
                        "words": ["open admin for schools"],
                        "type": "word",
                        "name": "open-admin-for-schools",
                    },
                    {
                        "words": ['target="_blank">open blog'],
                        "type": "word",
                        "name": "open-blog",
                    },
                    {
                        "words": ['id="footer-openedx'],
                        "type": "word",
                        "name": "open-edx",
                    },
                    {
                        "words": ['class="footer-about-openedx'],
                        "type": "word",
                        "name": "open-edx",
                    },
                    {
                        "words": ['alt="powered by open edx'],
                        "type": "word",
                        "name": "open-edx",
                    },
                    {
                        "words": ['content="open-realty'],
                        "type": "word",
                        "name": "open-realty",
                    },
                    {
                        "words": [
                            'href="http://www.open-realty.org" title="powered by open-realty'
                        ],
                        "type": "word",
                        "name": "open-realty",
                    },
                    {
                        "words": ['src="/otrs-web/images/'],
                        "type": "word",
                        "name": "open-source-ticket-request-system",
                    },
                    {
                        "words": [
                            "you need to enable javascript to access the open-xchange server"
                        ],
                        "type": "word",
                        "name": "open-xchange",
                    },
                    {
                        "words": ['id="browserchecktext_id'],
                        "type": "word",
                        "name": "open-xchange",
                    },
                    {
                        "words": ['action="/openam/ui/login"'],
                        "type": "word",
                        "name": "openam",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: openconf="],
                        "type": "word",
                        "name": "openconf",
                    },
                    {
                        "words": ['powered by <a href="http://www.openconf.org'],
                        "type": "word",
                        "name": "openconf",
                    },
                    {
                        "words": ['src="openconf.js?'],
                        "type": "word",
                        "name": "openconf",
                    },
                    {
                        "words": ["webpackJsonpopencti-front"],
                        "type": "word",
                        "name": "opencti",
                    },
                    {
                        "words": ["j_security_check"],
                        "type": "word",
                        "name": "opendaylight-product",
                    },
                    {
                        "words": ["welcome to opendocman"],
                        "type": "word",
                        "name": "opendocman",
                    },
                    {
                        "words": ['target="_new">opendocman'],
                        "type": "word",
                        "name": "opendocman",
                    },
                    {
                        "words": ['href="http://www.openfiler.com/">openfiler'],
                        "type": "word",
                        "name": "openfiler",
                    },
                    {
                        "words": ["</strong>openfiler nas/san appliance"],
                        "type": "word",
                        "name": "openfiler",
                    },
                    {
                        "words": [
                            "background: transparent url(images/login_logo.gif) no-repeat"
                        ],
                        "type": "word",
                        "name": "openfire",
                    },
                    {
                        "words": ['class="row justify-content-center"'],
                        "type": "word",
                        "name": "openfire",
                    },
                    {
                        "words": ["<title>openfire 管理界面</title>"],
                        "type": "word",
                        "name": "openfire",
                    },
                    {
                        "words": ["/stylesheets/themes/wifi2/ui.theme.css"],
                        "type": "word",
                        "name": "opengoss-wlan",
                    },
                    {
                        "words": ['content="wlan综合网管系统"'],
                        "type": "word",
                        "name": "opengoss-wlan",
                    },
                    {
                        "words": ['content="opennemas '],
                        "type": "word",
                        "name": "opennemas",
                    },
                    {
                        "words": ['href="http://netmeans.net">netmeans.net'],
                        "type": "word",
                        "name": "opennewsletter",
                    },
                    {
                        "words": [
                            "this software is based on the opennewsletter project"
                        ],
                        "type": "word",
                        "name": "opennewsletter",
                    },
                    {
                        "words": ["/images/opsview_logo_large.gif"],
                        "type": "word",
                        "name": "opsview-product",
                    },
                    {
                        "words": ["/images/opsviewcommunitylogo-large.png"],
                        "type": "word",
                        "name": "opsview-product",
                    },
                    {
                        "words": ["follow @opsview"],
                        "type": "word",
                        "name": "opsview-product",
                    },
                    {
                        "words": ["href=\"javascript:__dopostback('lbanonymous','')\""],
                        "type": "word",
                        "name": "opt-webfieldassis",
                    },
                    {
                        "words": ["/css/optergy.css"],
                        "type": "word",
                        "name": "optergy-product",
                    },
                    {
                        "words": ['footerversion">oracle access manager version'],
                        "type": "word",
                        "name": "oracle-access-manager",
                    },
                    {
                        "words": ['content="oracle adf'],
                        "type": "word",
                        "name": "oracle-adf-faces",
                    },
                    {
                        "words": ["<!-- created by oracle adf faces"],
                        "type": "word",
                        "name": "oracle-adf-faces",
                    },
                    {
                        "words": ["var _adfwindowopenerror"],
                        "type": "word",
                        "name": "oracle-adf-faces",
                    },
                    {
                        "words": ["oracle http server</font></font></b></h1>"],
                        "type": "word",
                        "name": "oracle-application-server",
                    },
                    {
                        "words": ["mod_ssl web site"],
                        "type": "word",
                        "name": "oracle-application-server",
                    },
                    {
                        "words": ["oracle jsp documentation"],
                        "type": "word",
                        "name": "oracle-application-server",
                    },
                    {
                        "words": ["mod_ose documentation"],
                        "type": "word",
                        "name": "oracle-application-server",
                    },
                    {
                        "words": ['id="oracle-cc'],
                        "type": "word",
                        "name": "oracle-commerce-cloud",
                    },
                    {
                        "words": ['<a href="workspace/dynamichelp'],
                        "type": "word",
                        "name": "oracle-enterprise-performance-management-system",
                    },
                    {
                        "words": ['href="css/fmw_bottom_area.css'],
                        "type": "word",
                        "name": "oracle-fusion-middleware",
                    },
                    {
                        "words": ["operalogin/welcome.do"],
                        "type": "word",
                        "name": "oracle-opera",
                    },
                    {
                        "words": ["primavera systems, inc"],
                        "type": "word",
                        "name": "oracle-primerva",
                    },
                    {
                        "words": ['class="introareabuildid'],
                        "type": "word",
                        "name": "oracle-primerva",
                    },
                    {
                        "words": [
                            "com.primavera.detectplugin.detectpluginapplet.class"
                        ],
                        "type": "word",
                        "name": "oracle-primerva",
                    },
                    {
                        "words": ["ot='siebwebmainwindow'>"],
                        "type": "word",
                        "name": "oracle-siebel-crm",
                    },
                    {
                        "words": [
                            "scripting is used to manage data interactions between the siebel server/web server"
                        ],
                        "type": "word",
                        "name": "oracle-siebel-crm",
                    },
                    {
                        "words": ["onload=\"gotourl('start.swe?swecmd=start')"],
                        "type": "word",
                        "name": "oracle-siebel-crm",
                    },
                    {
                        "part": "header",
                        "words": ["Server: oracle_webdb_listener"],
                        "type": "word",
                        "name": "oracle-webdb",
                    },
                    {
                        "part": "header",
                        "words": ["Server: oracle xml db"],
                        "type": "word",
                        "name": "oracle-xdb",
                    },
                    {
                        "words": ["welcome to the orangehrm ver"],
                        "type": "word",
                        "name": "orangehrm",
                    },
                    {
                        "words": ['name="hdnusertimezoneoffset'],
                        "type": "word",
                        "name": "orangehrm",
                    },
                    {
                        "words": ['href="http://www.orangehrm.com" target='],
                        "type": "word",
                        "name": "orangehrm",
                    },
                    {
                        "words": ["getorderbook: function"],
                        "type": "word",
                        "name": "orderbook",
                    },
                    {
                        "words": ["联系新订单系统开发同事进行修改。</div>"],
                        "type": "word",
                        "name": "ordermanagementsystem",
                    },
                    {
                        "part": "header",
                        "words": ["Server: orenosv"],
                        "type": "word",
                        "name": "orenosv",
                    },
                    {
                        "words": ["<span>radone pacs</span>"],
                        "type": "word",
                        "name": "osirix-pixmeo",
                    },
                    {
                        "words": [
                            'service provided by <a href="https://www.osirix-viewer.com"'
                        ],
                        "type": "word",
                        "name": "osirix-viewer",
                    },
                    {
                        "words": ['src="/uf/login/login.jsp" >'],
                        "type": "word",
                        "name": "oss",
                    },
                    {
                        "words": ['class="sys-title-right">智联云服务'],
                        "type": "word",
                        "name": "ostd-cloud",
                    },
                    {
                        "words": ["background-image: url('/icones/fundo_firebox.png')"],
                        "type": "word",
                        "name": "ostec-firebox",
                    },
                    {
                        "words": ["http://colorzilla.com/"],
                        "type": "word",
                        "name": "ostec-firebox",
                    },
                    {
                        "words": ['href="http://osticket.com">osticket.com'],
                        "type": "word",
                        "name": "osticket",
                    },
                    {
                        "words": ['<span i18n="1">益模模具智能制造系统</span>'],
                        "type": "word",
                        "name": "oureman-eman",
                    },
                    {
                        "words": [
                            '<a href="https://www.ovirt.org" class="obrand_loginpagelogolink">'
                        ],
                        "type": "word",
                        "name": "ovirt-virtualization",
                    },
                    {
                        "words": ['src="/res/api/owvmapi.js?'],
                        "type": "word",
                        "name": "owtware-iconn-end-user-calculation",
                    },
                    {
                        "words": ['content="pageadmin cms"'],
                        "type": "word",
                        "name": "pageadmin",
                    },
                    {
                        "words": ["/e/images/favicon.ico"],
                        "type": "word",
                        "name": "pageadmin",
                    },
                    {
                        "words": ['powered by <a href="http://pagecookery.com'],
                        "type": "word",
                        "name": "pagecookery-microblog",
                    },
                    {
                        "words": ["powered by pageup people"],
                        "type": "word",
                        "name": "pageup-people",
                    },
                    {
                        "words": [
                            'class="pageuplink" href="http://www.pageuppeople.com'
                        ],
                        "type": "word",
                        "name": "pageup-people",
                    },
                    {
                        "words": ["global-protect/login.esp"],
                        "type": "word",
                        "name": "paloalto-globalprotect",
                    },
                    {
                        "words": [" 2015 palo alto networks, inc. "],
                        "type": "word",
                        "name": "paloalto-networks-sso",
                    },
                    {
                        "words": ["panasonic_logo.gif"],
                        "type": "word",
                        "name": "panasonic-maintenance-utility",
                    },
                    {
                        "words": ['title="pandora rss feed'],
                        "type": "word",
                        "name": "pandora-fms",
                    },
                    {
                        "words": ["pansoftplugins/multithreading/pancalc.js"],
                        "type": "word",
                        "name": "pansoft-financial-system",
                    },
                    {
                        "words": ['src="/login/img/loading.gif"'],
                        "type": "word",
                        "name": "pansoft-financial-system",
                    },
                    {
                        "words": ['directlink = "eafc.application";'],
                        "type": "word",
                        "name": "pansoft-management-system",
                    },
                    {
                        "words": ["parallels ip holdings gmbh"],
                        "type": "word",
                        "name": "parallels-plesk-panel",
                    },
                    {
                        "words": [
                            '<img src="./img/panel-logo.png" alt="parallels plesk panel"></a>'
                        ],
                        "type": "word",
                        "name": "parallels-plesk-panel",
                    },
                    {
                        "words": ['src="kbfolder.asp?deptid='],
                        "type": "word",
                        "name": "parature",
                    },
                    {
                        "words": ["redirectportalurl('/ics/support/custhandler.asp?"],
                        "type": "word",
                        "name": "parature",
                    },
                    {
                        "words": ["<!-- <title>派拉统一身份管理系统</title> -->"],
                        "type": "word",
                        "name": "paraview-uams",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: pbootsystem="],
                        "type": "word",
                        "name": "pbootcms",
                    },
                    {
                        "words": ["powered by pc4uploader"],
                        "type": "word",
                        "name": "pc4uploader",
                    },
                    {
                        "words": ["pc4uploader <font color"],
                        "type": "word",
                        "name": "pc4uploader",
                    },
                    {
                        "words": ["deze server is eigendom van <a"],
                        "type": "word",
                        "name": "pcextreme",
                    },
                    {
                        "words": ["images/slider/banner-gis.png"],
                        "type": "word",
                        "name": "pcitc-cameras-and-surveillance",
                    },
                    {
                        "words": ["js/scripts/common/easyui/jquery.easyui.min.js"],
                        "type": "word",
                        "name": "pcitc-file-management-system",
                    },
                    {
                        "words": ["/home/plug_in_download"],
                        "type": "word",
                        "name": "pcitc-lims",
                    },
                    {
                        "words": [' href="/newimages/login/logo_icon.ico'],
                        "type": "word",
                        "name": "pcitc-logistics-management-mystem",
                    },
                    {
                        "words": [
                            'window.open("http://itmc.mmsh.sinopec.com/itgk/sysmgr/productregister/yunweiproregister.view"); '
                            "}  "
                        ],
                        "type": "word",
                        "name": "pcitc-management-platform",
                    },
                    {
                        "words": ['src="validatecode.aspx"'],
                        "type": "word",
                        "name": "pcitc-remotesystem",
                    },
                    {
                        "words": ['class="f-loading-mask ui-widget ui-widget-content"'],
                        "type": "word",
                        "name": "pcitc-remotesystem",
                    },
                    {
                        "words": ['src="new_style/placeholderfriend.js"'],
                        "type": "word",
                        "name": "pcitc-sslvpn",
                    },
                    {
                        "words": ["动设备运行风险分析系统"],
                        "type": "word",
                        "name": "pcitc-system",
                    },
                    {
                        "words": ['title="powered by pcpin chat'],
                        "type": "word",
                        "name": "pcpin-chat",
                    },
                    {
                        "words": [
                            'onclick="document.loginform.register.value=0; document.loginform.lostpassword.value=0'
                        ],
                        "type": "word",
                        "name": "pcpin-chat",
                    },
                    {
                        "words": ["window.appname_='pcpin_chat'"],
                        "type": "word",
                        "name": "pcpin-chat",
                    },
                    {
                        "words": ["installed packages, channel pear"],
                        "type": "word",
                        "name": "pear",
                    },
                    {
                        "words": ['content="webbased pear package manager'],
                        "type": "word",
                        "name": "pear",
                    },
                    {
                        "words": ["unable to logon to the pegarules system"],
                        "type": "word",
                        "name": "pegarules",
                    },
                    {
                        "words": ['href="images/pzpegaicon.ico'],
                        "type": "word",
                        "name": "pegarules",
                    },
                    {
                        "words": ['content="ikey,众人科技,ikey'],
                        "type": "word",
                        "name": "peoplenet-ikey",
                    },
                    {
                        "words": ["percona xtradb cluster node"],
                        "type": "word",
                        "name": "percona-xtradb-cluster",
                    },
                    {
                        "words": ["<h2>pexip infinity</h2>"],
                        "type": "word",
                        "name": "pexip",
                    },
                    {"words": ["<h2>会议平台</h2>"], "type": "word", "name": "pexip"},
                    {"words": ["pexip infinity"], "type": "word", "name": "pexip"},
                    {"words": ["pex-app"], "type": "word", "name": "pexip"},
                    {
                        "words": ["rubicon communications, llc (netgate)"],
                        "type": "word",
                        "name": "pfsense",
                    },
                    {
                        "words": ["<h4>login to pfsense</h4>"],
                        "type": "word",
                        "name": "pfsense",
                    },
                    {
                        "words": [
                            ">pg real estate solution - real estate web site design"
                        ],
                        "type": "word",
                        "name": "pg-real-estate-solution",
                    },
                    {
                        "words": ["powered by pg roomate finder solution"],
                        "type": "word",
                        "name": "pg-roomate-finder-solution",
                    },
                    {
                        "words": ['powered by <a href="http://www.realtysoft.pro'],
                        "type": "word",
                        "name": "pg-roomate-finder-solution",
                    },
                    {
                        "words": ["pgadmin 客户端安装包"],
                        "type": "word",
                        "name": "pgadmin",
                    },
                    {
                        "words": ["phabricator-application-launch-container"],
                        "type": "word",
                        "name": "phabricator",
                    },
                    {
                        "words": ["res/phabricator"],
                        "type": "word",
                        "name": "phabricator",
                    },
                    {
                        "words": ['href="https://www.phocassoftware.com"'],
                        "type": "word",
                        "name": "phocas",
                    },
                    {
                        "words": ['<a href="/pacs/"><img src="pacs/images/logo.svg">'],
                        "type": "word",
                        "name": "phonix-pacs",
                    },
                    {
                        "words": ['powered by <a href="http://www.phorum.org'],
                        "type": "word",
                        "name": "phorum",
                    },
                    {
                        "words": ['src="adm-misc.php?admact=mainmenu'],
                        "type": "word",
                        "name": "photopost-php",
                    },
                    {
                        "words": ['href="http://www.photopost.com">photopost'],
                        "type": "word",
                        "name": "photopost-php",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.ktools.net/photostore/index.php'
                        ],
                        "type": "word",
                        "name": "photostore",
                    },
                    {
                        "words": ['<div class="index_link_list_name">'],
                        "type": "word",
                        "name": "php-cloud",
                    },
                    {
                        "words": ["yun_toplogin a.yun_more"],
                        "type": "word",
                        "name": "php-cloud",
                    },
                    {"words": ["yun_index.css"], "type": "word", "name": "php-cloud"},
                    {
                        "words": ["$(this).removeclass('dn'"],
                        "type": "word",
                        "name": "php-cloud",
                    },
                    {"words": ["phpyun"], "type": "word", "name": "php-cloud"},
                    {
                        "words": ['content="php code snippet'],
                        "type": "word",
                        "name": "php-csl",
                    },
                    {
                        "words": ['title="php-csl">php-csl'],
                        "type": "word",
                        "name": "php-csl",
                    },
                    {
                        "words": ['alt="powered by php layers menu'],
                        "type": "word",
                        "name": "php-layers",
                    },
                    {
                        "words": ["<!-- end of menu header - php layers menu"],
                        "type": "word",
                        "name": "php-layers",
                    },
                    {
                        "words": ["<!-- beginning of menu header - php layers menu"],
                        "type": "word",
                        "name": "php-layers",
                    },
                    {
                        "words": ["powered by phplinkdirectory"],
                        "type": "word",
                        "name": "php-link-directory",
                    },
                    {
                        "words": ['content="php link directory'],
                        "type": "word",
                        "name": "php-link-directory",
                    },
                    {
                        "words": ['target="_blank">php server monitor'],
                        "type": "word",
                        "name": "php-server-monitor",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.phpservermonitor.org/'
                        ],
                        "type": "word",
                        "name": "php-server-monitor",
                    },
                    {
                        "words": ['title="php support tickets">php support tickets'],
                        "type": "word",
                        "name": "php-support-tickets",
                    },
                    {
                        "words": ['content="triangle solutions ltd'],
                        "type": "word",
                        "name": "php-support-tickets",
                    },
                    {
                        "words": [
                            'content="http://www.888072.com',
                            'content="qq 7000719',
                        ],
                        "type": "word",
                        "name": "php-voting-system",
                        "condition": "and",
                    },
                    {
                        "words": ['content="http://www.vote123.cn'],
                        "type": "word",
                        "name": "php-voting-system",
                    },
                    {
                        "words": ["var system = 'cms"],
                        "type": "word",
                        "name": "php168cms",
                    },
                    {
                        "words": ['src="viewer_bottom.php?file='],
                        "type": "word",
                        "name": "phpatm",
                    },
                    {"words": ["powered by phpatm"], "type": "word", "name": "phpatm"},
                    {
                        "words": ["powered by php advanced transfer manager"],
                        "type": "word",
                        "name": "phpatm",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: phpbb3_"],
                        "type": "word",
                        "name": "phpbb",
                    },
                    {
                        "words": ["http://www.longluntan.com/zh/phpbb/"],
                        "type": "word",
                        "name": "phpbb",
                    },
                    {
                        "words": ["phpbb", 'phpbb group" />'],
                        "type": "word",
                        "name": "phpbb",
                        "condition": "and",
                    },
                    {
                        "words": ["start quick hack - phpbb statistics mod"],
                        "type": "word",
                        "name": "phpbb",
                    },
                    {
                        "words": [
                            '<a href="http://www.phpcms.cn" target="_blank">PHPCMS</a>'
                        ],
                        "type": "word",
                        "name": "phpcms",
                    },
                    {
                        "words": ["<!-- powered by phpcollab"],
                        "type": "word",
                        "name": "phpcollab",
                    },
                    {
                        "words": ["content='phpcollab"],
                        "type": "word",
                        "name": "phpcollab",
                    },
                    {
                        "words": [
                            'class="pythonselect',
                            'for="dealer_radiuss_dealer_zip',
                        ],
                        "type": "word",
                        "name": "phpdealerlocator",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'content="chat4all phpdenora',
                            "powered by phpdenora",
                        ],
                        "type": "word",
                        "name": "phpdenora",
                        "condition": "and",
                    },
                    {
                        "words": ['powered by <a href="http://denorastats.org/'],
                        "type": "word",
                        "name": "phpdenora",
                    },
                    {
                        "words": ["powered by phpdisk"],
                        "type": "word",
                        "name": "phpdisk",
                    },
                    {"words": ['content="phpdisk'], "type": "word", "name": "phpdisk"},
                    {
                        "words": ["/plus/laydate/laydate.js"],
                        "type": "word",
                        "name": "phpecms",
                    },
                    {"words": ["powered by phpfox"], "type": "word", "name": "phpfox"},
                    {
                        "words": ['powered by <a href="http://www.phpfox.com'],
                        "type": "word",
                        "name": "phpfox",
                    },
                    {
                        "words": ["powered by phpfreechat"],
                        "type": "word",
                        "name": "phpfreechat",
                    },
                    {
                        "words": ['src="http://www.phpfreechat.net/pub/'],
                        "type": "word",
                        "name": "phpfreechat",
                    },
                    {
                        "words": ["<title>phpinfo", "Virtual Directory Support"],
                        "type": "word",
                        "name": "phpinfo",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'href="http://phpldapadmin.sourceforge.net/documentation" onclick'
                        ],
                        "type": "word",
                        "name": "phpldapadmin",
                    },
                    {
                        "words": [
                            'src="images/default/logo.png" title="phpldapadmin logo'
                        ],
                        "type": "word",
                        "name": "phpldapadmin",
                    },
                    {
                        "words": ['content="michiel dethmers - http://www.phplist.com'],
                        "type": "word",
                        "name": "phplist",
                    },
                    {
                        "words": ['content="phplist version'],
                        "type": "word",
                        "name": "phplist",
                    },
                    {
                        "words": ['&copy; <a href="http://phplist.com" target'],
                        "type": "word",
                        "name": "phplist",
                    },
                    {
                        "words": ["href='http://phpmoneybooks.com'>phpmoneybooks"],
                        "type": "word",
                        "name": "phpmoneybooks",
                    },
                    {"words": ["powered by phpmps"], "type": "word", "name": "phpmps"},
                    {
                        "words": ["templates/phpmps/style/index.css"],
                        "type": "word",
                        "name": "phpmps",
                    },
                    {
                        "words": ['href="phpmyadmin.css.php'],
                        "type": "word",
                        "name": "phpmyadmin",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: phpmyadmin="],
                        "type": "word",
                        "name": "phpmyadmin",
                    },
                    {"words": ["pma_password"], "type": "word", "name": "phpmyadmin"},
                    {
                        "words": [
                            'please login (use your mysql username and password): <a href="index.php?login=true'
                        ],
                        "type": "word",
                        "name": "phpmybackuppro",
                    },
                    {
                        "words": ["<div class='chaphead'>"],
                        "type": "word",
                        "name": "phpmybible",
                    },
                    {
                        "words": ["<!-- main content table : stop -->"],
                        "type": "word",
                        "name": "phpmyrealty",
                    },
                    {
                        "words": ['powered by <a href="http://www.phpmyrealty.com'],
                        "type": "word",
                        "name": "phpmyrealty",
                    },
                    {
                        "words": ["phpmywind.com all rights reserved"],
                        "type": "word",
                        "name": "phpmywind",
                    },
                    {
                        "words": ['content="phpmywind'],
                        "type": "word",
                        "name": "phpmywind",
                    },
                    {
                        "words": ['href="http://phpnow.org/go.php?id=1005">'],
                        "type": "word",
                        "name": "phpnow",
                    },
                    {
                        "words": ['content="yinzcn@gmail.com'],
                        "type": "word",
                        "name": "phpnow",
                    },
                    {
                        "words": ['by <a href="http://phpnow.org">phpnow.org'],
                        "type": "word",
                        "name": "phpnow",
                    },
                    {
                        "words": ["url(template/default/images/admin_img/msg.png)"],
                        "type": "word",
                        "name": "phpoa",
                    },
                    {
                        "words": ["admin_img/msg_bg.png"],
                        "type": "word",
                        "name": "phpoa",
                    },
                    {"words": ['content="phpok'], "type": "word", "name": "phpok"},
                    {
                        "words": ["phpopenchat installation"],
                        "type": "word",
                        "name": "phpopenchat",
                    },
                    {
                        "words": ['class="appname">phppgadmin'],
                        "type": "word",
                        "name": "phppgadmin",
                    },
                    {
                        "words": ["style='font: 8pt verdana'>phpremoteview"],
                        "type": "word",
                        "name": "phpremoteview",
                    },
                    {"words": ["powered by phpshe"], "type": "word", "name": "phpshe"},
                    {"words": ['content="phpshe'], "type": "word", "name": "phpshe"},
                    {
                        "words": ['var stargeturl = "index.php?disp=dynamic'],
                        "type": "word",
                        "name": "phpsysinfo",
                    },
                    {
                        "words": ['content="phpsysinfo'],
                        "type": "word",
                        "name": "phpsysinfo",
                    },
                    {
                        "words": [
                            'href="http://phpsysinfo.sourceforge.net/">phpsysinfo'
                        ],
                        "type": "word",
                        "name": "phpsysinfo",
                    },
                    {
                        "words": ["/templates/phpsysinfo.css"],
                        "type": "word",
                        "name": "phpsysinfo",
                    },
                    {"words": ["pdv_pagename"], "type": "word", "name": "phpweb"},
                    {
                        "words": ["powered by phpwiki"],
                        "type": "word",
                        "name": "phpwiki",
                    },
                    {
                        "words": ['<td><div class="minicalmonth'],
                        "type": "word",
                        "name": "phxeventmanager",
                    },
                    {
                        "words": ['src="pem-includes/xajax/xajax_js/xajax_core.js'],
                        "type": "word",
                        "name": "phxeventmanager",
                    },
                    {
                        "words": ["powered by phxeventmanager"],
                        "type": "word",
                        "name": "phxeventmanager",
                    },
                    {
                        "words": ["ajax/confirm.ashx", "css/sexybuttons.css"],
                        "type": "word",
                        "name": "piaoyou-erp",
                        "condition": "and",
                    },
                    {
                        "words": ['alt="piaware skyview"'],
                        "type": "word",
                        "name": "piaware-skyview",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: pigcms.com"],
                        "type": "word",
                        "name": "pigcms",
                    },
                    {
                        "words": ["axiom storage services manager"],
                        "type": "word",
                        "name": "pillar-axiom-software",
                    },
                    {
                        "words": ["/admin/css/images/pineapp.ico"],
                        "type": "word",
                        "name": "pineapp",
                    },
                    {
                        "words": ['<body id="pinpoint">', "<title>PINPOINT</title>"],
                        "type": "word",
                        "name": "pinpoint",
                        "condition": "and",
                    },
                    {"words": ["powered bypivot"], "type": "word", "name": "pivot"},
                    {
                        "words": ['href="http://www.pivotlog.net/?ver=pivot'],
                        "type": "word",
                        "name": "pivot",
                    },
                    {
                        "words": [
                            '<frame name="hidden" src="xmlloader.asp?type=portal'
                        ],
                        "type": "word",
                        "name": "pivotal-crm",
                    },
                    {
                        "words": ['title="hidden" src="xmlloader.asp?type=portal'],
                        "type": "word",
                        "name": "pivotal-crm",
                    },
                    {
                        "words": ['src="includes/js/pivotx.js'],
                        "type": "word",
                        "name": "pivotx",
                    },
                    {
                        "words": ['templates_internal/assets/pivotx.png" alt="pivotx'],
                        "type": "word",
                        "name": "pivotx",
                    },
                    {"words": ['content="pivotx'], "type": "word", "name": "pivotx"},
                    {
                        "words": ['generator" content="piwigo'],
                        "type": "word",
                        "name": "piwigo",
                    },
                    {
                        "words": ['title="powered by pixelpost'],
                        "type": "word",
                        "name": "pixelpost",
                    },
                    {
                        "words": ['powered by <a href="http://www.pixelpost.org'],
                        "type": "word",
                        "name": "pixelpost",
                    },
                    {
                        "words": ['title="pixelpost - '],
                        "type": "word",
                        "name": "pixelpost",
                    },
                    {
                        "words": ['<span class="apptitle" id="apppixviewer">'],
                        "type": "word",
                        "name": "pixeon-pacs",
                    },
                    {
                        "words": ['name="plesk-build"'],
                        "type": "word",
                        "name": "plesk-plesk-onyx",
                    },
                    {
                        "words": ['风<span style="padding-left: 12px;"></span>格'],
                        "type": "word",
                        "name": "pmway-e4",
                    },
                    {
                        "words": ["热情似火</option>"],
                        "type": "word",
                        "name": "pmway-e4",
                    },
                    {
                        "words": [
                            'tip_browsertoolow:"您当前使用的浏览器版本或模式太低，鹏为e5为了您更好的体验，请升级您的ie版本至8.0或以上。"'
                        ],
                        "type": "word",
                        "name": "pmway-e5",
                    },
                    {
                        "words": ["<span class='commentout-pmwikiorg'>"],
                        "type": "word",
                        "name": "pmwiki",
                    },
                    {
                        "words": ['<a href="http://www.pmwiki.org/" target='],
                        "type": "word",
                        "name": "pmwiki",
                    },
                    {
                        "words": ['class="heading1">policyretriever service</p>'],
                        "type": "word",
                        "name": "policyretriever",
                    },
                    {
                        "words": [
                            "window.location.href = '/syncmodule/synchome/index';"
                        ],
                        "type": "word",
                        "name": "pollutionsourcemonitoringdataexchangeplatform",
                    },
                    {
                        "words": ['src="cgi-bin/ippbx.cgi?module=showlogin"'],
                        "type": "word",
                        "name": "polycom-ippbx",
                    },
                    {
                        "words": ['src="cgi-bin/httptohttps.cgi"'],
                        "type": "word",
                        "name": "polycom-ippbx",
                    },
                    {
                        "words": ['window.location.replace("/rss/")'],
                        "type": "word",
                        "name": "polycom-rss-record",
                    },
                    {
                        "words": ['powered by <a href="http://www.pommo.org/'],
                        "type": "word",
                        "name": "pommo",
                    },
                    {"words": ["pommo.confirmmsg = "], "type": "word", "name": "pommo"},
                    {
                        "words": ['ng-app="portainer"'],
                        "type": "word",
                        "name": "portainer",
                    },
                    {
                        "words": ['content="posterous'],
                        "type": "word",
                        "name": "posterous",
                    },
                    {
                        "words": ['class="posterous_site_data'],
                        "type": "word",
                        "name": "posterous",
                    },
                    {
                        "words": ['src="/view/static/js/eidpcommon.min.js"'],
                        "type": "word",
                        "name": "posun-sales",
                    },
                    {
                        "words": ["<p>快易销公众号</p>"],
                        "type": "word",
                        "name": "posun-sales",
                    },
                    {
                        "words": ['post("/ssosaml/saml2signonhandler.ashx")'],
                        "type": "word",
                        "name": "power-cpms",
                    },
                    {
                        "words": ["apphub.server.registertohub(qrcodeid)"],
                        "type": "word",
                        "name": "power-powerpms",
                    },
                    {
                        "words": ["/app_themes/default/assets/css/style.min.css"],
                        "type": "word",
                        "name": "power-powerpms",
                    },
                    {
                        "words": ["/scripts/boot.js"],
                        "type": "word",
                        "name": "power-powerpms",
                    },
                    {
                        "part": "header",
                        "words": ["Server: poweralert http server"],
                        "type": "word",
                        "name": "poweralert",
                    },
                    {
                        "words": ["email:support@powercreator.com.cn<br />"],
                        "type": "word",
                        "name": "powercreator-cms",
                    },
                    {
                        "words": ['<span class="bottom_versionno">'],
                        "type": "word",
                        "name": "powercreator-cms",
                    },
                    {
                        "words": ["powercreator "],
                        "type": "word",
                        "name": "powercreator-cms",
                    },
                    {
                        "words": [
                            "<html><body>access denied.  please consult the http-access directive in the user's guide "
                            "for more information.</body>"
                        ],
                        "type": "word",
                        "name": "powermta",
                    },
                    {
                        "part": "header",
                        "words": ["Server: powerweb"],
                        "type": "word",
                        "name": "powerweb",
                    },
                    {
                        "words": ["ppvod copyright"],
                        "type": "word",
                        "name": "ppvod-videosystem",
                    },
                    {
                        "words": ['action="/gqjx/loginmgr.do?method=dologin'],
                        "type": "word",
                        "name": "preamsolutions-inspection-and-modification-information-platform",
                    },
                    {
                        "words": ['powered by <a href="http://pritlog.com/">pritlog'],
                        "type": "word",
                        "name": "pritlog",
                    },
                    {
                        "words": [
                            '<em id="jserror">please enable javascript for full functionality'
                        ],
                        "type": "word",
                        "name": "pritlog",
                    },
                    {
                        "words": ['border="0" alt="pro chat rooms'],
                        "type": "word",
                        "name": "pro-chat-rooms",
                    },
                    {
                        "words": ["href='http://prochatrooms.com'>pro chat rooms</a>"],
                        "type": "word",
                        "name": "pro-chat-rooms",
                    },
                    {
                        "words": ["processmaker ver"],
                        "type": "word",
                        "name": "processmaker",
                    },
                    {
                        "words": [
                            'href="http://www.processmaker.com" alt="processmaker'
                        ],
                        "type": "word",
                        "name": "processmaker",
                    },
                    {
                        "words": ['alt="powered by processmaker'],
                        "type": "word",
                        "name": "processmaker",
                    },
                    {
                        "part": "header",
                        "words": ["Server: profense"],
                        "type": "word",
                        "name": "profense-firewall",
                    },
                    {
                        "words": ["myicalusername"],
                        "type": "word",
                        "name": "progress-imailserver",
                    },
                    {
                        "words": ["var right = regexp.rightcontext"],
                        "type": "word",
                        "name": "project-management-system",
                    },
                    {
                        "words": ['window.top.location = "login.aspx?url=" + right"'],
                        "type": "word",
                        "name": "project-management-system",
                    },
                    {
                        "words": [
                            "powered by squirrelmail.org. squirrelmail",
                            "promail &trade; - login",
                        ],
                        "type": "word",
                        "name": "promail",
                        "condition": "and",
                    },
                    {
                        "words": ["<title>Prometheus"],
                        "type": "word",
                        "name": "prometheus",
                    },
                    {
                        "words": ["js/promise/themes/apple/images/logo_promise.png"],
                        "type": "word",
                        "name": "promise-webpam",
                    },
                    {
                        "words": ['src="js/dojo/promise.js'],
                        "type": "word",
                        "name": "promise-webpam",
                    },
                    {
                        "words": ['name="promisecactivex"'],
                        "type": "word",
                        "name": "promisec-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: proscan"],
                        "type": "word",
                        "name": "proscan",
                    },
                    {
                        "words": ['class="boxheadline">proxmox virtual environment'],
                        "type": "word",
                        "name": "proxmox-ve",
                    },
                    {
                        "words": [
                            "href='http://www.proxmox.com' target='_blank' class=\"boxheadline"
                        ],
                        "type": "word",
                        "name": "proxmox-ve",
                    },
                    {
                        "words": ["ext.create('pve.stdworkspace')"],
                        "type": "word",
                        "name": "proxmox-ve",
                    },
                    {
                        "words": ['content="公安检查站人脸/证件合一核录系统'],
                        "type": "word",
                        "name": "public-security-checkpoint-document-verification-system",
                    },
                    {
                        "words": ["/mpoweb/a/login"],
                        "type": "word",
                        "name": "publicopinionmonitoringsystem",
                    },
                    {
                        "words": ["<b>pulse connect secure</b>"],
                        "type": "word",
                        "name": "pulsesecure-ssl-vpn",
                    },
                    {
                        "words": ['content="/puridiom/system/header.jsp'],
                        "type": "word",
                        "name": "puridiom",
                    },
                    {
                        "words": ['src="/puridiom/system/processing.jsp'],
                        "type": "word",
                        "name": "puridiom",
                    },
                    {
                        "words": [
                            'generated by <a href="http://www.quux.org/devel/gopher/pygopherd'
                        ],
                        "type": "word",
                        "name": "pygopherd",
                    },
                    {
                        "words": ['content="pyspider dashboard"'],
                        "type": "word",
                        "name": "pyspider",
                    },
                    {
                        "words": ["zend engine version:</b>"],
                        "type": "word",
                        "name": "qcodo-development-framework",
                    },
                    {
                        "words": ["<b>qcodo version:"],
                        "type": "word",
                        "name": "qcodo-development-framework",
                    },
                    {
                        "words": ['<div id="codeversion">qcubed development framework'],
                        "type": "word",
                        "name": "qcubed-development-framework",
                    },
                    {
                        "words": ["<b>qcubed version:</b>"],
                        "type": "word",
                        "name": "qcubed-development-framework",
                    },
                    {
                        "words": [
                            'href="/static/build/animate_nprogress_timepiacker_tooltipster.min.css'
                        ],
                        "type": "word",
                        "name": "qianxin-analytics",
                    },
                    {
                        "words": ['input name="s1" type="image"'],
                        "type": "word",
                        "name": "qianxing-oa",
                    },
                    {
                        "words": ["count/mystat.asp"],
                        "type": "word",
                        "name": "qianxing-oa",
                    },
                    {
                        "words": ["powered by qibosoft v1.0"],
                        "type": "word",
                        "name": "qibosoft-microsite",
                    },
                    {
                        "words": ['/images/v7/cms.css">'],
                        "type": "word",
                        "name": "qibosoft-v7",
                    },
                    {"words": ["hsse 系统"], "type": "word", "name": "qingyuan-hsse"},
                    {
                        "words": ["-moz-background-size", 'class="u_logo fa fa-user"'],
                        "type": "word",
                        "name": "qingyuan-management-system",
                        "condition": "and",
                    },
                    {
                        "words": [
                            "如果能访问到qinzhe网站上的图片，说明网络是通的，显示新闻"
                        ],
                        "type": "word",
                        "name": "qinzhe-excel",
                    },
                    {
                        "words": ['id="chkworkbyreplacer" type="checkbox"'],
                        "type": "word",
                        "name": "qinzhe-excel",
                    },
                    {
                        "words": ['src="polyfills.js'],
                        "type": "word",
                        "name": "qm-system",
                    },
                    {
                        "words": ["assets/css/fdb.css"],
                        "type": "word",
                        "name": "qm-system",
                    },
                    {"words": ["url=gqrtweb"], "type": "word", "name": "qtweb"},
                    {
                        "words": [
                            'window.location.replace("/cgi-bin/web2cgi/index.cgi");'
                        ],
                        "type": "word",
                        "name": "quarkmail",
                    },
                    {
                        "words": [
                            '<iframe src="/cgi-bin/web2cgi/index.cgi" scrolling="no" frameborder='
                        ],
                        "type": "word",
                        "name": "quarkmail",
                    },
                    {"words": ["quest software"], "type": "word", "name": "quest-dr"},
                    {"words": ["cui-login-screen"], "type": "word", "name": "quest-dr"},
                    {
                        "words": ['style="display:none" id="account_notfilled.textbox'],
                        "type": "word",
                        "name": "quest-password-manager",
                    },
                    {
                        "words": ['id="ginapageexpiration'],
                        "type": "word",
                        "name": "quest-password-manager",
                    },
                    {
                        "words": ['id="ctl00_ctl00_ctl00_ctl00_body'],
                        "type": "word",
                        "name": "quest-password-manager",
                    },
                    {
                        "words": [
                            'id="ctl00_ctl00_ctl00_ctl00_contentplaceholder_pleasewait_content'
                        ],
                        "type": "word",
                        "name": "quest-password-manager",
                    },
                    {
                        "words": ['target="_blank">the quix project</a></small>'],
                        "type": "word",
                        "name": "quixplorer",
                    },
                    {
                        "words": ["/qzmail/index.php"],
                        "type": "word",
                        "name": "qzsafemail",
                    },
                    {
                        "words": ["<title>RabbitMQ Management</title>"],
                        "type": "word",
                        "name": "rabbitmq",
                    },
                    {
                        "words": ["unauthorized activity has been detected."],
                        "type": "word",
                        "name": "radware-appwall",
                    },
                    {
                        "words": ["/webimages/raidenmaild.jpg"],
                        "type": "word",
                        "name": "raiden-mail",
                    },
                    {
                        "words": [
                            '<a href="http://www.rainier.net.cn">北京润尼尔网络科技有限公司'
                        ],
                        "type": "word",
                        "name": "rainier-internet-product",
                    },
                    {
                        "words": [".: <b>rainmail intranet login </b> :.</div>"],
                        "type": "word",
                        "name": "rainmail",
                    },
                    {
                        "words": ['href="/resources/rainmailvpninstaller.exe'],
                        "type": "word",
                        "name": "rainmail",
                    },
                    {
                        "words": ['id="com_raisecom_ums_aos_portal_login_domain'],
                        "type": "word",
                        "name": "raisecom-ivoice8000",
                    },
                    {
                        "words": ["ralph <strong>3</strong>"],
                        "type": "word",
                        "name": "ralph",
                    },
                    {
                        "words": ["/sys/index.php?m=user&f=login&referer="],
                        "type": "word",
                        "name": "ranzhi-oa",
                    },
                    {
                        "words": ["webpackJsonprap2-dolores"],
                        "type": "word",
                        "name": "rap2",
                    },
                    {
                        "words": ["<!-- ### bullet table ### -->"],
                        "type": "word",
                        "name": "rapid-browser",
                    },
                    {
                        "words": [
                            'images/login_button.gif" alt="login to rapid browser'
                        ],
                        "type": "word",
                        "name": "rapid-browser",
                    },
                    {
                        "words": ['name="redirectto" value="/zym/rbkj.nsf"'],
                        "type": "word",
                        "name": "rbsoft-software",
                    },
                    {"words": ["rConfigLogo"], "type": "word", "name": "rconfig"},
                    {
                        "words": ['content="realtime web acars'],
                        "type": "word",
                        "name": "realtime-web-acars",
                    },
                    {
                        "words": ["148779de-1cf1-49bb-8bdb-129321cf8974"],
                        "type": "word",
                        "name": "reddoxx",
                    },
                    {
                        "words": ["<b>登录到红旗集群管理系统</b></td>"],
                        "type": "word",
                        "name": "redflag-linux-cluster-management-system",
                    },
                    {
                        "words": ["authenticity_token", "redmine"],
                        "type": "word",
                        "name": "redmine",
                        "condition": "and",
                    },
                    {"words": ['content="redmine'], "type": "word", "name": "redmine"},
                    {
                        "words": ['powered by <a href="http://www.redmine.org/'],
                        "type": "word",
                        "name": "redmine",
                    },
                    {
                        "words": ['alt="redseal, inc."/></a>'],
                        "type": "word",
                        "name": "redseal-system",
                    },
                    {
                        "words": ["stylesheets/redwoodtheme/resources/js/azzurra.js"],
                        "type": "word",
                        "name": "redwoodhq",
                    },
                    {
                        "words": ['content="remobjects sdk'],
                        "type": "word",
                        "name": "remobjects-dxsock",
                    },
                    {
                        "words": ["remobjects software, llc."],
                        "type": "word",
                        "name": "remobjects-dxsock",
                    },
                    {
                        "words": ['/img/ralogo.png" alt="remotelyanywhere'],
                        "type": "word",
                        "name": "remotelyanywhere",
                    },
                    {
                        "part": "header",
                        "words": ["Server: remotelyanywhere"],
                        "type": "word",
                        "name": "remotelyanywhere",
                    },
                    {
                        "words": ["/resources/imgs/defaultannex/loginpictures/"],
                        "type": "word",
                        "name": "renwoxing-crm",
                    },
                    {
                        "words": ["蝙蝠在线考试系统"],
                        "type": "word",
                        "name": "reremouse-exam-system",
                    },
                    {
                        "words": [
                            "博库医学在线考试系统，技术支持：杭州博库科技有限公司"
                        ],
                        "type": "word",
                        "name": "reremouse-exam-system",
                    },
                    {
                        "words": ['src="/resources/js/upscroll.js"'],
                        "type": "word",
                        "name": "reremouse-exam-system",
                    },
                    {
                        "words": [
                            "this is standby rm. redirecting to the current active rm"
                        ],
                        "type": "word",
                        "name": "resourcemanager",
                    },
                    {
                        "words": ["/static/rb/images/delete"],
                        "type": "word",
                        "name": "reviewboard",
                    },
                    {
                        "words": ['content="richmail', "richmail"],
                        "type": "word",
                        "name": "richinfo-richmail",
                        "condition": "and",
                    },
                    {
                        "words": ['src="ravweb_files/'],
                        "type": "word",
                        "name": "rising-antivirus-online",
                    },
                    {
                        "words": [
                            'action="/index.php" onsubmit="return checkfrm(this);'
                        ],
                        "type": "word",
                        "name": "rising-antivirus-wall",
                    },
                    {
                        "words": ["uiwebinsights/webinsights.html"],
                        "type": "word",
                        "name": "riverbed-appresponse",
                    },
                    {
                        "words": ["cloudclient httpserver is running..."],
                        "type": "word",
                        "name": "rmihttpserver",
                    },
                    {
                        "words": ['onclick="loginsubmit()"'],
                        "type": "word",
                        "name": "rockoa-oa",
                    },
                    {"words": ["信呼开发团队"], "type": "word", "name": "rockoa-oa"},
                    {
                        "words": ['技术支持：<a href="http://www.rockoa.com/"'],
                        "type": "word",
                        "name": "rockoa-oa",
                    },
                    {
                        "words": ["http://www.remotecall.com"],
                        "type": "word",
                        "name": "rsupport-remotecall",
                    },
                    {
                        "words": ["msg_abortservice"],
                        "type": "word",
                        "name": "rsupport-remotecall",
                    },
                    {
                        "words": ['content="rcp, 管理平台"'],
                        "type": "word",
                        "name": "ruijie-cloud",
                    },
                    {
                        "words": [
                            '<span class="resource" mark="login.copyright">锐捷网络</span>'
                        ],
                        "type": "word",
                        "name": "ruijie-eweb",
                    },
                    {
                        "words": ["var logincookiename = 'riil_id'"],
                        "type": "word",
                        "name": "ruijie-it",
                    },
                    {
                        "words": [
                            "bbs.ruijie.com.cn",
                            "锐捷统一上网行为管理与审计系统",
                        ],
                        "type": "word",
                        "name": "ruijie-rg-uac",
                        "condition": "and",
                    },
                    {
                        "words": ["src='images/free_login.png'"],
                        "type": "word",
                        "name": "ruijie-rg-uac",
                    },
                    {
                        "part": "header",
                        "words": ["Server: rgos http-server"],
                        "type": "word",
                        "name": "ruijie-rgos",
                    },
                    {
                        "words": ["web_monitor_config.htm", "锐捷网络"],
                        "type": "word",
                        "name": "ruijie-router-nbr",
                        "condition": "and",
                    },
                    {
                        "words": ["无线smartWeb"],
                        "type": "word",
                        "name": "ruijie-smart-web",
                    },
                    {
                        "words": ['document.cookie = "rjsslvpn_encookie=yes;"'],
                        "type": "word",
                        "name": "ruijie-sslvpn",
                    },
                    {
                        "words": ["SSLVPN", "login", "rjsslvpn_encookie"],
                        "type": "word",
                        "name": "ruijie-sslvpn",
                        "condition": "and",
                    },
                    {
                        "words": ['class="log_rbox"'],
                        "type": "word",
                        "name": "runda-supervisory-platform",
                    },
                    {
                        "words": ["/ry-ui.css", "/ry-ui.js"],
                        "type": "word",
                        "name": "ruoyi-system",
                        "condition": "and",
                    },
                    {
                        "words": ["plugins/wbb/barrett.js"],
                        "type": "word",
                        "name": "rusong-product",
                    },
                    {
                        "words": [
                            '<iframe id="ifrm" width="100%" height="100%" frameborder="0" scrolling="no" '
                            'src="/include/login.aspx'
                        ],
                        "type": "word",
                        "name": "ruvar-oa",
                    },
                    {
                        "words": ["<frame", "<iframe "],
                        "type": "word",
                        "name": "ruvar-oa",
                        "condition": "and",
                    },
                    {
                        "words": ['src="/ruvarhrm/web_login/login.aspx'],
                        "type": "word",
                        "name": "ruvarhrm",
                    },
                    {
                        "words": [
                            '<p class="alignright">powered by s:cms - copyright ©',
                            'class="scms_container w1200"',
                        ],
                        "type": "word",
                        "name": "s-cms",
                        "condition": "and",
                    },
                    {
                        "words": ["/media/20151019095214828.png"],
                        "type": "word",
                        "name": "s-cms",
                    },
                    {
                        "words": ["<h2>闪灵cms建站系统</h2>"],
                        "type": "word",
                        "name": "s-cms",
                    },
                    {"words": ["type=news&s_id="], "type": "word", "name": "s-cms"},
                    {"words": ["type=newsinfo&s_id="], "type": "word", "name": "s-cms"},
                    {
                        "words": ["rmsie = /(msie\\s|trident.*rv:)([\\w.]+)/"],
                        "type": "word",
                        "name": "salien-device-integrity-management-system",
                    },
                    {
                        "words": ['href="images/login/cbgl/favicon.ico"'],
                        "type": "word",
                        "name": "salien-performance-management-platform",
                    },
                    {
                        "words": ['content="copyright 2010 www.salien.com.cn"'],
                        "type": "word",
                        "name": "salien-performance-management-platform",
                    },
                    {
                        "words": ['href="images/login/msn/favicon.ico"'],
                        "type": "word",
                        "name": "salien-software-system",
                    },
                    {
                        "words": ["北京市时林电脑公司"],
                        "type": "word",
                        "name": "salien-software-system",
                    },
                    {
                        "words": ["<title>SaltStack</title>"],
                        "type": "word",
                        "name": "saltstack",
                    },
                    {
                        "part": "header",
                        "words": ["Server: TornadoServer"],
                        "type": "word",
                        "name": "saltstack",
                    },
                    {"words": ["songinfo.php"], "type": "word", "name": "samphpweb"},
                    {
                        "words": ["copyright spacial audio solutions"],
                        "type": "word",
                        "name": "samphpweb",
                    },
                    {
                        "words": [
                            'href="http://www.spacialaudio.com/products/sambroadcaster/"'
                        ],
                        "type": "word",
                        "name": "samphpweb",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=playing.html">'
                        ],
                        "type": "word",
                        "name": "samphpweb",
                    },
                    {
                        "words": ["青岛叁度信息技术有限公司"],
                        "type": "word",
                        "name": "sandu-oa",
                    },
                    {
                        "words": [
                            "haovuytkjlnvxpuhsecmbljplpvjz = function(str, key) "
                        ],
                        "type": "word",
                        "name": "sangfor-ba",
                    },
                    {
                        "words": ['<a href="/webauth/">'],
                        "type": "word",
                        "name": "sangfor-behavior-management-or-identity-authentication-system",
                    },
                    {
                        "words": ["身份认证系统"],
                        "type": "word",
                        "name": "sangfor-behavior-management-or-identity-authentication-system",
                    },
                    {
                        "words": ['class="info-inner"'],
                        "type": "word",
                        "name": "sangfor-behavior-management-or-identity-authentication-system",
                    },
                    {
                        "words": ["location.href = '/bbc/index'"],
                        "type": "word",
                        "name": "sangfor-branch-business-center",
                    },
                    {
                        "words": ["acloglogin.php"],
                        "type": "word",
                        "name": "sangfor-data-center",
                    },
                    {
                        "part": "header",
                        "words": ["Location: ./src/acloglogin.php"],
                        "type": "word",
                        "name": "sangfor-data-center",
                    },
                    {
                        "words": ["datalayer','gtm-tl7g2lw'"],
                        "type": "word",
                        "name": "sangfor-edr",
                    },
                    {
                        "words": [" = function(str, key)", 'content="must-revalidate"'],
                        "type": "word",
                        "name": "sangfor-employee-internet-management",
                        "condition": "and",
                    },
                    {
                        "words": ["document.cookie = 'sangfor_session_hash=0'"],
                        "type": "word",
                        "name": "sangfor-employee-internet-management",
                    },
                    {
                        "words": ["上网优化管理"],
                        "type": "word",
                        "name": "sangfor-employee-internet-management",
                    },
                    {
                        "words": ['href="http://sec.sangfor.com.cn/events/89.html"'],
                        "type": "word",
                        "name": "sangfor-employee-internet-management",
                    },
                    {
                        "words": ["internet authentication system"],
                        "type": "word",
                        "name": "sangfor-employee-internet-management",
                    },
                    {
                        "words": ["sangfor-ipsec"],
                        "type": "word",
                        "name": "sangfor-ipsec-vpn",
                    },
                    {
                        "words": ["/cgi-bin/login.cgi?requestname="],
                        "type": "word",
                        "name": "sangfor-managementsystem",
                    },
                    {
                        "words": [
                            "var msg = '对不起，集中管理平台暂不支持您当前使用的浏览器"
                        ],
                        "type": "word",
                        "name": "sangfor-managementsystem",
                    },
                    {
                        "words": [
                            "var msg = '对不起, '+str+'暂不支持您当前使用的浏览器"
                        ],
                        "type": "word",
                        "name": "sangfor-managementsystem",
                    },
                    {
                        "words": [
                            'window.location.href="https://"+window.location.host+"/fort/login'
                        ],
                        "type": "word",
                        "name": "sangfor-osm",
                    },
                    {
                        "words": ["user_blur();"],
                        "type": "word",
                        "name": "sangfor-reporting",
                    },
                    {
                        "words": ["ad report"],
                        "type": "word",
                        "name": "sangfor-reporting",
                    },
                    {
                        "words": ["window.sessionstorage.removeitem('serialcheckobj')"],
                        "type": "word",
                        "name": "sangfor-sip",
                    },
                    {
                        "words": ['src="/apps/secvisual/static/js/runtime.js?'],
                        "type": "word",
                        "name": "sangfor-sip",
                    },
                    {
                        "words": ["url: '../auth_manage/auth_manage/on_login'"],
                        "type": "word",
                        "name": "sangfor-sip",
                    },
                    {
                        "words": ["login_psw.csp"],
                        "type": "word",
                        "name": "sangfor-ssl-vpn",
                    },
                    {
                        "words": [
                            '<li style="color:#999999;margin-left:6px;list-style:circle inside;">如忘记密码，请与防火墙管理员联系</li>'
                        ],
                        "type": "word",
                        "name": "sangfor-tamper-resistance",
                    },
                    {
                        "words": ['href="tamper/style/control.css"'],
                        "type": "word",
                        "name": "sangfor-tamper-resistance",
                    },
                    {
                        "words": ['content="webkit|ie-stand"', 'id="privacytipwin"'],
                        "type": "word",
                        "name": "sangfor-virtualization-management",
                        "condition": "and",
                    },
                    {
                        "words": ['background="images/vista.jpg"'],
                        "type": "word",
                        "name": "sanshuichinatelecombusinesssupportroomsystem",
                    },
                    {
                        "part": "header",
                        "words": ["Server: adaptiveserveranywhere"],
                        "type": "word",
                        "name": "sap-sybase",
                    },
                    {
                        "part": "header",
                        "words": ["Server: sap web application server"],
                        "type": "word",
                        "name": "sap-web-application-server",
                    },
                    {
                        "words": [
                            "<title>Application Server Error</title>",
                            "SAP logo",
                        ],
                        "type": "word",
                        "name": "sap-web-application-server",
                        "condition": "and",
                    },
                    {
                        "part": "header",
                        "words": ["Server: sap netweaver application server"],
                        "type": "word",
                        "name": "sap-web-application-server",
                    },
                    {
                        "part": "header",
                        "words": ["Server: sap j2ee engine"],
                        "type": "word",
                        "name": "sap-web-application-server",
                    },
                    {
                        "words": ['class="logo"><a href="http://sarg.sourceforge.net"'],
                        "type": "word",
                        "name": "sarg",
                    },
                    {
                        "part": "header",
                        "words": ["Server: savant"],
                        "type": "word",
                        "name": "savant-web-server",
                    },
                    {
                        "words": ['<div id="password:error'],
                        "type": "word",
                        "name": "sawmill",
                    },
                    {
                        "words": ['src="/picts/sawmill_logo.png'],
                        "type": "word",
                        "name": "sawmill",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0; url=/citectscada">'
                        ],
                        "type": "word",
                        "name": "schneider-citectscada",
                    },
                    {
                        "words": [
                            'content="start services, start group, the start group, automation, industrial, software '
                            "engineering, scada, plc, rtu, rockwell, rockwell automation, allen-bradley, allen bradley, "
                            "allenbradley, citect, citectscada, kingfisher"
                        ],
                        "type": "word",
                        "name": "schneider-citectscada",
                    },
                    {
                        "words": ['content: "/lfsms/user/login2?go=" + go'],
                        "type": "word",
                        "name": "scientific-research-instrument-network-service-platform",
                    },
                    {
                        "words": ["sco corporate web site"],
                        "type": "word",
                        "name": "sco-openserver",
                    },
                    {
                        "words": ["gif/rlogo1.gif"],
                        "type": "word",
                        "name": "sco-openserver",
                    },
                    {
                        "words": ['powered by <a class="externallink" href="http:'],
                        "type": "word",
                        "name": "screwturn-wiki-web-service",
                    },
                    {
                        "words": [
                            'title="screwturn wiki" target="_blank">screwturn wiki'
                        ],
                        "type": "word",
                        "name": "screwturn-wiki-web-service",
                    },
                    {
                        "words": ['<em class="wk-fl">在沃系统</em>'],
                        "type": "word",
                        "name": "sdcncsi",
                    },
                    {
                        "words": ["refreshcaptchaimgfun"],
                        "type": "word",
                        "name": "sdcncsi",
                    },
                    {
                        "words": ["css/seahub.min.css"],
                        "type": "word",
                        "name": "seafile",
                    },
                    {
                        "words": ["var sgl_js_sessid"],
                        "type": "word",
                        "name": "seagull-php-framework",
                    },
                    {
                        "words": ["/secbiutils.min.js"],
                        "type": "word",
                        "name": "secbi-product",
                    },
                    {
                        "words": [
                            "use this if you want to run the seceon module of kibana."
                        ],
                        "type": "word",
                        "name": "seceon-otm",
                    },
                    {
                        "words": ["nanjing rickyinfo technology"],
                        "type": "word",
                        "name": "security-intelligent-management-platform",
                    },
                    {
                        "words": ['href="upvalidatefile.aspx'],
                        "type": "word",
                        "name": "seehealth-health-management-system",
                    },
                    {
                        "words": ["www.smartlogic.com"],
                        "type": "word",
                        "name": "semaphore",
                    },
                    {
                        "words": ['alt="powered by semaphore"'],
                        "type": "word",
                        "name": "semaphore",
                    },
                    {
                        "words": [" sentora_logo.png "],
                        "type": "word",
                        "name": "sentora",
                    },
                    {
                        "words": [' href="http://www.sentora.org/"'],
                        "type": "word",
                        "name": "sentora",
                    },
                    {
                        "words": [
                            'src: url(/static/fonts/lcdd.eot) format("truetype"), url(/static/fonts/lcdd.ttf) '
                            'format("truetype");'
                        ],
                        "type": "word",
                        "name": "sentry-cloud",
                    },
                    {
                        "words": [
                            '<p class="note error">javascript is turned off in your web browser. turn it on to take full '
                            "advantage of this site, then refresh the page.</p>"
                        ],
                        "type": "word",
                        "name": "seo-panel",
                    },
                    {
                        "words": [
                            "var wantproceed = 'do you really want to proceed?';",
                            "var wantproceed = 'wollen sie wirklich fortfahren?';",
                        ],
                        "type": "word",
                        "name": "seo-panel",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'class="btn btn-default" onclick="updateconnect();">'
                        ],
                        "type": "word",
                        "name": "sequoiadb-",
                    },
                    {
                        "words": ["serendipity_editor.js"],
                        "type": "word",
                        "name": "serendipity-php-architecture",
                    },
                    {
                        "words": ['alt="serv-u ftp server'],
                        "type": "word",
                        "name": "serv-u-ftp",
                    },
                    {
                        "words": ["/static/all/img/logo/sgp"],
                        "type": "word",
                        "name": "sgp-managerial-system",
                    },
                    {
                        "words": ["/dist/libs/layui/css/layui.css"],
                        "type": "word",
                        "name": "shanshanes-system",
                    },
                    {
                        "words": ["amap.mousetool,amap.districtsearch"],
                        "type": "word",
                        "name": "shanshanes-system",
                    },
                    {
                        "words": ["<title>杉杉储能监控平台</title>"],
                        "type": "word",
                        "name": "shanshanes-system",
                    },
                    {
                        "words": [
                            'content: "获取一证通信息异常。请检查数字证书是否正常运行'
                        ],
                        "type": "word",
                        "name": "sheca-cert",
                    },
                    {
                        "words": ['<li class="in" id="cert_li">'],
                        "type": "word",
                        "name": "sheca-cert",
                    },
                    {
                        "words": ['var granttype="authorization_code'],
                        "type": "word",
                        "name": "shenguyun-sgc8000",
                    },
                    {
                        "words": ["/frameworks/images/ico.ico"],
                        "type": "word",
                        "name": "shenri-elevator",
                    },
                    {
                        "words": ["images/login/btnmobile.gif"],
                        "type": "word",
                        "name": "shenri-elevator",
                    },
                    {
                        "words": [
                            "copyright 2015 shenri habiliment co.,ltd. all rights reserved"
                        ],
                        "type": "word",
                        "name": "shenri-elevator",
                    },
                    {
                        "words": ['href="xmsenv.exe">系统运行环境'],
                        "type": "word",
                        "name": "shiji-xms",
                    },
                    {
                        "words": ['id="www_zzjs_net_loding"'],
                        "type": "word",
                        "name": "shop7z",
                    },
                    {
                        "words": ["function www_zzjs_net_change"],
                        "type": "word",
                        "name": "shop7z",
                    },
                    {
                        "words": ['content="mallbuilder', "powered by mallbuilder"],
                        "type": "word",
                        "name": "shop_builder-mallbuilder",
                        "condition": "and",
                    },
                    {
                        "words": ['content="shopbuilder', "powered by shopbuilder"],
                        "type": "word",
                        "name": "shop_builder-shopbuilder",
                        "condition": "and",
                    },
                    {
                        "words": ["shopbuilder版权所有"],
                        "type": "word",
                        "name": "shop_builder-shopbuilder",
                    },
                    {"words": ['content="shopex'], "type": "word", "name": "shopex"},
                    {
                        "words": ["@author litie[aita]shopex.cn"],
                        "type": "word",
                        "name": "shopex",
                    },
                    {
                        "words": ["http://www.shopex.cn' target='_blank'"],
                        "type": "word",
                        "name": "shopex",
                    },
                    {"words": ["powered by shopnc"], "type": "word", "name": "shopnc"},
                    {
                        "words": ["copyright 2007-2014 shopnc inc"],
                        "type": "word",
                        "name": "shopnc",
                    },
                    {"words": ['content="shopnc'], "type": "word", "name": "shopnc"},
                    {"words": ['content="shopsn'], "type": "word", "name": "shopsn"},
                    {
                        "words": [
                            'href="http://www.shopsn.net">商城系统</a>&nbsp;提供技术支持'
                        ],
                        "type": "word",
                        "name": "shopsn",
                    },
                    {
                        "words": [
                            '<span>shopsn全网开源<a style="padding: 0px" href="http://www.shopsn.net'
                        ],
                        "type": "word",
                        "name": "shopsn",
                    },
                    {
                        "words": [
                            'Powered by <a href="http://shopxo.net/" title="ShopXO电商系统" target="_blank">'
                        ],
                        "type": "word",
                        "name": "shopxo",
                    },
                    {
                        "words": ["content=本网站基于开源版showdoc搭建"],
                        "type": "word",
                        "name": "showdoc",
                    },
                    {
                        "words": ['for="ctl00_cph_l_login_username">crm帐号'],
                        "type": "word",
                        "name": "shunde-cms",
                    },
                    {
                        "words": ["关于正则表达式的内容在本书的第10章中有较详细的讨论"],
                        "type": "word",
                        "name": "shwatermeter-system",
                    },
                    {
                        "words": ["$.cookie('sianglng' , null)"],
                        "type": "word",
                        "name": "siangsoft-filesystem",
                    },
                    {
                        "words": ["版权所有：北京格尔国信科技有限公司"],
                        "type": "word",
                        "name": "signature-verification-management-system",
                    },
                    {"words": ["var slsbuild"], "type": "word", "name": "sillysmart"},
                    {
                        "words": ["framework/javascript/htmleditorfield.js"],
                        "type": "word",
                        "name": "silverstripe",
                    },
                    {
                        "words": ['content="silverstripe/'],
                        "type": "word",
                        "name": "silverstripe",
                    },
                    {
                        "words": ['content="simbix framework v'],
                        "type": "word",
                        "name": "simbix-framework",
                    },
                    {
                        "words": ["logo-lpage-owner.png"],
                        "type": "word",
                        "name": "simbix-framework",
                    },
                    {
                        "words": [
                            '<div class="image"><img src="/logo-lpage.png" width="40" height="40" alt="simbix framework" '
                            "/></div>"
                        ],
                        "type": "word",
                        "name": "simbix-framework",
                    },
                    {
                        "words": [
                            '<span id="spt"><a href="http://www.sptoolkit.com/download/" target="_blank">'
                        ],
                        "type": "word",
                        "name": "simple-phishing-toolkit",
                    },
                    {
                        "words": [
                            'content="welcome to spt - simple phishing toolkit.  spt is a super simple but powerful '
                            'phishing toolkit." />'
                        ],
                        "type": "word",
                        "name": "simple-phishing-toolkit",
                    },
                    {
                        "words": [
                            '<form onsubmit="sendinfo(); return false;" name="logon'
                        ],
                        "type": "word",
                        "name": "simsweb",
                    },
                    {
                        "words": ['src="/simsweb/monitor.js'],
                        "type": "word",
                        "name": "simsweb",
                    },
                    {
                        "words": [
                            'index.html"><font color="black" face="arial">loading simsweb, please '
                            "wait.....</font></a></h2>"
                        ],
                        "type": "word",
                        "name": "simsweb",
                    },
                    {"words": ["lib.sinaapp.com"], "type": "word", "name": "sina-sae"},
                    {
                        "words": ["app_themes/1/style.css"],
                        "type": "word",
                        "name": "sinosoft-technology-e-government-system",
                    },
                    {
                        "words": ['window.location = "homepages/index.aspx'],
                        "type": "word",
                        "name": "sinosoft-technology-e-government-system",
                    },
                    {
                        "words": ["homepages/content_page.aspx"],
                        "type": "word",
                        "name": "sinosoft-technology-e-government-system",
                    },
                    {
                        "words": ['content="boka siteengine'],
                        "type": "word",
                        "name": "siteengine",
                    },
                    {
                        "words": [
                            "var portalbrowser = window.open('popup.php?page_type='+page_type+'&lang="
                        ],
                        "type": "word",
                        "name": "sitegenius",
                    },
                    {
                        "words": ["http://www.siteserver.cn", "powered by"],
                        "type": "word",
                        "name": "siteserver",
                        "condition": "and",
                    },
                    {"words": ["siteserver cms"], "type": "word", "name": "siteserver"},
                    {
                        "words": ["siteserver", "t_系统首页模板"],
                        "type": "word",
                        "name": "siteserver",
                        "condition": "and",
                    },
                    {"words": ["sitefiles"], "type": "word", "name": "siteserver"},
                    {
                        "words": [
                            '<table border="0" width="100%" id="logobanner"><tr width="100%"><td  width="82%"><img '
                            'src="https://customer.skillport.com/spcustom/'
                        ],
                        "type": "word",
                        "name": "skillsoft-skillport-lms",
                    },
                    {
                        "words": [
                            '<div id="simple-footer"><span>slideshowpro director'
                        ],
                        "type": "word",
                        "name": "slideshowpro-director",
                    },
                    {
                        "words": ["</div> <!--close login-container--></body>"],
                        "type": "word",
                        "name": "slideshowpro-director",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0; url=/culture/index.dml">'
                        ],
                        "type": "word",
                        "name": "sma-sunny_webbox",
                    },
                    {
                        "words": ['name="formdoc" action="/hz/sys/login/logined.jsp'],
                        "type": "word",
                        "name": "smart-cms",
                    },
                    {
                        "words": ["gcfutil = jsloader.resolve('smartbi.gcf.gcfutil')"],
                        "type": "word",
                        "name": "smartbi",
                    },
                    {
                        "part": "header",
                        "words": ["Server: smartcds"],
                        "type": "word",
                        "name": "smartcds",
                    },
                    {
                        "words": ['id="sitenametitle">智慧城建项目管理系统</p'],
                        "type": "word",
                        "name": "smartcityconstructionprojectmanagementsystem",
                    },
                    {
                        "words": ['class="container_huanlegendtext"'],
                        "type": "word",
                        "name": "smartcommunityintegratedmanagementcloudplatform",
                    },
                    {
                        "words": [
                            "<a href='http://www.smartertools.com/smartermail/mail-server-software.aspx' "
                            "target='_blank>smartermail"
                        ],
                        "type": "word",
                        "name": "smartermail",
                    },
                    {
                        "words": ["<td class=bar1inner>smarterstats"],
                        "type": "word",
                        "name": "smarterstats",
                    },
                    {
                        "words": [
                            "login - smartermail",
                            "<a href='http://www.smartertools.com/smartermail/mail-server-software.aspx' "
                            "target='_blank'>smartermail",
                        ],
                        "type": "word",
                        "name": "smartertools-smartermail",
                        "condition": "and",
                    },
                    {"words": ["/smartoa.plist"], "type": "word", "name": "smartoa"},
                    {
                        "words": [
                            'href="http://www.smartoa.com.cn/download/smartoa.apk">安卓客户端</a>'
                        ],
                        "type": "word",
                        "name": "smartoa",
                    },
                    {
                        "words": [
                            'powered by<a href="http://www.smart-scripts.com">smart thumbs</a>'
                        ],
                        "type": "word",
                        "name": "smartthumbs",
                    },
                    {
                        "words": [
                            '<a href="http://www.simplemachines.org/about/copyright.php" title="free forum software" '
                            'target="_blank'
                        ],
                        "type": "word",
                        "name": "smf",
                    },
                    {
                        "words": ['<img class="floatright" id="smflogo" src='],
                        "type": "word",
                        "name": "smf",
                    },
                    {
                        "words": [
                            'document.getelementbyid("upshrink").src = smf_images_url + '
                        ],
                        "type": "word",
                        "name": "smf",
                    },
                    {
                        "words": [
                            '<a href="http://oss.oetiker.ch/smokeping/counter.cgi/'
                        ],
                        "type": "word",
                        "name": "smokeping",
                    },
                    {
                        "words": [
                            '<tr><td class="menuitem" colspan="2">&nbsp;-&nbsp;<a class="menulink" href="?target='
                        ],
                        "type": "word",
                        "name": "smokeping",
                    },
                    {
                        "words": ['copyright 2005–2009 <a href="http://www.s-mo.com">'],
                        "type": "word",
                        "name": "snb-stock-trading-software",
                    },
                    {
                        "words": ["diag.htm?src=index"],
                        "type": "word",
                        "name": "socomec-webserver",
                    },
                    {
                        "words": ["egss_user"],
                        "type": "word",
                        "name": "soeasy-website-cluster-system",
                    },
                    {
                        "words": ["<title>SOFFICE登录</title>"],
                        "type": "word",
                        "name": "soffice",
                    },
                    {
                        "words": ["offlineservice/showdownloadmessage.aspx"],
                        "type": "word",
                        "name": "soft78-system",
                    },
                    {"words": ["add by sll"], "type": "word", "name": "soft78-system"},
                    {
                        "words": ['powered by <a href="http://www.softbizscripts.com'],
                        "type": "word",
                        "name": "softbiz-online-auctions-script",
                    },
                    {
                        "words": [
                            "http://www.softbizscripts.com/classified-ads-plus-script-features.php"
                        ],
                        "type": "word",
                        "name": "softbiz-online-classifieds",
                    },
                    {
                        "case-insensitive": True,
                        "words": ["<li>manage this vpn server or vpn bridge<ul>"],
                        "type": "word",
                        "name": "softether-vpn",
                    },
                    {
                        "words": ["snspam/spam_request/"],
                        "type": "word",
                        "name": "softnext-spam",
                    },
                    {
                        "words": ["snspam/start_page.asp"],
                        "type": "word",
                        "name": "softnext-spam",
                    },
                    {
                        "words": ["spam_request/spam_requestact.asp"],
                        "type": "word",
                        "name": "softnext-spam",
                    },
                    {
                        "words": ["solarwinds"],
                        "type": "word",
                        "name": "solarwinds-traffic-management",
                    },
                    {
                        "words": ['<meta http-equiv="refresh" content="0; url=/hmi/">'],
                        "type": "word",
                        "name": "solidyne-inet-server",
                    },
                    {
                        "words": [
                            '<frame name="frleft" scrolling="no" id="frleft" src="qfrleft.aspx">'
                        ],
                        "type": "word",
                        "name": "solidyne-inet-server",
                    },
                    {
                        "words": ['content="sonarqube'],
                        "type": "word",
                        "name": "sonarqube",
                    },
                    {
                        "words": [
                            '<meta name="author" content="sony pictures networks india pvt. ltd">'
                        ],
                        "type": "word",
                        "name": "sony-liv",
                    },
                    {
                        "words": [
                            '<meta name="twitter:image" content="http://sonyliv.com/asset/socialsharelogo"/>'
                        ],
                        "type": "word",
                        "name": "sony-liv",
                    },
                    {
                        "words": ['content="sony liv'],
                        "type": "word",
                        "name": "sony-liv",
                    },
                    {
                        "words": ['src="/images/customizeimages/uploadedlogo.jpg"'],
                        "type": "word",
                        "name": "sophos-cyberoam-ssl-vpn",
                    },
                    {
                        "words": [
                            "sslvpnuserportalloginform",
                            "cyberoam ssl vpn portal",
                        ],
                        "type": "word",
                        "name": "sophos-cyberoam-sslvpn",
                        "condition": "and",
                    },
                    {
                        "words": ["powered by utm web protection"],
                        "type": "word",
                        "name": "sophos-utm-web-protection",
                    },
                    {
                        "words": ["url(resources/images/en/login_swa.jpg)"],
                        "type": "word",
                        "name": "sophos-web-appliance",
                    },
                    {
                        "words": ["pages/login_@del.jsp"],
                        "type": "word",
                        "name": "sosen-airsoft",
                    },
                    {
                        "words": ['<div id="footqversion">version('],
                        "type": "word",
                        "name": "sourcebans",
                    },
                    {
                        "words": [
                            'http://www.sourcebans.net" target="_blank"><img src="images/sb.png'
                        ],
                        "type": "word",
                        "name": "sourcebans",
                    },
                    {
                        "words": [
                            'document.getelementbyid("redirectform").action = "../mxworkspace/login.aspx'
                        ],
                        "type": "word",
                        "name": "sourcecode-k2",
                    },
                    {
                        "words": [
                            'document.getelementbyid("redirectform").action = "../workspace/default.aspx'
                        ],
                        "type": "word",
                        "name": "sourcecode-k2",
                    },
                    {"words": ["/southidckefu.js"], "type": "word", "name": "southidc"},
                    {
                        "words": ['content="copyright 2003-2015 - southidc.net'],
                        "type": "word",
                        "name": "southidc",
                    },
                    {"words": ["/southidcj2f.js"], "type": "word", "name": "southidc"},
                    {
                        "words": [
                            '<table class="lhead"><tr><td class="img"><img src="/imgs/logo.gif" alt="spamtitan '
                            'logo"></td></tr></table></div>'
                        ],
                        "type": "word",
                        "name": "spamtitan",
                    },
                    {"words": ["serversparkversion"], "type": "word", "name": "spark"},
                    {
                        "words": ['src="/static/historypage-common.js">'],
                        "type": "word",
                        "name": "spark-history-server",
                    },
                    {
                        "words": ['href="/jobs/job?id='],
                        "type": "word",
                        "name": "spark-jobs",
                    },
                    {"words": ["spark jobs"], "type": "word", "name": "spark-jobs"},
                    {
                        "words": ['var unitid = getpagerequestvalue("unitid")'],
                        "type": "word",
                        "name": "special-device-testing-risk-assessment-system",
                    },
                    {
                        "words": [
                            'classid="clsid:53028063-426b-446d-85f9-62e9e3dc5501"'
                        ],
                        "type": "word",
                        "name": "speed-test",
                    },
                    {
                        "words": ['<meta name="author" content="spiceworks, inc." />'],
                        "type": "word",
                        "name": "spiceworks",
                    },
                    {
                        "words": ['<link href="/stylesheets/general.css?'],
                        "type": "word",
                        "name": "spiceworks",
                    },
                    {
                        "words": [
                            '<h1><img alt="spiceworks" src="/images/logos/large.png'
                        ],
                        "type": "word",
                        "name": "spiceworks",
                    },
                    {
                        "words": ['content="Splunk Inc."'],
                        "type": "word",
                        "name": "splunk",
                    },
                    {
                        "part": "header",
                        "words": ["Server: Splunkd", "Set-Cookie: splunkweb_uid"],
                        "type": "word",
                        "name": "splunk",
                    },
                    {
                        "words": ["<title>303 See Other</title>"],
                        "type": "word",
                        "name": "splunk",
                    },
                    {
                        "part": "header",
                        "words": ["Server: Splunkd", "Set-Cookie: ROUTEID"],
                        "type": "word",
                        "name": "splunk",
                    },
                    {
                        "words": ["<title>splunkd</title>"],
                        "type": "word",
                        "name": "splunkd",
                    },
                    {
                        "part": "header",
                        "words": ["Server: Splunkd"],
                        "type": "word",
                        "name": "splunkd",
                    },
                    {
                        "words": ["spring boot admin"],
                        "type": "word",
                        "name": "spring-boot-admin",
                    },
                    {
                        "words": ["<h1>whitelabel error page</h1>"],
                        "type": "word",
                        "name": "spring-framework",
                    },
                    {
                        "words": ['src="//static.springer.com/'],
                        "type": "word",
                        "name": "springer",
                    },
                    {
                        "words": ["3cspan id='cnzz_stat_icon_1274142628"],
                        "type": "word",
                        "name": "sq580-system",
                    },
                    {
                        "words": ['h3><a href="http://www.sqlbuddy.com/help'],
                        "type": "word",
                        "name": "sql-buddy",
                    },
                    {
                        "words": ['href="themes/bittersweet/css/main.css'],
                        "type": "word",
                        "name": "sql-buddy",
                    },
                    {
                        "words": [
                            'new squarespace.fixedpositiontip("logout successful", "you have been successfully logged '
                            'out.", { xmargin: 15, ymargin: 15, icon: "/universal/images/helptip-info.png", orientation: '
                            '"upper-right", viewportfixed: true, autohide: 1800 }).show();'
                        ],
                        "type": "word",
                        "name": "squarespace",
                    },
                    {
                        "words": ["function squirrelmail_loginpage_onload()"],
                        "type": "word",
                        "name": "squirrelmail",
                    },
                    {
                        "words": ["players/js/winlin.utility.js"],
                        "type": "word",
                        "name": "srs-live-server",
                    },
                    {
                        "words": ['window.location = "/user/home.jsx";'],
                        "type": "word",
                        "name": "staragent",
                    },
                    {
                        "words": ['href="/aisc/aisc.css'],
                        "type": "word",
                        "name": "staragent",
                    },
                    {
                        "words": [
                            '<tr><td><a href="http://www.stardot.com" target="_blank"><img src="logo.gif" alt="" '
                            'width="227" height="45"></a></td>'
                        ],
                        "type": "word",
                        "name": "stardot-express",
                    },
                    {
                        "words": ['content="startbbs'],
                        "type": "word",
                        "name": "startbbs",
                    },
                    {
                        "words": ['src="img/portfolio/thumbnails/4.jpg"'],
                        "type": "word",
                        "name": "startbootstrap-product",
                    },
                    {
                        "words": ["we've got what you need!</h2>"],
                        "type": "word",
                        "name": "startbootstrap-product",
                    },
                    {
                        "words": [
                            '<p>this site is powered by <a href="http://status.net/">statusnet</a> version'
                        ],
                        "type": "word",
                        "name": "statusnet",
                    },
                    {
                        "words": [
                            'it runs the <a href="http://status.net/">statusnet</a> microblogging software, version '
                        ],
                        "type": "word",
                        "name": "statusnet",
                    },
                    {
                        "words": ["firmware version: stor_2.0"],
                        "type": "word",
                        "name": "storage-management",
                    },
                    {"words": ["stormtimestr"], "type": "word", "name": "storm"},
                    {
                        "words": ["struts problem report"],
                        "type": "word",
                        "name": "struts2",
                    },
                    {
                        "words": ["there is no action mapped for namespace"],
                        "type": "word",
                        "name": "struts2",
                    },
                    {
                        "words": ["no result defined for action and result input"],
                        "type": "word",
                        "name": "struts2",
                    },
                    {
                        "words": ['href="http://www.subrion.com'],
                        "type": "word",
                        "name": "subrion-cms",
                    },
                    {
                        "words": ['content="subrion cms'],
                        "type": "word",
                        "name": "subrion-cms",
                    },
                    {
                        "words": ["subsonic"],
                        "type": "word",
                        "name": "subsonic-as-subsonic",
                    },
                    {
                        "words": [
                            "onload=\"document.getelementbyid('j_username').focus()\""
                        ],
                        "type": "word",
                        "name": "subsonic-as-subsonic",
                    },
                    {
                        "words": [
                            'parent.frames.upper.keyboardshortcut("showindex", index.touppercase());'
                        ],
                        "type": "word",
                        "name": "subsonic-as-subsonic",
                    },
                    {
                        "words": [
                            "sucuri website firewall - cloudproxy - access denied"
                        ],
                        "type": "word",
                        "name": "sucuri",
                    },
                    {
                        "words": ["cloudproxy@sucuri.net"],
                        "type": "word",
                        "name": "sucuri",
                    },
                    {
                        "words": [
                            "<a href=\" javascript:void window.open('http://support.sugarcrm.com')\">support</a>"
                        ],
                        "type": "word",
                        "name": "sugarcrm",
                    },
                    {
                        "words": [
                            "<img style='margin-top: 2px' border='0' width='106' height='23' "
                            "src='include/images/poweredby_sugarcrm.png' alt='powered by sugarcrm'>"
                        ],
                        "type": "word",
                        "name": "sugarcrm",
                    },
                    {
                        "words": [
                            "<script>var module_sugar_grp1 = 'users';</script><script>var action_sugar_grp1 = "
                            "'login';</script><script>jscal_today"
                        ],
                        "type": "word",
                        "name": "sugarcrm",
                    },
                    {
                        "words": ["sugar.themes.theme_name = 'suitep'"],
                        "type": "word",
                        "name": "suitecrm",
                    },
                    {
                        "words": ["sugar.themes.theme_name      = 'suiter'"],
                        "type": "word",
                        "name": "suitecrm",
                    },
                    {
                        "words": [
                            'src="img/suitecrm.png" alt="bitnami suitecrm stack"'
                        ],
                        "type": "word",
                        "name": "suitecrm",
                    },
                    {
                        "words": ["supercharged by suitecrm"],
                        "type": "word",
                        "name": "suitecrm",
                    },
                    {
                        "words": ['src="custom/themes/default/images/company_logo.png'],
                        "type": "word",
                        "name": "suitecrm",
                    },
                    {"words": ['alt="suitecrm"'], "type": "word", "name": "suitecrm"},
                    {
                        "part": "header",
                        "words": ["Set-Cookie: sugar_user_theme=suitep"],
                        "type": "word",
                        "name": "suitecrm",
                    },
                    {
                        "words": ["glassfish community", "webui/jsf"],
                        "type": "word",
                        "name": "sun-glassfish",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<img src="imx/login-logo.gif" width="186" height="79" alt="sun microsystems, inc.">'
                        ],
                        "type": "word",
                        "name": "sun-java-system-calendar-express",
                    },
                    {
                        "part": "header",
                        "words": ["Server: sun-web-server"],
                        "type": "word",
                        "name": "sun-web-server",
                    },
                    {
                        "words": ["/sunbox/assets/js/ace.min.js"],
                        "type": "word",
                        "name": "sunboxsoft-marketing",
                    },
                    {
                        "words": ["技术支持：广东旭诚科技有限公司"],
                        "type": "word",
                        "name": "suncere-system",
                    },
                    {
                        "words": ["尚景流量管理平台"],
                        "type": "word",
                        "name": "sungoin-traffic-management-platform",
                    },
                    {
                        "words": ["速达软件技术（广州）有限公司"],
                        "type": "word",
                        "name": "superdata-oa",
                    },
                    {
                        "words": ["北京超图软件股份有限公司"],
                        "type": "word",
                        "name": "supermap-gis",
                    },
                    {
                        "words": ['window.location.href="iserver";'],
                        "type": "word",
                        "name": "supermap-iserver",
                    },
                    {
                        "words": ['id="copyright"><a href="http://www.supermap.com.cn'],
                        "type": "word",
                        "name": "supermap-iserver",
                    },
                    {
                        "words": ['id="ctl00_topcontrol1_lblsitedescription"'],
                        "type": "word",
                        "name": "superv-meeting",
                    },
                    {
                        "words": ["images/supervisor.gif"],
                        "type": "word",
                        "name": "supervisord",
                    },
                    {
                        "words": ["<div class='windowtitle'>sit! - login</div>"],
                        "type": "word",
                        "name": "support-incident-tracker",
                    },
                    {
                        "words": ['content="sit! support incident tracker'],
                        "type": "word",
                        "name": "support-incident-tracker",
                    },
                    {
                        "words": ["<h1>欢迎使用360书生云盘！</h1>"],
                        "type": "word",
                        "name": "surdoc",
                    },
                    {
                        "words": ["<p>copyright@2016 pan.surdoc.net all rights</p>"],
                        "type": "word",
                        "name": "surdoc",
                    },
                    {
                        "words": ['content="北京国信安邮科技有限公司'],
                        "type": "word",
                        "name": "suremail",
                    },
                    {
                        "words": ["<span> 客服邮箱：support@suremail.cn</span>"],
                        "type": "word",
                        "name": "suremail",
                    },
                    {
                        "words": ["/ws2004/public/"],
                        "type": "word",
                        "name": "suyaxing-campus-management-system",
                    },
                    {"words": ["Swagger UI"], "type": "word", "name": "swagger"},
                    {"words": ["swagger-ui.css"], "type": "word", "name": "swagger"},
                    {"words": ["swagger-ui.js"], "type": "word", "name": "swagger"},
                    {
                        "words": ['content="sweetrice'],
                        "type": "word",
                        "name": "sweetrice",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.basic-cms.org">basic cms sweetrice</a>'
                        ],
                        "type": "word",
                        "name": "sweetrice",
                    },
                    {
                        "words": [
                            '<a href="http://minnow.cc.gatech.edu/swiki" title="comswiki: powered by squeak"><img '
                            'src="/defaultscheme/comswiki.gif" border=0 width=277 height=88 alt="comswiki: powered by '
                            'squeak"></a><br>'
                        ],
                        "type": "word",
                        "name": "swiki",
                    },
                    {
                        "words": [
                            'alert("ihr suchbegriff muss mind. aus 3 zeichen bestehen.");'
                        ],
                        "type": "word",
                        "name": "sx-shop",
                    },
                    {
                        "words": ['content="source worx - software development">'],
                        "type": "word",
                        "name": "sx-shop",
                    },
                    {
                        "words": ['<div class="head_right" onclick="window_close()">'],
                        "type": "word",
                        "name": "sxt234-system",
                    },
                    {
                        "words": [
                            'src="/public/sec/assets/js/libs/jquery.placeholder.min.js">'
                        ],
                        "type": "word",
                        "name": "sxzrouter-caching",
                    },
                    {
                        "words": [' href="http://www.dwcache.com"'],
                        "type": "word",
                        "name": "sxzrouter-caching",
                    },
                    {
                        "words": [
                            "<!-- symantec client security web based installation -->"
                        ],
                        "type": "word",
                        "name": "symantec-client-security",
                    },
                    {
                        "words": [
                            '<div style="font-family: tahoma, verdana, arial, helvetica, sans-serif; '
                            'font-size:11px;">version'
                        ],
                        "type": "word",
                        "name": "symantec-endpoint-protection-manager",
                    },
                    {
                        "words": [
                            "/portal/about.jsp",
                            "<!-- now, if it is ie on windows platform, we check to see which version of jws is "
                            "installed -->",
                        ],
                        "type": "word",
                        "name": "symantec-endpoint-protection-manager",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<tr><td align="left" style="font-family:arial; font-size:18pt"><b>symantec endpoint '
                            "protection manager<br>web access</b></td></tr>"
                        ],
                        "type": "word",
                        "name": "symantec-endpoint-protection-manager",
                    },
                    {
                        "words": ["https://seal.thawte.com/getthawteseal"],
                        "type": "word",
                        "name": "symantec-thawte_ssl_cert",
                    },
                    {
                        "words": ["powered by symfony"],
                        "type": "word",
                        "name": "symfony",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.symfony-project.org/">'
                        ],
                        "type": "word",
                        "name": "symfony",
                    },
                    {
                        "words": ['content="sympa'],
                        "type": "word",
                        "name": "sympa-mailing-list-server",
                    },
                    {
                        "words": ['alt="sympa logo'],
                        "type": "word",
                        "name": "sympa-mailing-list-server",
                    },
                    {
                        "words": [
                            '<meta name="application-name" content="Synology DiskStation'
                        ],
                        "type": "word",
                        "name": "synology-diskstation-nas",
                    },
                    {
                        "words": ['class="logo-synology"'],
                        "type": "word",
                        "name": "synology-dms",
                    },
                    {
                        "words": ['class="logo-dsm"'],
                        "type": "word",
                        "name": "synology-dms",
                    },
                    {
                        "words": ["photo_new/syno_photo_main.js"],
                        "type": "word",
                        "name": "synology-photo-station",
                    },
                    {
                        "words": ['content="photo station 6"'],
                        "type": "word",
                        "name": "synology-photo-station",
                    },
                    {
                        "words": ['content="service_not_available"'],
                        "type": "word",
                        "name": "synology-photo-station",
                    },
                    {
                        "words": ['content="photo station'],
                        "type": "word",
                        "name": "synology-photo-station",
                    },
                    {
                        "words": ['content="album'],
                        "type": "word",
                        "name": "synology-photo-station",
                    },
                    {
                        "words": ['hostname" : "synologyrouter"'],
                        "type": "word",
                        "name": "synology-router-manager",
                    },
                    {
                        "words": ['content="synologyrouter'],
                        "type": "word",
                        "name": "synology-router-manager",
                    },
                    {
                        "words": [
                            'id="paragraph">web station has been enabled. to finish setting up your website, please see '
                            'the "web service'
                        ],
                        "type": "word",
                        "name": "synology-webstation",
                    },
                    {
                        "words": [
                            '<div id="footer_copyright" class="shade footer_copyright">powered by <a '
                            'href="http://www.wolfshead-solutions.com/ws-products/product-1'
                        ],
                        "type": "word",
                        "name": "tab-and-link-manager",
                    },
                    {"words": ["template/taocms"], "type": "word", "name": "taocms"},
                    {
                        "words": ['src="/taocms/code/calendar.js"'],
                        "type": "word",
                        "name": "taocms",
                    },
                    {
                        "words": ['<a href="http://www.taskfreak.com">taskfreak'],
                        "type": "word",
                        "name": "taskfreak",
                    },
                    {
                        "words": ["灵志日志审计系统"],
                        "type": "word",
                        "name": "tass-smart-auditor",
                    },
                    {
                        "words": ["北京江南天安科技有限公司"],
                        "type": "word",
                        "name": "tass-smart-auditor",
                    },
                    {"words": ['alt="plexpy"'], "type": "word", "name": "tautulli"},
                    {
                        "words": ["<div class='log-dvr-logo'>"],
                        "type": "word",
                        "name": "tbk-dvr",
                    },
                    {
                        "words": ["index.php?ac=link_more"],
                        "type": "word",
                        "name": "tccms",
                    },
                    {
                        "words": ["index.php?ac=news_list"],
                        "type": "word",
                        "name": "tccms",
                    },
                    {
                        "words": ['<a name="topofdoc" id="topofdoc"></a>'],
                        "type": "word",
                        "name": "tcexam",
                    },
                    {
                        "words": ['content="nicola asuni - tecnick.com s.r.l." />'],
                        "type": "word",
                        "name": "tcexam",
                    },
                    {
                        "words": ["<a href=\"licence.asp\"><b style='color:#ff9900'>"],
                        "type": "word",
                        "name": "team-board",
                    },
                    {
                        "words": ['href=http://www.team5.cn"><b>team '],
                        "type": "word",
                        "name": "team-board",
                    },
                    {
                        "words": ["team5.cn by daymoon"],
                        "type": "word",
                        "name": "team-board",
                    },
                    {
                        "words": ['id="labellink"'],
                        "type": "word",
                        "name": "teamdoc-filesystem",
                    },
                    {
                        "words": ["teamviewer", "this site is running"],
                        "type": "word",
                        "name": "teamviewer",
                        "condition": "and",
                    },
                    {
                        "words": ["<a href='http://www.teamviewer.com'>teamviewer</a>"],
                        "type": "word",
                        "name": "teamviewer",
                    },
                    {
                        "words": ["this site is running"],
                        "type": "word",
                        "name": "teamviewer",
                    },
                    {
                        "words": [
                            'href="http://www.techbridge-inc.com/" target="_blank"></a></span>'
                        ],
                        "type": "word",
                        "name": "techbridge-cloud-conference",
                    },
                    {
                        "words": ["/images/web_meeting/tool_bg.jpg"],
                        "type": "word",
                        "name": "techbridge-cloud-conference",
                    },
                    {
                        "words": ["/joinmeeting.js"],
                        "type": "word",
                        "name": "techbridge-cloud-conference",
                    },
                    {
                        "words": [
                            '<form id="default_login" name="default_login" method="post" action="/goform/user_login">'
                        ],
                        "type": "word",
                        "name": "telenor-4g-router",
                    },
                    {
                        "words": [
                            "please power off and plug in (u)sim card. then power on again. or pin is permanently "
                            "blocked, please contact the provider"
                        ],
                        "type": "word",
                        "name": "telenor-4g-router",
                    },
                    {
                        "words": ["https://clients.telrads.com/css/feedback.css"],
                        "type": "word",
                        "name": "teleradiology-telrads",
                    },
                    {
                        "words": ["telerik.web.ui.webresource.axd"],
                        "type": "word",
                        "name": "telerik-sitefinity",
                    },
                    {
                        "words": ['content="sitefinity'],
                        "type": "word",
                        "name": "telerik-sitefinity",
                    },
                    {
                        "words": ["527meeting"],
                        "type": "word",
                        "name": "telinxinxi-527meeting",
                    },
                    {
                        "words": [' content="轻会议"'],
                        "type": "word",
                        "name": "telinxinxi-527meeting",
                    },
                    {
                        "words": ["/cgi-bin/getinvestigate?flowid="],
                        "type": "word",
                        "name": "tencent-exmail",
                    },
                    {
                        "words": ['content="登录腾讯企业邮箱'],
                        "type": "word",
                        "name": "tencent-exmail",
                    },
                    {
                        "words": ['src="/images//foxs_logo.gif'],
                        "type": "word",
                        "name": "tencent-foxmail-server",
                    },
                    {
                        "words": ["/_common/scripts/global.js"],
                        "type": "word",
                        "name": "tengweioa",
                    },
                    {"words": ["/valcode.aspx"], "type": "word", "name": "tengweioa"},
                    {
                        "words": ["images/teradata_aster_logo.png"],
                        "type": "word",
                        "name": "teradata-aster",
                    },
                    {
                        "words": [
                            "redirecting to aster ncluster management console (amc)"
                        ],
                        "type": "word",
                        "name": "teradata-ncluster",
                    },
                    {
                        "words": ["var currentmode ='fittowindowonload"],
                        "type": "word",
                        "name": "teradata-parallel",
                    },
                    {
                        "words": ["pcoip&#174 zero client"],
                        "type": "word",
                        "name": "teradici-pcoip-zero-client",
                    },
                    {
                        "words": ["password_value"],
                        "type": "word",
                        "name": "teradici-pcoip-zero-client",
                    },
                    {
                        "words": ["/home/pkibinduser?subjectname="],
                        "type": "word",
                        "name": "terminal-feature-collection-and-control-system",
                    },
                    {
                        "words": [
                            '<a href="http://thehostingtool.com" target="_blank">thehostingtool</a>'
                        ],
                        "type": "word",
                        "name": "thehostingtool",
                    },
                    {
                        "words": ['<td width="20%"><strong>server os:</strong></td>'],
                        "type": "word",
                        "name": "thehostingtool",
                    },
                    {
                        "words": ['page=status&sub=phpinfo">phpinfo</a>)</td>'],
                        "type": "word",
                        "name": "thehostingtool",
                    },
                    {
                        "words": ["ThinkAdmin</title>"],
                        "type": "word",
                        "name": "thinkadmin",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: ThinkCMF"],
                        "type": "word",
                        "name": "thinkcmf",
                    },
                    {
                        "words": ['content="thinkcmf'],
                        "type": "word",
                        "name": "thinkcmf",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: thinkcmf"],
                        "type": "word",
                        "name": "thinkcmf",
                    },
                    {
                        "words": [
                            'made by <a href="http://www.thinkcmf.com" target="_blank">thinkcmf</a>'
                        ],
                        "type": "word",
                        "name": "thinkcmf",
                    },
                    {
                        "words": [
                            '<a title="官方网站" href="http://www.thinkcmf.com">ThinkCMF</a>'
                        ],
                        "type": "word",
                        "name": "thinkcmf",
                    },
                    {
                        "words": ["智能网关系统<br>"],
                        "type": "word",
                        "name": "thinker-intelligentgateway",
                    },
                    {
                        "words": [
                            "href='http://www.thinkcloud.cn' target='_blank'>thinkcloud</a>.all right reserved</div>"
                        ],
                        "type": "word",
                        "name": "thinkmail",
                    },
                    {
                        "words": ['app_download":"","thinkmail官网'],
                        "type": "word",
                        "name": "thinkmail",
                    },
                    {
                        "words": ['/webmail"+"/common/validatecode.do'],
                        "type": "word",
                        "name": "thinkmail",
                    },
                    {
                        "words": ["/resource/se/common/jquery.js"],
                        "type": "word",
                        "name": "thinkmail",
                    },
                    {
                        "words": ["powered by thinkox"],
                        "type": "word",
                        "name": "thinkox",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: ThinkPHP"],
                        "type": "word",
                        "name": "thinkphp",
                    },
                    {
                        "words": ['href="http://www.thinkphp.cn">thinkphp</a><sup>'],
                        "type": "word",
                        "name": "thinkphp",
                    },
                    {
                        "words": ["/public/others/maxlength.js", "yfcmf"],
                        "type": "word",
                        "name": "thinkphp-yfcmf",
                        "condition": "and",
                    },
                    {
                        "words": ["/yfcmf/yfcmf.js"],
                        "type": "word",
                        "name": "thinkphp-yfcmf",
                    },
                    {
                        "words": [
                            'href="https://www.thinksaas.cn/app/home/skins/default/style.css"'
                        ],
                        "type": "word",
                        "name": "thinksaas",
                    },
                    {
                        "words": ["_static/image/favicon.ico"],
                        "type": "word",
                        "name": "thinksns",
                    },
                    {
                        "words": ['powered by <a href="http://www.thinksns.com">'],
                        "type": "word",
                        "name": "thinksns",
                    },
                    {
                        "words": [
                            "<html><head><title>error</title></head><body>your request produced an error.</body></html>"
                        ],
                        "type": "word",
                        "name": "thoughtconduit",
                    },
                    {
                        "words": ['content="threatconnect" />'],
                        "type": "word",
                        "name": "threatconnect-platform",
                    },
                    {
                        "words": [
                            'value="sinopec.ad" readonly="readonly" id="ddldomains2"'
                        ],
                        "type": "word",
                        "name": "tianyang-bpm-system",
                    },
                    {
                        "words": [
                            '<link rel="stylesheet" href="/include/firedigit.css">'
                        ],
                        "type": "word",
                        "name": "tiger-ip-connect",
                    },
                    {
                        "words": ['<link rel="stylesheet" href="/include/tms.css">'],
                        "type": "word",
                        "name": "tiger-ip-connect",
                    },
                    {
                        "words": ["/include/tiger.css"],
                        "type": "word",
                        "name": "tiger-ip-connect",
                    },
                    {
                        "words": [
                            '<a href="http://www.tightvnc.com/">www.tightvnc.com'
                        ],
                        "type": "word",
                        "name": "tightvnc",
                    },
                    {
                        "words": ["js/thinvnc.sdk.js"],
                        "type": "word",
                        "name": "tightvnc",
                    },
                    {
                        "words": ["js/thinvnc.app.js"],
                        "type": "word",
                        "name": "tightvnc",
                    },
                    {
                        "words": ["jquerytiki = new object"],
                        "type": "word",
                        "name": "tiki-wiki-cms",
                    },
                    {
                        "words": ["link international corp. all rights reserved"],
                        "type": "word",
                        "name": "timelink",
                    },
                    {
                        "words": [
                            '<link rel="shortcut icon" type="image/png" href="/timelink/images/favicon.ico"/>'
                        ],
                        "type": "word",
                        "name": "timelink",
                    },
                    {
                        "words": ["北京数字认证股份有限公司"],
                        "type": "word",
                        "name": "timestamp-server",
                    },
                    {
                        "words": ['id="hwebsystemtitle"'],
                        "type": "word",
                        "name": "tingjiandan-iot",
                    },
                    {
                        "part": "header",
                        "words": ["Server: tinyproxy/"],
                        "type": "word",
                        "name": "tinyproxy",
                    },
                    {
                        "words": ["var server_url = '/__con__/__act__';"],
                        "type": "word",
                        "name": "tinyrise-tinyshop",
                    },
                    {
                        "words": ["tiny_token_"],
                        "type": "word",
                        "name": "tinyrise-tinyshop",
                    },
                    {"words": ['content="tipask'], "type": "word", "name": "tipask"},
                    {
                        "words": ["dlqaweb_deploy/webresource.axd"],
                        "type": "word",
                        "name": "tisson-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: titan ftp server"],
                        "type": "word",
                        "name": "titan-ftp",
                    },
                    {
                        "words": ['href="/tmailer/img/logo/favicon.ico'],
                        "type": "word",
                        "name": "tmailer",
                    },
                    {
                        "words": ["todaymail anti-spam police", "todaynic.com,inc."],
                        "type": "word",
                        "name": "todaymail",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'content="tomatocart open source shopping cart solutions" />'
                        ],
                        "type": "word",
                        "name": "tomatocart",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.tomatocart.com" target="_blank">tomatocart</a>'
                        ],
                        "type": "word",
                        "name": "tomatocart",
                    },
                    {
                        "words": ["tomato.core.widget.loader.baseurl = 'http://"],
                        "type": "word",
                        "name": "tomatocms",
                    },
                    {
                        "words": [
                            '<a href="http://www.tomatocms.com" title="powered by tomatocms" target="_blank">'
                        ],
                        "type": "word",
                        "name": "tomatocms",
                    },
                    {
                        "words": [
                            "tomcat monitor uses wadl to describe services it can offer"
                        ],
                        "type": "word",
                        "name": "tomcat-monitor-uses-wadl",
                    },
                    {
                        "words": ['href="/static/images/tongda.ico"'],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": [
                            "<a href='http://www.tongda2000.com/' target='_black'>通达官网</a>"
                        ],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": ["/images/tongda.ico"],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {"words": ["Office Anywhere"], "type": "word", "name": "tongda-oa"},
                    {
                        "words": ["login", "tongda2000"],
                        "type": "word",
                        "name": "tongda-oa",
                        "condition": "and",
                    },
                    {
                        "words": ["/static/templates/2013_01/index.css/"],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": ["javascript:document.form1.uname.focus()"],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": [
                            '<link rel="shortcut icon" href="/images/tongda.ico" />'
                        ],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": ["oa提示：不能登录oa"],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": ["紧急通知：今日10点停电"],
                        "type": "word",
                        "name": "tongda-oa",
                    },
                    {
                        "words": ['src="/images/sinopeclogo.png"'],
                        "type": "word",
                        "name": "topbpm-user-authorization-center",
                    },
                    {
                        "words": ['background="images/logbg.jpg"'],
                        "type": "word",
                        "name": "topfreeweb-charging",
                    },
                    {
                        "words": ['value="经理">经理</option>'],
                        "type": "word",
                        "name": "topfreeweb-charging",
                    },
                    {
                        "words": ["mailto:kenstar@kenstar-group.com"],
                        "type": "word",
                        "name": "topper-nms",
                    },
                    {
                        "words": [
                            '<img src="static/images/login/loading.gif" /><span id="message">loading......</span>'
                        ],
                        "type": "word",
                        "name": "topsec-dlp",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=/usercertloginaction.action" />'
                        ],
                        "type": "word",
                        "name": "topwalk-mtp",
                    },
                    {
                        "part": "header",
                        "words": ["Server: tornadoserver"],
                        "type": "word",
                        "name": "tornadoserver",
                    },
                    {
                        "words": ['<param name="environment" value='],
                        "type": "word",
                        "name": "totvs-smartclient",
                    },
                    {
                        "words": ['content="width=1300,initial-scale=1,minimal-ui"'],
                        "type": "word",
                        "name": "tp_link-omada-controller",
                    },
                    {
                        "words": ["tpshop.css", "tpshop_config"],
                        "type": "word",
                        "name": "tpshop",
                        "condition": "and",
                    },
                    {
                        "words": ["tq.cn/floatcard?"],
                        "type": "word",
                        "name": "tq-cloud-call-center",
                    },
                    {
                        "words": ["<h1>available projects</h1>"],
                        "type": "word",
                        "name": "trac",
                    },
                    {"words": ["wiki/tracguide"], "type": "word", "name": "trac"},
                    {"words": ["powered by trac"], "type": "word", "name": "trac"},
                    {"words": ["emlsoft"], "type": "word", "name": "traderencrm"},
                    {
                        "words": ['content="dpivision.com ltd" />'],
                        "type": "word",
                        "name": "tradingeye",
                    },
                    {
                        "words": [
                            'id="credits"><a href="http://www.tradingeye.com/">powered by tradingeye</a>'
                        ],
                        "type": "word",
                        "name": "tradingeye",
                    },
                    {
                        "words": ["lastrow_right"],
                        "type": "word",
                        "name": "trend-smart-protection-server",
                    },
                    {
                        "words": ["redirect_reason"],
                        "type": "word",
                        "name": "trend-smart-protection-server",
                    },
                    {
                        "words": ['href="#">trs hybase</a>'],
                        "type": "word",
                        "name": "trs-hybase",
                    },
                    {"words": ["/wcm/app/js"], "type": "word", "name": "trs-wcm"},
                    {"words": ["0;url=/wcm"], "type": "word", "name": "trs-wcm"},
                    {
                        "words": ['window.location.href = "/wcm";'],
                        "type": "word",
                        "name": "trs-wcm",
                    },
                    {"words": ["forum.trs.com.cn"], "type": "word", "name": "trs-wcm"},
                    {
                        "words": ['/wcm" target="_blank">网站管理', "wcm"],
                        "type": "word",
                        "name": "trs-wcm",
                        "condition": "and",
                    },
                    {
                        "words": ['/wcm" target="_blank">管理'],
                        "type": "word",
                        "name": "trs-wcm",
                    },
                    {
                        "words": ['href="http://www.trunkey.com/"'],
                        "type": "word",
                        "name": "trunkey-icpsystem",
                    },
                    {
                        "words": [
                            'var url = getcontextname() + "?service=ajaxdirect/1/'
                        ],
                        "type": "word",
                        "name": "tsm",
                    },
                    {
                        "words": ["编码申报系统"],
                        "type": "word",
                        "name": "tuling-code-filing-system",
                    },
                    {
                        "words": ["长沙图灵科技有限公司"],
                        "type": "word",
                        "name": "tuling-procurement-strategy-management-system",
                    },
                    {
                        "words": ['src="/login/main_body_bg.jpg"'],
                        "type": "word",
                        "name": "tuling-procurement-strategy-management-system",
                    },
                    {
                        "words": [
                            'href="/content/login-ui/static/h-ui.admin/css/h-ui.login.css"'
                        ],
                        "type": "word",
                        "name": "tulingtech-system",
                    },
                    {
                        "words": ['<meta name="tumblr-theme" content='],
                        "type": "word",
                        "name": "tumblr",
                    },
                    {
                        "words": [
                            '<!-- begin tumblr code --><iframe src="http://assets.tumblr.com/iframe.html'
                        ],
                        "type": "word",
                        "name": "tumblr",
                    },
                    {
                        "words": [
                            "var myspecs = "
                            '"\'menubar=0,status=1,resizable=1,location=0,titlebar=1,toolbar=1,scrollbars=1,width=" + '
                            'mywidth + ",height=" + myheight +'
                        ],
                        "type": "word",
                        "name": "turbo-seek",
                    },
                    {
                        "words": ["TurboMail管理系统"],
                        "type": "word",
                        "name": "turbomail",
                    },
                    {
                        "words": ["powered by turbomail", "wzcon1 clearfix"],
                        "type": "word",
                        "name": "turbomail",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<a href="http://www.turbomail.org">powered by turbomail</a>'
                        ],
                        "type": "word",
                        "name": "turbomail",
                    },
                    {
                        "words": ['alt="turbomail 电子邮件系统"/>'],
                        "type": "word",
                        "name": "turbomail",
                    },
                    {
                        "words": ['font><a href="http://www.go-redrock.com"><font'],
                        "type": "word",
                        "name": "tutortrac",
                    },
                    {
                        "words": ['href="/twcms/theme/default/css/global.css"'],
                        "type": "word",
                        "name": "twcms",
                    },
                    {
                        "words": [
                            '<meta name="description" content="twonkymedia digital home">'
                        ],
                        "type": "word",
                        "name": "twonkyserver",
                    },
                    {
                        "words": ["typecho", "usr/themes"],
                        "type": "word",
                        "name": "typecho",
                        "condition": "and",
                    },
                    {
                        "words": ['<body link="white" vlink="white" alink="white">'],
                        "type": "word",
                        "name": "u-mail",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=./webmail/">'
                        ],
                        "type": "word",
                        "name": "u-mail",
                    },
                    {
                        "words": [
                            'power by <a href="http://www.comingchina.com">u-mail邮件服务器</a>'
                        ],
                        "type": "word",
                        "name": "u-mail",
                    },
                    {
                        "words": ["u-mail webadmin 要求启用 javascript"],
                        "type": "word",
                        "name": "u-mail",
                    },
                    {
                        "words": ['content="ureader'],
                        "type": "word",
                        "name": "u-reader-digital-library",
                    },
                    {
                        "words": ['class="login-show-title">dynomedia inc.</p>'],
                        "type": "word",
                        "name": "u-reader-digital-library",
                    },
                    {
                        "words": ['content="unms"'],
                        "type": "word",
                        "name": "ubiquiti-unms",
                    },
                    {
                        "words": ["welcome to nginx on ubuntu!"],
                        "type": "word",
                        "name": "ubuntu",
                    },
                    {
                        "words": [
                            "<title>Test Page for the Apache HTTP Server on openEuler Linux</title>",
                            "<title>Test Page for the Nginx HTTP Server on openEuler</title>",
                        ],
                        "type": "word",
                        "name": "openeuler",
                        "condition": "or",
                    },
                    {
                        "words": ['src="http://so.kaipuyun.cn?sitecode='],
                        "type": "word",
                        "name": "ucap-search-",
                    },
                    {
                        "words": [
                            'method="post" action="s" onsubmit="return checksearchform();">'
                        ],
                        "type": "word",
                        "name": "ucap-search-",
                    },
                    {
                        "words": ['hidden-xs">物联网云平台'],
                        "type": "word",
                        "name": "uccc-iot",
                    },
                    {
                        "words": ["assets-cli.udesk.cn/im_client/js/udeskapi.js"],
                        "type": "word",
                        "name": "udesk",
                    },
                    {
                        "words": ["udesk.cn/im_client/?web_plugin_id="],
                        "type": "word",
                        "name": "udesk",
                    },
                    {
                        "words": [
                            '<script type="text/javascript" src="themes/default/js/webmail.js"></script>',
                            "uebimiau",
                        ],
                        "type": "word",
                        "name": "uebimiau-webmail",
                        "condition": "and",
                    },
                    {"words": ["ueditor.all.js"], "type": "word", "name": "ueditor"},
                    {
                        "words": ['src="sdkjs/uftttsdk2.js"'],
                        "type": "word",
                        "name": "ufttt-iot",
                    },
                    {"words": ["uin.plist"], "type": "word", "name": "uin-meeting"},
                    {
                        "words": ['<meta name="generator" content="ubb.threads'],
                        "type": "word",
                        "name": "ultimate-bulletin-board",
                    },
                    {
                        "words": [
                            '<a href="http://www.groupee.com/landing/goto.php?a=ubb.classic">powered by '
                            "ubb.classic&trade"
                        ],
                        "type": "word",
                        "name": "ultimate-bulletin-board",
                    },
                    {
                        "words": ["url=iportal/login.aspx"],
                        "type": "word",
                        "name": "ultimus-bpm",
                    },
                    {
                        "words": ["/preauth/login.cgi"],
                        "type": "word",
                        "name": "ultra_electronics",
                    },
                    {
                        "words": ["/preauth/style.css"],
                        "type": "word",
                        "name": "ultra_electronics",
                    },
                    {
                        "words": [
                            "&nbsp;tonethink.soft&nbsp;&nbsp;all rights reserved"
                        ],
                        "type": "word",
                        "name": "ultracrm",
                    },
                    {
                        "words": ["<li><p>欢迎进入身份与安全管控系统</p></li>"],
                        "type": "word",
                        "name": "ultrapower-identity",
                    },
                    {
                        "words": [
                            '<img src="./images/main/ultrastatslogo.png" width="300" height="200" name="ultrastats_logo" '
                            'align="center">'
                        ],
                        "type": "word",
                        "name": "ultrastats",
                    },
                    {
                        "words": ['<a href="http://www.uniformserver.com">'],
                        "type": "word",
                        "name": "uniform-server",
                    },
                    {
                        "words": [
                            '<meta name="description" content="the uniform server 8.1.0-coral." />'
                        ],
                        "type": "word",
                        "name": "uniform-server",
                    },
                    {
                        "words": [
                            '<div id="divider">developed by <a href="http://www.uniformserver.com/">the uniform server '
                            "development team</a></div>"
                        ],
                        "type": "word",
                        "name": "uniform-server",
                    },
                    {
                        "words": ["txtpasswordcssclass"],
                        "type": "word",
                        "name": "unimas-cameraaudit",
                    },
                    {
                        "words": ['src="images/pag-logo.png"'],
                        "type": "word",
                        "name": "uniview-ezclould",
                    },
                    {
                        "words": ["<h1>welcome to ezstation vc server"],
                        "type": "word",
                        "name": "uniview-ezstation",
                    },
                    {"words": ["vm5.0"], "type": "word", "name": "uniview-vm50"},
                    {
                        "words": ['alt="powered by uportal'],
                        "type": "word",
                        "name": "uportal",
                    },
                    {
                        "words": ["b0.upaiyun.com"],
                        "type": "word",
                        "name": "upyun-js-library",
                    },
                    {
                        "words": ["/public/css/common_uqcms.css"],
                        "type": "word",
                        "name": "uqcms",
                    },
                    {
                        "words": ['<input name="j_captcha_response" type="hidden'],
                        "type": "word",
                        "name": "urp-integrated-educational-system",
                    },
                    {
                        "words": ["北京清元优软科技有限公司", "教务系统"],
                        "type": "word",
                        "name": "urp-integrated-educational-system",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '<form id="system-form-registration" enctype="application/x-www-form-urlencoded" '
                            'class="system-form-registration" accept-charset="utf-8'
                        ],
                        "type": "word",
                        "name": "useresponse",
                    },
                    {
                        "words": [
                            'title="customer feedback software, community support system" target="_blank" '
                            'href="http://www.useresponse.com" class="popup-logo">'
                        ],
                        "type": "word",
                        "name": "useresponse",
                    },
                    {"words": ["libs.useso.com"], "type": "word", "name": "useso"},
                    {
                        "words": ['action="/usezan/login/getlogin"'],
                        "type": "word",
                        "name": "usezan-system",
                    },
                    {
                        "words": [
                            'class="utt-inline-block" src="./images/login_btmlogo.png"'
                        ],
                        "type": "word",
                        "name": "utt-device",
                    },
                    {
                        "words": ['<frame src="../conference/currentconfaction.do'],
                        "type": "word",
                        "name": "v2-video-conferencing",
                    },
                    {
                        "words": ['src="content.jsp"'],
                        "type": "word",
                        "name": "v2-video-conferencing",
                    },
                    {
                        "words": ['href="/viewcmscac.do', 'href="viewcmscac.do'],
                        "type": "word",
                        "name": "valleycms",
                        "condition": "and",
                    },
                    {
                        "words": ["log into my eamic® account"],
                        "type": "word",
                        "name": "valueapex-eamic",
                    },
                    {
                        "words": ['id="mymodallabel">login vam system'],
                        "type": "word",
                        "name": "vam-product",
                    },
                    {
                        "words": ["powered by virtual airlines manager"],
                        "type": "word",
                        "name": "vam-product",
                    },
                    {
                        "words": ['src="js/vam.js"'],
                        "type": "word",
                        "name": "vam-product",
                    },
                    {
                        "words": [
                            'href="https://virtualairlinesmanager.net/">virtual airlines manager'
                        ],
                        "type": "word",
                        "name": "vam-product",
                    },
                    {
                        "words": [
                            'stylesheets/load/vamcart.css" rel="stylesheet"  media="screen'
                        ],
                        "type": "word",
                        "name": "vamcart",
                    },
                    {
                        "words": ["<!-- powered by: vamcart (http://vamcart.com) -->"],
                        "type": "word",
                        "name": "vamcart",
                    },
                    {
                        "words": [
                            '<p><a href="http://vamcart.com/">php shopping cart</a> <a '
                            'href="http://vamcart.com/">vamcart</a></p>'
                        ],
                        "type": "word",
                        "name": "vamcart",
                    },
                    {
                        "words": ["function loadmunchkinkey()"],
                        "type": "word",
                        "name": "varmour-product",
                    },
                    {
                        "words": ['content="vbulletin'],
                        "type": "word",
                        "name": "vbulletin",
                    },
                    {
                        "words": ["powered by vbulletin&trade;"],
                        "type": "word",
                        "name": "vbulletin",
                    },
                    {
                        "words": [
                            '<!-- do not remove this copyright notice -->powered by < a href="https://www.vbulletin.com" '
                            'id="vbulletinlink">'
                        ],
                        "type": "word",
                        "name": "vbulletin",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.vcalendar.org">vcalendar</a>'
                        ],
                        "type": "word",
                        "name": "vcalendar",
                    },
                    {
                        "words": ['<link href="styles/basic/style.css"'],
                        "type": "word",
                        "name": "vcalendar",
                    },
                    {
                        "part": "header",
                        "words": ["Location: /sra-purpletools-webui/app"],
                        "type": "word",
                        "name": "vectr",
                    },
                    {
                        "words": ["vectra.base.css"],
                        "type": "word",
                        "name": "vectra-product",
                    },
                    {
                        "words": ['gps-web"></iframe>'],
                        "type": "word",
                        "name": "vehiclemonitoringcloudplatform",
                    },
                    {
                        "words": [
                            'href="/opscenter/features/common/images/favicon.ico"'
                        ],
                        "type": "word",
                        "name": "veritas-netbackup",
                    },
                    {
                        "words": ['var port = "9528'],
                        "type": "word",
                        "name": "vertiv-system",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: vertx-web.session"],
                        "type": "word",
                        "name": "vertx",
                    },
                    {
                        "words": ["/vhplot/webresource.axd"],
                        "type": "word",
                        "name": "vhsoft-vhplot",
                    },
                    {
                        "words": ["url=/vicidial/welcome.php"],
                        "type": "word",
                        "name": "vicidial",
                    },
                    {
                        "words": [
                            'value="style2012/style1/scripts/expressinstall.swf"'
                        ],
                        "type": "word",
                        "name": "victorysoft",
                    },
                    {
                        "words": ['href="webstyles/webstyle1/style1/css.css"'],
                        "type": "word",
                        "name": "victorysoft",
                    },
                    {
                        "words": ['class="row fl-controls-left'],
                        "type": "word",
                        "name": "victorysoft-performance-management-system",
                    },
                    {
                        "words": ["casui/themes/siam/login.css"],
                        "type": "word",
                        "name": "victorysoft-performance-management-system",
                    },
                    {
                        "words": ["power by linksoon - videosoon"],
                        "type": "word",
                        "name": "videosoon",
                    },
                    {
                        "words": ['href="skin/anysoondefault/anystyles.css'],
                        "type": "word",
                        "name": "videosoon",
                    },
                    {
                        "words": [" <span>平台采用最新图像化展现技术"],
                        "type": "word",
                        "name": "videosurveillancemanagementplatform",
                    },
                    {
                        "words": ["fgetquery"],
                        "type": "word",
                        "name": "viewgood-streammedia",
                    },
                    {
                        "words": ["viewgood"],
                        "type": "word",
                        "name": "viewgood-streammedia",
                    },
                    {
                        "words": [
                            "location.href",
                            "var webvirtualdiretory = 'viewgood';",
                        ],
                        "type": "word",
                        "name": "viewgood-streammedia",
                        "condition": "and",
                    },
                    {
                        "words": ["src='/viewgood/pc/"],
                        "type": "word",
                        "name": "viewgood-streammedia",
                    },
                    {
                        "words": ['<body onload="forward_to_logon()">'],
                        "type": "word",
                        "name": "violation-outreach-monitoring-system",
                    },
                    {
                        "words": ["window.location='login.action';"],
                        "type": "word",
                        "name": "violation-outreach-monitoring-system",
                    },
                    {
                        "words": ["欢迎登录违规外联平台"],
                        "type": "word",
                        "name": "violation-outreach-monitoring-system",
                    },
                    {
                        "words": [
                            "<center><a href=/virtualmin-password-recovery/>forgot your virtualmin "
                            "password?</a></center>"
                        ],
                        "type": "word",
                        "name": "virtualmin",
                    },
                    {
                        "words": ["<!-- begin myconnection server applet -->"],
                        "type": "word",
                        "name": "visualware-myconnection-server",
                    },
                    {
                        "words": ["function toggle(targetid)"],
                        "type": "word",
                        "name": "vmedia-multimedia-publishing-platform",
                    },
                    {
                        "words": ['class="video_00"'],
                        "type": "word",
                        "name": "vmedia-multimedia-publishing-platform",
                    },
                    {
                        "words": ['content="vmware esxi'],
                        "type": "word",
                        "name": "vmware-esx",
                    },
                    {
                        "words": [
                            'document.write("<title>" + id_eesx_welcome + "</title>");'
                        ],
                        "type": "word",
                        "name": "vmware-esx",
                    },
                    {
                        "words": [
                            '<meta http-equiv="refresh" content="0;url=\'/ui\'"/>'
                        ],
                        "type": "word",
                        "name": "vmware-esx",
                    },
                    {
                        "words": ['content="vmware esx '],
                        "type": "word",
                        "name": "vmware-esx",
                    },
                    {
                        "words": ["document.write(id_esx_viclientdesc);"],
                        "type": "word",
                        "name": "vmware-esx",
                    },
                    {
                        "words": ['ng-app="esxuiapp"'],
                        "type": "word",
                        "name": "vmware-esxi",
                    },
                    {
                        "words": ['<title ng-bind="$root.title">'],
                        "type": "word",
                        "name": "vmware-esxi",
                    },
                    {
                        "words": ["href='https://www.vmware.com/go/viewclients'"],
                        "type": "word",
                        "name": "vmware-horizon",
                    },
                    {
                        "words": ['alt="vmware horizon">'],
                        "type": "word",
                        "name": "vmware-horizon",
                    },
                    {
                        "words": ['content="vmware server is virtual'],
                        "type": "word",
                        "name": "vmware-server-2",
                    },
                    {
                        "words": ["/converter/vmware-converter-client.exe"],
                        "type": "word",
                        "name": "vmware-vcenter",
                    },
                    {
                        "words": ['content="vmware vcenter'],
                        "type": "word",
                        "name": "vmware-vcenter",
                    },
                    {
                        "words": ["/vmw_nsx_logo-black-triangle-500w.png"],
                        "type": "word",
                        "name": "vmware-vcenter",
                    },
                    {
                        "words": ['content="vmware virtualcenter'],
                        "type": "word",
                        "name": "vmware-virtualcenter",
                    },
                    {
                        "words": ['content="vmware vsphere'],
                        "type": "word",
                        "name": "vmware-virtualcenter",
                    },
                    {
                        "words": ["url=vcops-vsphere/"],
                        "type": "word",
                        "name": "vmware-virtualcenter",
                    },
                    {
                        "words": ["the vshield manager requires"],
                        "type": "word",
                        "name": "vmware-virtualcenter",
                    },
                    {
                        "words": ["正在重定向到 vrealize operations manager web"],
                        "type": "word",
                        "name": "vmware-vrealize",
                    },
                    {
                        "words": ["Identity Manager", "VMware"],
                        "type": "word",
                        "name": "vmware-vrealize-operations-manager",
                        "condition": "and",
                    },
                    {
                        "words": ['<meta name="description" content="VMware vSphere'],
                        "type": "word",
                        "name": "vmware-vsphere",
                    },
                    {
                        "words": ["<title>VMware View Portal</title>"],
                        "type": "word",
                        "name": "vmwareview",
                    },
                    {
                        "words": ["<applet code=vncviewer.class archive=vncviewer.jar"],
                        "type": "word",
                        "name": "vnc",
                    },
                    {
                        "part": "header",
                        "words": ["Server: vnc server enterprise edition"],
                        "type": "word",
                        "name": "vnc-enterprise-",
                    },
                    {"words": ["lgdynacodebtn"], "type": "word", "name": "vop"},
                    {
                        "words": ['id="lgform" action="/sso/login', "vop"],
                        "type": "word",
                        "name": "vop",
                        "condition": "and",
                    },
                    {
                        "words": ['content="vos2009, voip, voip运营支撑系统, 软交换"'],
                        "type": "word",
                        "name": "vos-vos2009",
                    },
                    {
                        "words": ["images/vos3000.ico"],
                        "type": "word",
                        "name": "vos3000",
                    },
                    {
                        "words": ['content="微平台投票系统'],
                        "type": "word",
                        "name": "votemanager",
                    },
                    {
                        "words": [
                            '<a href="http://www.cdrbp.cn">微信数字投票',
                            'content="微平台投票管理系统',
                        ],
                        "type": "word",
                        "name": "votemanager",
                        "condition": "and",
                    },
                    {
                        "words": ['<a href="http://www.vpasp.com">'],
                        "type": "word",
                        "name": "vp-asp",
                    },
                    {"words": ['src="vs350.js'], "type": "word", "name": "vp-asp"},
                    {
                        "words": ["shopdisplayproducts.asp?id="],
                        "type": "word",
                        "name": "vp-asp",
                    },
                    {
                        "words": ['class="form-actions j_add_ip_actions"'],
                        "type": "word",
                        "name": "vpn358system",
                    },
                    {
                        "words": ['href="/lib/bootstrap/ico/favicon.ico"'],
                        "type": "word",
                        "name": "vpn358system",
                    },
                    {
                        "words": [
                            '<span id="lblvalidcompany" class="validcompany">vrv'
                        ],
                        "type": "word",
                        "name": "vrv-desktop-application-system",
                    },
                    {
                        "words": ["var vver = $('#hidverify').val();"],
                        "type": "word",
                        "name": "vrv-desktop-application-system",
                    },
                    {
                        "words": ["<h3>连豆豆pc客户端 </h3>"],
                        "type": "word",
                        "name": "vrv-im",
                    },
                    {
                        "words": [
                            'href="http://im.vrv.cn/server-securitycenter/password/goretrieval.vrv'
                        ],
                        "type": "word",
                        "name": "vrv-im",
                    },
                    {
                        "words": [
                            'class="loginusername" value="" placeholder="连豆豆账号/邮箱/手机号'
                        ],
                        "type": "word",
                        "name": "vrv-im",
                    },
                    {
                        "words": ['class="wj-text wj-title">下载信源豆豆</p>'],
                        "type": "word",
                        "name": "vrv-im",
                    },
                    {"words": ['id="modal_delay"'], "type": "word", "name": "vrv-nac"},
                    {
                        "words": [
                            "localstorage.setitem('doctitle','北信源网络接入控制系统')",
                            "欢迎登录北信源网络接入控制系统",
                        ],
                        "type": "word",
                        "name": "vrv-nac",
                        "condition": "and",
                    },
                    {"words": ["errmag"], "type": "word", "name": "vts-cms"},
                    {
                        "words": [
                            'class="copyright">powered by <a href="http://www.we7.cc"><b>微擎</b>'
                        ],
                        "type": "word",
                        "name": "w7-officialaccounts",
                    },
                    {
                        "words": [
                            'content="微擎,微信',
                            'onsubmit="return formcheck();" class="we7-form">',
                        ],
                        "type": "word",
                        "name": "w7-officialaccounts",
                        "condition": "and",
                    },
                    {
                        "words": ["powered by we7.cc"],
                        "type": "word",
                        "name": "w7-officialaccounts",
                    },
                    {
                        "words": [
                            '<a href="http://www.ninechime.com/products/" title="get your own free bbs!">wacintaki'
                        ],
                        "type": "word",
                        "name": "wacintaki-poteto-bbs",
                    },
                    {
                        "words": [
                            'by <a href="http://suteki.nu">ranmaguy</a> and <a '
                            'href="http://www.cellosoft.com">marcello</a>'
                        ],
                        "type": "word",
                        "name": "wacintaki-poteto-bbs",
                    },
                    {
                        "words": ["<h2>welcome to wackopicko</h2>"],
                        "type": "word",
                        "name": "wackopicko",
                    },
                    {
                        "words": ['<h1 id="title"><a href="/">wackopicko.com</a></h1>'],
                        "type": "word",
                        "name": "wackopicko",
                    },
                    {
                        "words": ["/javascript/js/witfunctions.js"],
                        "type": "word",
                        "name": "wantit-erp",
                    },
                    {
                        "words": ["window.location = 'wap.htm'"],
                        "type": "word",
                        "name": "wap",
                    },
                    {
                        "words": ["pending waspd activities</font>"],
                        "type": "word",
                        "name": "waspd",
                    },
                    {
                        "words": ["生产经营计划统计一体化管理信息系统安装程序"],
                        "type": "word",
                        "name": "wat-system",
                    },
                    {
                        "words": ["welcome.cgi?p=logo&signinid=url_default"],
                        "type": "word",
                        "name": "waterssslvpn",
                    },
                    {
                        "words": ["application/views/img/logo_wavetop.png"],
                        "type": "word",
                        "name": "wavetop-days",
                    },
                    {
                        "words": ['href="http://www.wdlinux.cn/bbs/index.php'],
                        "type": "word",
                        "name": "wdlinux-wdcpsystem",
                    },
                    {
                        "words": ["linux云主机"],
                        "type": "word",
                        "name": "wdlinux-wdcpsystem",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: wdcpsessionid"],
                        "type": "word",
                        "name": "wdlinux-wdcpsystem",
                    },
                    {
                        "words": ["<title>微擎", "w7.cc"],
                        "type": "word",
                        "name": "we7",
                        "condition": "and",
                    },
                    {
                        "words": ['<a href="http://nazarkin.name/projects/weatimages'],
                        "type": "word",
                        "name": "weatimages",
                    },
                    {
                        "words": ['<meta name="generator" content="weatimages"/>'],
                        "type": "word",
                        "name": "weatimages",
                    },
                    {
                        "words": [
                            '<div align="center" class="weatimages_toppest_navig" style="text-decoration:underline;">'
                        ],
                        "type": "word",
                        "name": "weatimages",
                    },
                    {
                        "words": ['<td><img src="/images/wcpe.gif'],
                        "type": "word",
                        "name": "web-control-panel",
                    },
                    {
                        "part": "header",
                        "words": ["Server: web crossing"],
                        "type": "word",
                        "name": "web-crossing-server",
                    },
                    {
                        "words": [
                            '<form name="webform1" method="post" action="default.aspx" onsubmit="javascript:return '
                            'webform_onsubmit();" id="webform1'
                        ],
                        "type": "word",
                        "name": "web-data-administrator",
                    },
                    {
                        "words": ["window.location='/www/login.html'"],
                        "type": "word",
                        "name": "web-erp-network-system",
                    },
                    {
                        "words": ['<a href="http://www.richtexteditor.org"'],
                        "type": "word",
                        "name": "web-wiz-rich-text-editor",
                    },
                    {
                        "words": [
                            "</head><body>fatal error. you haven't created a config file yet.<br/><a href="
                        ],
                        "type": "word",
                        "name": "web2project",
                    },
                    {
                        "words": ['<a href="http://www.webalizer.org'],
                        "type": "word",
                        "name": "webalizer-log",
                    },
                    {
                        "words": ["<!-- generated by the webalizer  ver"],
                        "type": "word",
                        "name": "webalizer-log",
                    },
                    {
                        "words": ["<!-- webalizer version"],
                        "type": "word",
                        "name": "webalizer-log",
                    },
                    {
                        "words": ['<a href="http://www.shop-script.com'],
                        "type": "word",
                        "name": "webasyst-shop-script",
                    },
                    {
                        "words": [
                            'powered by webasyst shop-script <a href="http://www.shop-script.com/" style="font-weight: '
                            'normal">shopping cart software</a>'
                        ],
                        "type": "word",
                        "name": "webasyst-shop-script",
                    },
                    {
                        "words": ["pear_frontend_web"],
                        "type": "word",
                        "name": "webbased-pear-package-manager",
                    },
                    {
                        "words": [
                            '<img src="?img=pear" width="104" height="50" vspace="2" hspace="5" alt="pear">'
                        ],
                        "type": "word",
                        "name": "webbased-pear-package-manager",
                    },
                    {
                        "words": ['src="webbuilder/script/wb.js'],
                        "type": "word",
                        "name": "webbuilder",
                    },
                    {
                        "words": ['href="/webengine/images/common.css'],
                        "type": "word",
                        "name": "webengine-site",
                    },
                    {
                        "words": ['location.href = "/webengine/web/";'],
                        "type": "word",
                        "name": "webengine-site",
                    },
                    {
                        "words": [
                            '<span id="invocation_sum"></span> different functions called in <span id="runtime_sum'
                        ],
                        "type": "word",
                        "name": "webgrind",
                    },
                    {
                        "words": [
                            'powered by <a href="http://www.webidsupport.com/">webid'
                        ],
                        "type": "word",
                        "name": "webid",
                    },
                    {
                        "words": ['<meta name="generator" content="webid">'],
                        "type": "word",
                        "name": "webid",
                    },
                    {
                        "words": ['<div id="header-right">webissues'],
                        "type": "word",
                        "name": "webissues",
                    },
                    {
                        "words": [
                            '<div><input type="hidden" name="__formid" id="field-login-__formid" value="login" />'
                        ],
                        "type": "word",
                        "name": "webissues",
                    },
                    {"words": ["WebLogic"], "type": "word", "name": "weblogic"},
                    {
                        "words": ["Hypertext Transfer Protocol"],
                        "type": "word",
                        "name": "weblogic",
                    },
                    {
                        "words": ["<i>Hypertext Transfer Protocol -- HTTP/1.1</i>"],
                        "type": "word",
                        "name": "weblogic",
                    },
                    {
                        "words": [
                            "/console/framework/skins/wlsconsole/images/login_WebLogic_branding.png"
                        ],
                        "type": "word",
                        "name": "weblogic",
                    },
                    {
                        "words": ["Welcome to Weblogic Application Server"],
                        "type": "word",
                        "name": "weblogic",
                    },
                    {"words": ["Error 403--"], "type": "word", "name": "weblogic"},
                    {
                        "words": ["Error 404--Not Found"],
                        "type": "word",
                        "name": "weblogic",
                    },
                    {
                        "words": ["Oracle WebLogic Server"],
                        "type": "word",
                        "name": "weblogic",
                    },
                    {
                        "words": ["Webmin", "session_login"],
                        "type": "word",
                        "name": "webmin",
                        "condition": "and",
                    },
                    {"words": ["webmin server on"], "type": "word", "name": "webmin"},
                    {
                        "words": ['<td align="right"><div id="inst_logo"><img src='],
                        "type": "word",
                        "name": "webpa",
                    },
                    {
                        "words": [
                            'class="disclaimer" style="color: #ffffff">《盛邦安全网站监控预警平台服务协议》</a>'
                        ],
                        "type": "word",
                        "name": "webray-situation-awareness",
                    },
                    {
                        "words": ["http://websidestory.com"],
                        "type": "word",
                        "name": "websidestory",
                    },
                    {
                        "words": ["websidestory code"],
                        "type": "word",
                        "name": "websidestory",
                    },
                    {
                        "words": [
                            "websidestory,inc. all rights reserved. u.s.patent no. 6,393,479b1"
                        ],
                        "type": "word",
                        "name": "websidestory",
                    },
                    {
                        "words": ["<!-- websidestory html for search -->"],
                        "type": "word",
                        "name": "websidestory",
                    },
                    {
                        "words": ["WebSVN", "subversion"],
                        "type": "word",
                        "name": "websvn",
                        "condition": "and",
                    },
                    {
                        "words": ["https://cert.webtrust.org/viewseal"],
                        "type": "word",
                        "name": "webtrust-cert",
                    },
                    {
                        "words": [
                            '本系统由<a href="http://www.weiphp.cn" target="_blank">weiphp</a>强力驱动'
                        ],
                        "type": "word",
                        "name": "weiphp",
                    },
                    {"words": ['content="weiphp'], "type": "word", "name": "weiphp"},
                    {"words": ["/css/weiphp.css"], "type": "word", "name": "weiphp"},
                    {
                        "words": ["/utility/corescripts/widget.js"],
                        "type": "word",
                        "name": "weisha-learningsystem",
                    },
                    {
                        "words": ['href="/web/vfyphrmedical">健康档案</a></li>'],
                        "type": "word",
                        "name": "wellcare-health-management-system",
                    },
                    {
                        "words": ["www.wellcare.cn"],
                        "type": "word",
                        "name": "wellcare-health-management-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: weonlydo"],
                        "type": "word",
                        "name": "weonlydo-product",
                    },
                    {
                        "part": "header",
                        "words": ["Server: wstl cpe"],
                        "type": "word",
                        "name": "westell-secure",
                    },
                    {
                        "words": [
                            "<body><center><table border=0><tr align=center><td><font color=red size=5>troy serial "
                            "server</font></td></tr>"
                        ],
                        "type": "word",
                        "name": "whatweb",
                    },
                    {
                        "words": [
                            "network card access password&#058; </b><input type=password size=16 maxlength=16 "
                            "name=access_psw>"
                        ],
                        "type": "word",
                        "name": "whatweb",
                    },
                    {"words": ["武汉富思特"], "type": "word", "name": "whfst-cms"},
                    {"words": ["css/css_whir.css"], "type": "word", "name": "whir"},
                    {
                        "words": ["ezofficeusername"],
                        "type": "word",
                        "name": "whir-ezoffice",
                    },
                    {
                        "words": ["whirrootpath"],
                        "type": "word",
                        "name": "whir-ezoffice",
                    },
                    {
                        "words": ["/defaultroot/js/cookie.js"],
                        "type": "word",
                        "name": "whir-ezoffice",
                    },
                    {
                        "words": ['var flexofficepath="\\/flexoffice"'],
                        "type": "word",
                        "name": "whir-flexoffice",
                    },
                    {
                        "words": ['powered by <a href="http://www.whmcs.com'],
                        "type": "word",
                        "name": "whmcs",
                    },
                    {
                        "words": [
                            '<div id="welcome_box">please <a href="clientarea.php" '
                            'title="login"><strong>login</strong></a> or <a href="register.php" '
                            'title="register"><strong>register</strong></a></div>'
                        ],
                        "type": "word",
                        "name": "whmcs",
                    },
                    {
                        "words": ['href="/content/home/tzjlog.ico"'],
                        "type": "word",
                        "name": "whtzjkj-erp",
                    },
                    {
                        "words": ["wildfly project"],
                        "type": "word",
                        "name": "wildfly-server",
                    },
                    {
                        "words": ["the wasion software foundation"],
                        "type": "word",
                        "name": "willfar-interface-management-tool",
                    },
                    {
                        "words": ['alt="接口应用管理工具"'],
                        "type": "word",
                        "name": "willfar-interface-management-tool",
                    },
                    {
                        "words": ['src="images/sbslogo.gif'],
                        "type": "word",
                        "name": "windows-business-server",
                    },
                    {
                        "words": ['href="/remote">remote web workplace'],
                        "type": "word",
                        "name": "windows-business-server",
                    },
                    {
                        "part": "header",
                        "words": ["Server: windriver-webserver"],
                        "type": "word",
                        "name": "windriver",
                    },
                    {
                        "part": "header",
                        "words": ["Server: wing ftp server"],
                        "type": "word",
                        "name": "wing-ftp-server",
                    },
                    {
                        "words": ["winisp.gif"],
                        "type": "word",
                        "name": "winiis-isp-access-resource-management-system",
                    },
                    {
                        "words": ["amax information technologies inc."],
                        "type": "word",
                        "name": "winmail-server",
                    },
                    {
                        "words": ["pop3,smtp server: <font color=red>"],
                        "type": "word",
                        "name": "winmail-server",
                    },
                    {
                        "words": ['src="themes/default/images/mail_pic.jpg'],
                        "type": "word",
                        "name": "winmail-server",
                    },
                    {
                        "words": ["encryptpwd", "sessid"],
                        "type": "word",
                        "name": "winmail-server",
                        "condition": "and",
                    },
                    {
                        "words": ["f_theme", "pwdplaceholder"],
                        "type": "word",
                        "name": "winmail-server",
                        "condition": "and",
                    },
                    {
                        "words": ["winmail mail server"],
                        "type": "word",
                        "name": "winmail-server",
                    },
                    {
                        "words": ["(build ", 'background="customer/winmail_bg11.jpg'],
                        "type": "word",
                        "name": "winmail-server",
                        "condition": "and",
                    },
                    {
                        "words": ['src="customer/index_winmail_new.gif'],
                        "type": "word",
                        "name": "winmail-server",
                    },
                    {
                        "words": ["winwebmail server"],
                        "type": "word",
                        "name": "winwebmail",
                    },
                    {
                        "words": ["images/owin.css"],
                        "type": "word",
                        "name": "winwebmail",
                    },
                    {
                        "words": [
                            "<td class=newsdiv-mid2>邮局管理员可自行分配邮箱！</td>"
                        ],
                        "type": "word",
                        "name": "winwebmail",
                    },
                    {
                        "words": ['type="hidden" name="secex"'],
                        "type": "word",
                        "name": "winwebmail",
                    },
                    {
                        "words": ['href="images\\hwem.css"'],
                        "type": "word",
                        "name": "winwebmail",
                    },
                    {
                        "words": ['var oemproductname = "mvc_howay6000"'],
                        "type": "word",
                        "name": "wireless-access-point-controller",
                    },
                    {
                        "words": [
                            '<select id = "selclangswitch" class="langswitch" onchange = "switchpagelanguage()">'
                        ],
                        "type": "word",
                        "name": "wireless-access-point-controller",
                    },
                    {
                        "words": ["苏州汉明科技有限公司"],
                        "type": "word",
                        "name": "wireless-access-point-controller",
                    },
                    {
                        "words": ['var oemproductname = "mvc_howay6100")'],
                        "type": "word",
                        "name": "wireless-access-point-controller",
                    },
                    {
                        "words": ['src="images/acchtext.png"'],
                        "type": "word",
                        "name": "wireless-access-point-controller",
                    },
                    {
                        "words": ["版权所有 &copy 2009-2017</div>"],
                        "type": "word",
                        "name": "wireless-access-point-controller",
                    },
                    {
                        "words": ["ctl00_contentplaceholder1_dlttopvideos"],
                        "type": "word",
                        "name": "wise-education-cloud-masters",
                    },
                    {
                        "words": ['action="/wisepower/login.jsp'],
                        "type": "word",
                        "name": "wisepower-oa",
                    },
                    {
                        "words": ["/resources/metronic/scripts/hz-tools.js"],
                        "type": "word",
                        "name": "wiserice-system",
                    },
                    {
                        "words": ["<h4>请在下框里画图形来提交登录"],
                        "type": "word",
                        "name": "wiserice-system",
                    },
                    {
                        "words": ["WishOA_WebPlugin.js"],
                        "type": "word",
                        "name": "wishoa",
                    },
                    {
                        "words": ["wishoa_webplugin.js"],
                        "type": "word",
                        "name": "wishoa",
                    },
                    {
                        "words": ["https://seal.wosign.com/tws.js"],
                        "type": "word",
                        "name": "wosign-ssl-cert",
                    },
                    {
                        "words": ["https://seal.wosign.com/signature"],
                        "type": "word",
                        "name": "wosign-ssl-cert",
                    },
                    {
                        "words": ["<html><head><title>wowza media server"],
                        "type": "word",
                        "name": "wowza-media-server",
                    },
                    {
                        "part": "header",
                        "words": ["Server: wowzastreamingengine"],
                        "type": "word",
                        "name": "wowza-wowzastreamingengine",
                    },
                    {
                        "words": ["powered by <a href='http://www.wqcms.com"],
                        "type": "word",
                        "name": "wq-cms",
                    },
                    {"words": ["inc/wqcms.js"], "type": "word", "name": "wq-cms"},
                    {
                        "words": ["style/wangqi/style.css"],
                        "type": "word",
                        "name": "wq-cms",
                    },
                    {
                        "words": ["websocket servers index.html"],
                        "type": "word",
                        "name": "ws-server",
                    },
                    {
                        "words": ['class="login">物联网供应链与金融风险管理服务'],
                        "type": "word",
                        "name": "wsncm-iot",
                    },
                    {
                        "words": ['class="login">wsncm动态仓单系统'],
                        "type": "word",
                        "name": "wsncm-system",
                    },
                    {
                        "part": "header",
                        "words": ["Server: wso2 carbon server"],
                        "type": "word",
                        "name": "wso2-carbon-server",
                    },
                    {
                        "words": ["powered by wstmart"],
                        "type": "word",
                        "name": "wstmart",
                    },
                    {
                        "words": ['href="/wstmart/home/'],
                        "type": "word",
                        "name": "wstmart",
                    },
                    {
                        "words": ["static/styles/frame/basic.css"],
                        "type": "word",
                        "name": "wuliupingtai",
                    },
                    {
                        "words": ['<meta name="generator" content="wuzhicms'],
                        "type": "word",
                        "name": "wuzhicms",
                    },
                    {
                        "words": ['content="wuzhicms', "powered by wuzhicms"],
                        "type": "word",
                        "name": "wuzhicms",
                        "condition": "and",
                    },
                    {
                        "words": ['href="wrzcnet.ico'],
                        "type": "word",
                        "name": "wygk-product",
                    },
                    {
                        "words": ['<a href="mailto:webmaster@wrzc.net'],
                        "type": "word",
                        "name": "wygk-product",
                    },
                    {
                        "words": ["url = 'wrzcnet_vote.asp?stype=view';"],
                        "type": "word",
                        "name": "wygk-product",
                    },
                    {
                        "words": ['font-size: 1.2em; color: red;">new xampp'],
                        "type": "word",
                        "name": "xampp",
                    },
                    {"words": ['content="xampp '], "type": "word", "name": "xampp"},
                    {
                        "words": ['if (!getcookie("x_gu_sid"'],
                        "type": "word",
                        "name": "xbrother-monitor",
                    },
                    {
                        "words": [">digital anywhere platform</h2>"],
                        "type": "word",
                        "name": "xcyg-system",
                    },
                    {
                        "words": ["system/templates/xdcms/"],
                        "type": "word",
                        "name": "xdcms",
                    },
                    {
                        "words": ["http://www.xdoa.cn</a>"],
                        "type": "word",
                        "name": "xdoa-oa",
                    },
                    {
                        "words": ["北京创信达科技有限公司"],
                        "type": "word",
                        "name": "xdoa-oa",
                    },
                    {
                        "words": ["xnstyle.css", "xecure vpn manager"],
                        "type": "word",
                        "name": "xecure-vpn",
                        "condition": "and",
                    },
                    {"words": ["xnstyle.css"], "type": "word", "name": "xecurevpn"},
                    {
                        "words": ['window.location="/citrix/xenapp"'],
                        "type": "word",
                        "name": "xenapp",
                    },
                    {
                        "words": ["xheditor_lang/zh-cn.js"],
                        "type": "word",
                        "name": "xheditor",
                    },
                    {"words": ['class="xheditor'], "type": "word", "name": "xheditor"},
                    {"words": [".xheditor("], "type": "word", "name": "xheditor"},
                    {
                        "words": ["<title>杏和区域检验业务协同平台登录界面</title>"],
                        "type": "word",
                        "name": "xhlis-oa",
                    },
                    {
                        "words": ["/template/ant/css/anthomecomm.css"],
                        "type": "word",
                        "name": "xiaomayi",
                    },
                    {
                        "words": ["http://stat.xiaonaodai.com/stat.php"],
                        "type": "word",
                        "name": "xiaonaodai",
                    },
                    {
                        "words": [
                            "北京心海导航教育科技股份有限公司-中国心理网版权所有<br>"
                        ],
                        "type": "word",
                        "name": "xinhaisoft-system",
                    },
                    {
                        "words": ["../regist.asp?school="],
                        "type": "word",
                        "name": "xinhaisoft-system",
                    },
                    {
                        "words": ["北京新网数码信息技术有限公司 版权所有</span>"],
                        "type": "word",
                        "name": "xinnet-enterprise-mail",
                    },
                    {
                        "words": ['src="cgijson/getloginimg.php?img=logo'],
                        "type": "word",
                        "name": "xinnet-mail",
                    },
                    {
                        "words": ["/webmail//cssv2/tamail.css"],
                        "type": "word",
                        "name": "xinnet-mail",
                    },
                    {
                        "part": "header",
                        "words": ["Server: xitami"],
                        "type": "word",
                        "name": "xitami",
                    },
                    {"words": ["xiuno/xiunobbs"], "type": "word", "name": "xiuno"},
                    {
                        "words": [
                            'class="hidden-xs ewheaderrow"><img src="aspximages/htqy.png"'
                        ],
                        "type": "word",
                        "name": "xjhtqy-crm",
                    },
                    {
                        "words": [
                            '<input class="an1" name="btnrst" id="btnrst" type="reset" value=" " />'
                        ],
                        "type": "word",
                        "name": "xjhyt-system",
                    },
                    {
                        "words": ['class="wrap login_wrap"'],
                        "type": "word",
                        "name": "xjhyt-system",
                    },
                    {
                        "words": ["url(images/yh.jpg)"],
                        "type": "word",
                        "name": "xjhyt-system",
                    },
                    {"words": ["xmadmin.exirck.cn"], "type": "word", "name": "xmall"},
                    {"words": ["include/xoops.js"], "type": "word", "name": "xoops"},
                    {
                        "words": ['src="template/paper/'],
                        "type": "word",
                        "name": "xpaper",
                    },
                    {
                        "words": ["/app_qjuserinfo/qjuserinfoadd.jsp"],
                        "type": "word",
                        "name": "xtoa-oa",
                    },
                    {
                        "words": ["/images/default/first/xtoa_logo.png"],
                        "type": "word",
                        "name": "xtoa-oa",
                    },
                    {
                        "words": ['src="systemfiles/js/iawebclientactivexcheck.js"'],
                        "type": "word",
                        "name": "xtoa-oa",
                    },
                    {
                        "words": ["玄鸟流量管理平台"],
                        "type": "word",
                        "name": "xuanniao-traffic-management-platform",
                    },
                    {"words": ['alt="xunruicms"'], "type": "word", "name": "xunruicms"},
                    {
                        "words": ["分布式任务调度平台XXL-JOB"],
                        "type": "word",
                        "name": "xxl-job",
                    },
                    {"words": ["advfile/ad12.js"], "type": "word", "name": "xycms"},
                    {"words": ["power by xyhcms"], "type": "word", "name": "xyhcms"},
                    {"words": ["yabbtime.gettime()"], "type": "word", "name": "yabb"},
                    {"words": ["/yabb.js"], "type": "word", "name": "yabb"},
                    {
                        "words": ["神盾fs<sup>3</sup>文档安全共享系统v2.0</div>"],
                        "type": "word",
                        "name": "yadongsoft-fs3",
                    },
                    {"words": ["id=subnet"], "type": "word", "name": "yearning"},
                    {
                        "words": ["/public/js/knockout-3.4.1.debug.js"],
                        "type": "word",
                        "name": "yelala",
                    },
                    {
                        "words": [
                            '<form action="/index.php/home/login/login.html" method="post" id="login" class="form" '
                            'data-bind="submit: submitform">'
                        ],
                        "type": "word",
                        "name": "yelala",
                    },
                    {
                        "words": ['background="oa/images/index/oalogin.jpg"'],
                        "type": "word",
                        "name": "yfidea-oa",
                    },
                    {
                        "words": ['href="/css/vendors~index.acfeb.css"'],
                        "type": "word",
                        "name": "yichao-crmreporting",
                    },
                    {
                        "words": ['src="amy/webos/jpmanager.js"'],
                        "type": "word",
                        "name": "yichao-system",
                    },
                    {
                        "words": ["get started with yii"],
                        "type": "word",
                        "name": "yii-framework",
                    },
                    {
                        "words": [
                            "<script>document.location='/index.mpl?a=login'</script>"
                        ],
                        "type": "word",
                        "name": "yioks-campus-football-management-platform",
                    },
                    {"words": ['content="yiqicms'], "type": "word", "name": "yiqi-cms"},
                    {
                        "words": ["/authjsp/login.jsp"],
                        "type": "word",
                        "name": "yirui-iras",
                    },
                    {
                        "words": ["fe0174bb-f093-42af-ab20-7ec621d10488"],
                        "type": "word",
                        "name": "yirui-iras",
                    },
                    {
                        "words": [
                            "opms",
                            "opms管理系统,织蝶-企业应用系统为您的企业保驾护航",
                        ],
                        "type": "word",
                        "name": "yiyu-opms",
                        "condition": "and",
                    },
                    {
                        "words": ['name="hidden_isbiaozhun"'],
                        "type": "word",
                        "name": "yizhitong-e7",
                    },
                    {
                        "words": ["/ymail/default/js/menu.js"],
                        "type": "word",
                        "name": "ymail-optical-content-reading",
                    },
                    {"words": ["/yimioa.apk"], "type": "word", "name": "ymhome-oa"},
                    {
                        "words": ["sheight*window.screen.deviceydpi"],
                        "type": "word",
                        "name": "yonyou-ism",
                    },
                    {"words": ["login_main_bg"], "type": "word", "name": "yonyou-erp"},
                    {"words": ["login_owner"], "type": "word", "name": "yonyou-erp"},
                    {
                        "words": ["/nc/servlet/nc.ui.iufo.login.index"],
                        "type": "word",
                        "name": "yonyou-erp-nc",
                    },
                    {
                        "words": ["v_hedden", "v_show"],
                        "type": "word",
                        "name": "yonyou-fe",
                        "condition": "and",
                    },
                    {
                        "words": ['window.location.replace("login.jsp?up=1")'],
                        "type": "word",
                        "name": "yonyou-grp-u8",
                    },
                    {
                        "words": [
                            "/modules/core/client/views/sidemenu.client.view.html"
                        ],
                        "type": "word",
                        "name": "yonyou-intelligentplant",
                    },
                    {
                        "words": ["onmouseout=\"this.classname='btn btnoff'\""],
                        "type": "word",
                        "name": "yonyou-ksoa",
                    },
                    {
                        "words": ['href="clientfile/rmisupdate.exe'],
                        "type": "word",
                        "name": "yonyou-rmis",
                    },
                    {
                        "words": ["/seeyon/USER-DATA/IMAGES/LOGIN/login.gif"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {
                        "words": ["/seeyon/common/"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {
                        "words": ["M3 Server"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {
                        "words": ["M1-Server"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {
                        "words": ["/seeyon/user-data/images/login/login.gif"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {
                        "words": ["seeyon", "seeyonproductid"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                        "condition": "and",
                    },
                    {
                        "words": ["var _ctxpath = '/seeyon'"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {
                        "words": ["a8-v5企业版"],
                        "type": "word",
                        "name": "yonyou-seeyon-oa",
                    },
                    {"words": ["/seeyon/"], "type": "word", "name": "yonyou-seeyon-oa"},
                    {
                        "words": ['url:"/shophome/ajaxgetcompetemessagelist.action",'],
                        "type": "word",
                        "name": "yonyou-shop",
                    },
                    {
                        "words": [
                            '$.post("/shopfront/shoppingcar/gotoshoppingcartajax.action",function(data){'
                        ],
                        "type": "word",
                        "name": "yonyou-shop",
                    },
                    {
                        "words": ["北京用友政务软件股份有限公司"],
                        "type": "word",
                        "name": "yonyou-shop",
                    },
                    {
                        "words": ["turboui.js"],
                        "type": "word",
                        "name": "yonyou-turbocrm",
                    },
                    {"words": ["getfirstu8accid"], "type": "word", "name": "yonyou-u8"},
                    {
                        "words": ["开启u8 cloud云端之旅"],
                        "type": "word",
                        "name": "yonyou-u8-cloud",
                    },
                    {
                        "words": ["http-equiv=refresh content=0;url=index.jsp"],
                        "type": "word",
                        "name": "yonyou-uclient",
                    },
                    {
                        "words": ["/system/login/login.asp?appid="],
                        "type": "word",
                        "name": "yonyou-ufida",
                    },
                    {
                        "words": ["ufida_iufo_over.png", "ufida_nc.png"],
                        "type": "word",
                        "name": "yonyou-ufida-nc",
                        "condition": "and",
                    },
                    {
                        "words": ["logo/images/", "ufida"],
                        "type": "word",
                        "name": "yonyou-ufida-nc",
                        "condition": "and",
                    },
                    {
                        "words": ["logo/images/ufida_nc.png"],
                        "type": "word",
                        "name": "yonyou-ufida-nc",
                    },
                    {
                        "words": ['<div id="nc_text">'],
                        "type": "word",
                        "name": "yonyou-ufida-nc",
                    },
                    {
                        "words": ['<div id="nc_img" onmouseover="overimage(\'nc\');'],
                        "type": "word",
                        "name": "yonyou-ufida-nc",
                    },
                    {
                        "words": ['href="/static/assets/yottaweb-elements/index.css"'],
                        "type": "word",
                        "name": "yottabyte-rizhiyi",
                    },
                    {
                        "words": ['var id = document.getelementbyid("txtyhmm").value'],
                        "type": "word",
                        "name": "youhua-iemos_cw",
                    },
                    {
                        "words": ['/ashx/log/logincheck.ashx?fresh=" + math.random()'],
                        "type": "word",
                        "name": "youhuaopt-system",
                    },
                    {
                        "words": ['href="http://www.dbmailserver.com" target=_blank'],
                        "type": "word",
                        "name": "youngzsoft-dbmail",
                    },
                    {
                        "words": ["youphptube"],
                        "type": "word",
                        "name": "youphptube-encoder",
                    },
                    {
                        "words": ["yuannian.css"],
                        "type": "word",
                        "name": "yuanian-accounting-software",
                    },
                    {
                        "words": ["/image/logo/yuannian.gif"],
                        "type": "word",
                        "name": "yuanian-accounting-software",
                    },
                    {"words": ["<h1>驭龙</h1>"], "type": "word", "name": "yulong-hids"},
                    {
                        "words": ["<h2>yulong - a cool hids system.</h2>"],
                        "type": "word",
                        "name": "yulong-hids",
                    },
                    {
                        "words": [
                            'id=mtokenplugin width=0 height=0 style="position: absolute;left: 0px; top: 0px"'
                        ],
                        "type": "word",
                        "name": "yunanbao-yunxz",
                    },
                    {
                        "words": ["云翌ip呼叫中心</span>"],
                        "type": "word",
                        "name": "yuneasy-ipcalling",
                    },
                    {"words": ['href="/17rec.html"'], "type": "word", "name": "yunec"},
                    {
                        "words": ["ui/js/seaconfig.js"],
                        "type": "word",
                        "name": "yunhezi",
                    },
                    {
                        "words": ["ui/skins/black/style.css"],
                        "type": "word",
                        "name": "yunhezi",
                    },
                    {
                        "words": ['class="client-list dm-clear">'],
                        "type": "word",
                        "name": "yunhezi",
                    },
                    {
                        "words": ['action="/alimail/error/browserlog'],
                        "type": "word",
                        "name": "yunkemail",
                    },
                    {
                        "words": ['content="阿里企业邮箱'],
                        "type": "word",
                        "name": "yunkemail",
                    },
                    {
                        "words": ['href="http://bbs.yunsuo.com.cn'],
                        "type": "word",
                        "name": "yunsuo",
                    },
                    {
                        "words": ['<img class="yunsuologo"'],
                        "type": "word",
                        "name": "yunsuo",
                    },
                    {
                        "words": ['class="headerbar gradientbar'],
                        "type": "word",
                        "name": "yzruida-system",
                    },
                    {
                        "words": ['generator" content="z-blog'],
                        "type": "word",
                        "name": "z-blog",
                    },
                    {
                        "words": ["Zabbix SIA", "zabbix"],
                        "type": "word",
                        "name": "zabbix",
                        "condition": "and",
                    },
                    {
                        "words": ["images/general/zabbix.ico"],
                        "type": "word",
                        "name": "zabbix",
                    },
                    {
                        "words": ['background="images/login_sample_bgz.jpg"'],
                        "type": "word",
                        "name": "zbintel-system",
                    },
                    {
                        "words": ["_ZCMS_ShowNewMessage", "zcms_skin"],
                        "type": "word",
                        "name": "zcms",
                        "condition": "and",
                    },
                    {
                        "words": ["_zcms_shownewmessage", "zcms_skin"],
                        "type": "word",
                        "name": "zcms",
                        "condition": "and",
                    },
                    {"words": ["app=zcms"], "type": "word", "name": "zcms"},
                    {
                        "words": ["shopping cart program by zen cart"],
                        "type": "word",
                        "name": "zen_cart-shopping",
                    },
                    {
                        "part": "header",
                        "words": ["Set-Cookie: zenid="],
                        "type": "word",
                        "name": "zen_cart-shopping",
                    },
                    {
                        "words": ["$('#zentao').addClass('btn-success');"],
                        "type": "word",
                        "name": "zentao-system",
                    },
                    {
                        "words": ["zentao/theme"],
                        "type": "word",
                        "name": "zentao-system",
                    },
                    {
                        "words": ["<a id='zentaopro' href='/pro/'"],
                        "type": "word",
                        "name": "zentao-system",
                    },
                    {
                        "words": ["$('#zentaopro').addclass"],
                        "type": "word",
                        "name": "zentao-system",
                    },
                    {
                        "words": [
                            "powered by <a href='http://www.zentao.net' target='_blank'>zentaopms",
                            "welcome to use zentao!",
                        ],
                        "type": "word",
                        "name": "zentao-system",
                        "condition": "and",
                    },
                    {
                        "words": ["href='/zentao/favicon.ico"],
                        "type": "word",
                        "name": "zentao-system",
                    },
                    {
                        "words": ["server: cpws"],
                        "type": "word",
                        "name": "zentao-system",
                    },
                    {
                        "words": ['content="站点介绍"', "style/base/jw.css"],
                        "type": "word",
                        "name": "zfsoft-educational-management-system",
                        "condition": "and",
                    },
                    {
                        "words": ['href="/jwglxt/logo/favicon.ico"'],
                        "type": "word",
                        "name": "zfsoft-leaingrn",
                    },
                    {
                        "words": ["images/lgline.gif"],
                        "type": "word",
                        "name": "zhengdazhongri-education",
                    },
                    {
                        "words": [
                            "lb_hint",
                            'onclick="safecodeclick" src="safecode.aspx',
                        ],
                        "type": "word",
                        "name": "zhengdazhongri-education",
                        "condition": "and",
                    },
                    {"words": ['content="智睿软件'], "type": "word", "name": "zhirui"},
                    {"words": ["zhirui.js"], "type": "word", "name": "zhirui"},
                    {
                        "words": [
                            '<div class="g-alert-content" id="g-alert-contentsystem">'
                        ],
                        "type": "word",
                        "name": "zhongan-xdecision",
                    },
                    {
                        "words": ["clientutil.isff=!clientutil.isie"],
                        "type": "word",
                        "name": "zhongshengsoft-crm",
                    },
                    {
                        "words": ['alert("餐厅编号不能为空")'],
                        "type": "word",
                        "name": "zhongshengsoft-crm",
                    },
                    {
                        "words": ["var pubnewsarray"],
                        "type": "word",
                        "name": "zhongtan-ndstart",
                    },
                    {
                        "words": ["<title>南大之星信息发布系统 "],
                        "type": "word",
                        "name": "zhongtan-ndstart",
                    },
                    {
                        "words": ["background=zhongyou.jpg"],
                        "type": "word",
                        "name": "zhongyou-system",
                    },
                    {
                        "words": ["众友科技巡检管理软件"],
                        "type": "word",
                        "name": "zhongyou-system",
                    },
                    {
                        "words": ["您访问的是主机宝服务器默认页"],
                        "type": "word",
                        "name": "zhu-ji-bao",
                    },
                    {
                        "words": ['<a href="http://z.admin5.com/" target='],
                        "type": "word",
                        "name": "zhu-ji-bao",
                    },
                    {
                        "words": ["session.infocss.infocssurl"],
                        "type": "word",
                        "name": "zhuofansoft-cms",
                    },
                    {
                        "words": ['src="/static/images/login/btn-login.gif"'],
                        "type": "word",
                        "name": "zidesoft-e6",
                    },
                    {
                        "words": ["广州紫光华宇信息技术有限公司"],
                        "type": "word",
                        "name": "ziguanghuayu-attendance-management-system",
                    },
                    {"words": ["ImgZimbraIcon"], "type": "word", "name": "zimbra"},
                    {
                        "part": "header",
                        "words": ["Set-Cookie: ZM_LOGIN_CSRF"],
                        "type": "word",
                        "name": "zimbra",
                    },
                    {"words": ["window._zimbramail"], "type": "word", "name": "zimbra"},
                    {"words": ['content="zimbra'], "type": "word", "name": "zimbra"},
                    {
                        "words": ['<base href="/zipkin/">'],
                        "type": "word",
                        "name": "zipkin",
                    },
                    {
                        "words": ['content="模板系统xinnet'],
                        "type": "word",
                        "name": "zizhujianzhan",
                    },
                    {
                        "words": ['href="msnim:chat?contact=xinnet@hotmail.com'],
                        "type": "word",
                        "name": "zizhujianzhan",
                    },
                    {
                        "words": [
                            "onclick=\"showstate(gettext('forgotten password')) "
                        ],
                        "type": "word",
                        "name": "zknet-attendance-management",
                    },
                    {
                        "words": ["zknet", "zksoftware inc."],
                        "type": "word",
                        "name": "zknet-attendance-management",
                        "condition": "and",
                    },
                    {
                        "words": ["web考勤管理系统"],
                        "type": "word",
                        "name": "zknet-attendance-management",
                    },
                    {
                        "words": ["src='/login/images/zksecurity.png'", "百傲瑞达"],
                        "type": "word",
                        "name": "zkteco-security-management-system",
                        "condition": "and",
                    },
                    {
                        "words": [
                            'class="login-finger-btn disabled"',
                            'id="password_hidden"',
                        ],
                        "type": "word",
                        "name": "zkteco-security-management-system",
                        "condition": "and",
                    },
                    {
                        "words": [
                            '$(".copyright").text("copyright ? " + server_current_year + " zkteco co., ltd. all rights '
                            'reserved");'
                        ],
                        "type": "word",
                        "name": "zkteco-security-management-system",
                    },
                    {
                        "words": ['class="m-btn  zkgreen rnd"'],
                        "type": "word",
                        "name": "zkteco-system",
                    },
                    {
                        "words": ["background-image:url(images/devicebg.jpg)"],
                        "type": "word",
                        "name": "zkwell-corrosion-monitoring-and-corrosion-protection-management-system",
                    },
                    {
                        "words": ["list.asp?caseid="],
                        "type": "word",
                        "name": "znv-digital-campus",
                    },
                    {
                        "words": ["zoneminder login"],
                        "type": "word",
                        "name": "zoneminder",
                    },
                    {
                        "words": ['data-errormessage-value-missing="* 请录入用户名'],
                        "type": "word",
                        "name": "zonghousc-system",
                    },
                    {
                        "words": ["style/default/frui.css"],
                        "type": "word",
                        "name": "zonghousc-system",
                    },
                    {
                        "words": ['name="zoom_query"'],
                        "type": "word",
                        "name": "zoom-search-engine",
                    },
                    {
                        "words": ['class="alert alert-success hideme zoom-newmessage"'],
                        "type": "word",
                        "name": "zoommeeting",
                    },
                    {
                        "part": "header",
                        "words": ["X-Powered-By: Zope"],
                        "type": "word",
                        "name": "zope",
                    },
                    {
                        "words": ["/lib/js/apps/zotonic-1.0", "powered by: zotonic"],
                        "type": "word",
                        "name": "zotonic",
                        "condition": "and",
                    },
                    {
                        "words": ["深圳市中兴信息技术有限公司版权所有"],
                        "type": "word",
                        "name": "zte-police-research-system",
                    },
                    {
                        "words": ['src="img/gonanlogo.jpg'],
                        "type": "word",
                        "name": "zte-police-research-system",
                    },
                    {"words": ["help/zuitu.php"], "type": "word", "name": "zuitu"},
                    {
                        "words": ['obj.src = "createcheckcode.aspx?id"+strmath;'],
                        "type": "word",
                        "name": "zxoa",
                    },
                    {
                        "words": [
                            'name="button1" value="" onclick="javascript:return checkfrom();" id="button1" '
                            'class="loginbtn" />'
                        ],
                        "type": "word",
                        "name": "zxoa",
                    },
                    {
                        "words": ["/inc/showuserlogin.php?style=h&t=math.random()"],
                        "type": "word",
                        "name": "zzcms",
                    },
                    {
                        "words": ['href="/skins/bicycle/css/login.css"'],
                        "type": "word",
                        "name": "zzsmit-public-bicycle-management-system",
                    },
                    {
                        "words": ["Powered by <a href='http://zzzcms.com'>ZZZcms</a>"],
                        "type": "word",
                        "name": "zzzcms",
                    },
                    {
                        "words": ["<title page-title>Bitwarden Web Vault</title>"],
                        "type": "word",
                        "name": "bitwarden",
                    },
                    {
                        "part": "header",
                        "words": ["Server: OpenBSD httpd"],
                        "type": "word",
                        "name": "OpenBSD relayd",
                    },
                    {
                        "part": "header",
                        "words": ["Server: Hunchentoot"],
                        "type": "word",
                        "name": "Hunchentoot",
                    },
                    {
                        "words": [
                            "Weblate",
                            '<meta name="author" content="Michal Čihař" />',
                        ],
                        "type": "word",
                        "name": "weblate",
                        "condition": "and",
                    },
                    {
                        "case-insensitive": True,
                        "part": "header",
                        "words": ["x-dispatcher:"],
                        "type": "word",
                        "name": "Adobe Experience Manager (AEM)",
                    },
                ],
                "e": [],
            }
        ],
    }
]


import base64
from cruster import HttpRequest, HttpResponse, finding


def scan(request: HttpRequest, response: HttpResponse):
    del request
    try:
        body = base64.b64decode(response.body).decode("latin-1")
    except Exception:
        body = ""
    headers = []
    server = []
    for item in response.headers:
        try:
            value = base64.b64decode(item["value"]).decode("latin-1")
        except Exception:
            value = ""
        headers.append(item["name"] + ": " + value)
        if item["name"].lower() == "server":
            server.append(value)
    header = "\n".join(headers)
    server_value = "\n".join(server)
    evidence = []
    for signature in FINGERPRINTS[0]["r"][0]["m"]:
        source = (
            header
            if signature.get("part") == "header"
            else server_value if signature.get("part") == "server" else body
        )
        words = signature["words"]
        if signature.get("case-insensitive"):
            source = source.lower()
            words = [word.lower() for word in words]
        matches = [word in source for word in words]
        matched = all(matches) if signature.get("condition") == "and" else any(matches)
        if matched:
            evidence.append(signature["name"])
    return (
        [
            finding(
                "fingerprinthub-web-fingerprints",
                extracted=list(dict.fromkeys(evidence)),
            )
        ]
        if evidence
        else []
    )
