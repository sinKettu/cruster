use super::CrusterCLIError;
use clap::ArgMatches;

#[derive(Default)]
pub(crate) struct RepeaterEditSettings {
    pub(crate) name_to_get: Option<String>,
    pub(crate) number_to_get: Option<usize>,
    pub(crate) name: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) redirects: Option<bool>,
    pub(crate) max_redirects: Option<usize>,
    pub(crate) https: Option<bool>,
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
        } else {
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
            } else {
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

impl RepeaterEditSettings {
    pub(crate) fn apply(&self, rep: &mut crate::siv_ui::repeater::RepeaterStateSerializable) {
        if let Some(name) = &self.name {
            rep.name = name.clone();
        }
        if let Some(address) = &self.address {
            rep.parameters.address = address.clone();
        }
        if let Some(https) = self.https {
            rep.parameters.https = https;
        }
        if let Some(redirects) = self.redirects {
            rep.parameters.redirects = redirects;
        }
        if let Some(max) = self.max_redirects {
            rep.parameters.max_redirects = max;
        }
    }
}

pub(crate) fn execute(settings: &RepeaterEditSettings, path: &str) -> Result<(), CrusterCLIError> {
    let (index, mut records) = super::operations::load_selected(
        path,
        settings.number_to_get,
        settings.name_to_get.as_deref(),
    )?;
    settings.apply(&mut records[index]);
    super::update_repeaters(std::path::Path::new(path), &records[index], index)
}
