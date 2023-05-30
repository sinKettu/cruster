use clap::ArgMatches;
use crate::cli::CrusterCLIError;


pub(crate) struct RepeaterExecSettings {
    pub(crate) name: Option<String>,
    pub(crate) number: Option<usize>,
    pub(crate) force: bool
}

impl TryFrom<&ArgMatches> for RepeaterExecSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = RepeaterExecSettings {
            name: None,
            number: None,
            force: false
        };

        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            settings.number = Some(number);
        }
        else {
            settings.name = Some(mark);
        }

        settings.force = args.get_flag("force");

        if settings.name.is_none() && settings.number.is_none() {
            return Err(
                CrusterCLIError::from("Use must specify number or name of repeater to work with")
            )
        }

        return Ok(settings);
    }
}

pub(crate) fn execute(settings: &RepeaterExecSettings, path: &str) -> Result<(), CrusterCLIError> {


    todo!()
}