use crate::cli::CrusterCLIError;
use clap::ArgMatches;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;
use serde_json as json;
use crate::http_storage;


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
            let line_ptr = &buf;
            let serializable_data: http_storage::serializable::SerializableProxyData = json::from_str(line_ptr)?;
            let pair: http_storage::RequestResponsePair = serializable_data.try_into()?;

            super::print_briefly(&pair, false);
        }

        buf.clear();
    }

    loop {
        while reader.read_line(&mut buf)? == 0 {
            sleep(Duration::from_millis(100));
        }

        let line_ptr = &buf;
        let serializable_data: http_storage::serializable::SerializableProxyData = json::from_str(line_ptr)?;
        let pair: http_storage::RequestResponsePair = serializable_data.try_into()?;
        super::print_briefly(&pair, false);
        
        buf.clear();
    }
}
