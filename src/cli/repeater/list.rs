use clap::ArgMatches;

use crate::cli::CrusterCLIError;

pub(crate) struct RepeaterListSettings {
    pub(crate) verbose: bool
}

impl TryFrom<&ArgMatches> for RepeaterListSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let verbose = args.get_flag("verbose");
        return Ok(
            RepeaterListSettings {
                verbose
            }
        );
    }
}


pub(crate) fn execute(args: &ArgMatches) -> Result<(), CrusterCLIError> {
    todo!()
}