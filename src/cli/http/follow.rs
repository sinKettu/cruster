use crate::cli::CrusterCLIError;
use clap::ArgMatches;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;


pub(crate) struct HttpFollowSettings {
    pub(crate) no_old_lines: bool,
}

impl TryFrom<&ArgMatches> for HttpFollowSettings {
    type Error = CrusterCLIError;
    fn try_from(args: &ArgMatches) -> Result<Self, Self::Error> {
        let mut settings = HttpFollowSettings {
            no_old_lines: false
        };

        settings.no_old_lines = args.get_flag("no-old-lines");

        return Ok(settings);
    }
}

pub(crate) fn exec(settings: &HttpFollowSettings, path: &str) -> Result<(), CrusterCLIError> {
    let fin = std::fs::File::open(path)?;
    let mut reader = BufReader::new(fin);

    let mut buf = String::with_capacity(1000000);
    while reader.read_line(&mut buf)? > 0 {
        if ! settings.no_old_lines {
            print!("{}", &buf);
        }

        buf.clear();
    }

    loop {
        while reader.read_line(&mut buf)? == 0 {
            sleep(Duration::from_millis(100));
        }

        print!("{}", &buf);
        buf.clear();
    }
}
