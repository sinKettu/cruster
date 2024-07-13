use clap::ArgMatches;

use crate::cli::CrusterCLIError;

pub(crate) struct AuditPrintConfig {
    pub(crate) audit_name: String,
    pub(crate) all: bool,
    pub(crate) index: usize,
    pub(crate) wout_body: bool,
}

impl TryFrom<&ArgMatches> for AuditPrintConfig {
    type Error = CrusterCLIError;
    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let audit_name = value.get_one::<String>("name").unwrap().to_owned();

        if value.get_flag("all") {
            return Ok(
                AuditPrintConfig {
                    audit_name,
                    all: true,
                    index: 0,
                    wout_body: false
                }
            )
        }
        else {
            let index = value.get_one::<usize>("index").unwrap().to_owned();
            let wout_body = value.get_flag("without-body");

            return Ok(
                AuditPrintConfig {
                    audit_name,
                    all: false,
                    index,
                    wout_body
                }
            )
        }
    }
}

pub(crate) async fn exec(print_conf: AuditPrintConfig, results: String) -> Result<(), CrusterCLIError> {
    


    Ok(())
}