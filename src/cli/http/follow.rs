use crate::cli::CrusterCLIError;
use clap::ArgMatches;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;


pub(crate) struct HttpFollowSettings {
    pub(crate) no_old_lines: bool,
    pub(crate) urls: bool,
}

impl TryFrom<&ArgMatches> for HttpFollowSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = HttpFollowSettings {
            no_old_lines: false,
            urls: false
        };

        settings.no_old_lines = args.get_flag("no-old-lines");
        settings.urls = args.get_flag("urls");

        return Ok(settings);
    }
}

pub(crate) fn exec(settings: &HttpFollowSettings, path: &str) -> Result<(), CrusterCLIError> {
    let fin = std::fs::File::open(path)?;
    let mut reader = BufReader::new(fin);

    let mut pending = Vec::with_capacity(1000000);
    while let Some(line) = super::read_history_line(&mut reader, &mut pending, true)? {
        if !settings.no_old_lines {
            print_line(&line.raw, settings)?;
        }
    }

    loop {
        if let Some(line) = super::read_history_line(&mut reader, &mut pending, true)? {
            print_line(&line.raw, settings)?;
        } else {
            sleep(Duration::from_millis(100));
        }
    }
}

fn print_line(raw: &str, settings: &HttpFollowSettings) -> Result<(), CrusterCLIError> {
    let pair = super::parse_history_pair(raw)?;
    if settings.urls {
        super::print_urls(&pair);
    } else {
        super::print_briefly(&pair, false);
    }
    Ok(())
}
