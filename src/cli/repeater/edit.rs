use clap::ArgMatches;
use super::CrusterCLIError;
use crate::siv_ui::repeater::RepeaterState;

pub(crate) struct RepeaterEditSettings {
    pub(crate) name_to_get: Option<String>,
    pub(crate) number_to_get: Option<usize>,
    pub(crate) name: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) redirects: Option<bool>,
    pub(crate) max_redirects: Option<usize>,
    pub(crate) https: Option<bool>
}

impl TryFrom<&ArgMatches> for RepeaterEditSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = RepeaterEditSettings {
            name_to_get: None,
            number_to_get: None,
            name: None,
            address: None,
            redirects: None,
            max_redirects: None,
            https: None,
        };

        let mark = args.get_one::<String>("mark").unwrap().to_string();
        if let Ok(number) = mark.parse::<usize>() {
            settings.number_to_get = Some(number);
        }
        else {
            settings.name_to_get = Some(mark);
        }
        
        if let Some(name) = args.get_one::<String>("name") {
            settings.name = Some(name.clone());
        }

        if let Some(address) = args.get_one::<String>("address") {
            settings.address = Some(address.clone());
        }

        if let Some(redirects) = args.get_one::<String>("redirects") {
            if redirects == "true" {
                settings.redirects = Some(true)
            } else if redirects == "false" {
                settings.redirects = Some(false)
            }
            else {
                unreachable!()
            }
        }

        if let Some(https) = args.get_one::<String>("https") {
            if https == "true" {
                settings.https = Some(true)
            } else if https == "false" {
                settings.https = Some(false)
            } else {
                unreachable!()
            }
        }

        if let Some(max_redirects) = args.get_one::<String>("max_redirects") {
            let number: usize = max_redirects.parse()?;
            settings.max_redirects = Some(number);
        }

        return Ok(settings);
    }
}

pub(crate) fn execute(settings: &RepeaterEditSettings, path: &str) -> Result<(), CrusterCLIError> {
    let update_closure = |settings: &RepeaterEditSettings, rep: &mut RepeaterState | {
        if let Some(name) = settings.name.as_ref() {
            rep.name = name.to_string();
        }

        if let Some(https) = settings.https.as_ref() {
            rep.parameters.https = https.clone();
        }

        if let Some(redirects) = settings.redirects.as_ref() {
            rep.parameters.redirects = redirects.clone();
        }

        if let Some(max_redirects) = settings.max_redirects.as_ref() {
            rep.parameters.max_redirects = max_redirects.clone();
        }

        if let Some(address) = settings.address.as_ref() {
            rep.parameters.address = address.clone();
        }
    };

    let repeater_iter = super::RepeaterIterator::new(path);
    let mut number_to_update: usize = 0;
    let mut repeater_to_update: Option<RepeaterState> = None;

    for (i, mut repeater) in repeater_iter.enumerate() {
        if let Some(number) = settings.number_to_get {
            if number == (i + 1) {
                number_to_update = i;
                update_closure(settings, &mut repeater);
                repeater_to_update = Some(repeater);
                break;
            }

            continue;
        }

        if let Some(name) = settings.name_to_get.as_ref() {
            if name == &repeater.name {
                number_to_update = i;
                update_closure(settings, &mut repeater);
                repeater_to_update = Some(repeater);
                break;
            }

            continue;
        }
    }

    if let Some(repeater) = repeater_to_update {
        super::update_repeaters(path, &repeater, number_to_update)
    } else {
        Err(
            CrusterCLIError::from("Could not find apropriate repeater to edit")
        )
    }
}