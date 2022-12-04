use regex::Regex;
use log::debug;

pub(super) fn make_re_list(str_re: &[String]) -> Vec<Regex> {
    let result: Vec<Regex> = str_re
        .iter()
        .map(|s| { Regex::new(s).expect(&format!("Cannot compile regex from '{}'", s)) })
        .collect();

    return result;
}

fn fit_regex_list(s: &str, res: &[Regex]) -> bool {
    debug!("URI: {}", s);
    debug!("Regexes: {:?}", res);
    let mut fit: bool = false;
    for re in res {
        if re.is_match(s) {
            fit = true;
            break;
        }
    }

    debug!("Fit: {}", fit);
    return fit;
}

pub(super) fn fit_included(uri: &str, inc: &[Regex]) -> bool {
    return fit_regex_list(uri, inc);
}

/// Returns `true` if no matches found, `false` otherwise
pub(super) fn fit_excluded(uri: &str, exc: &[Regex]) -> bool {
    let fit = fit_regex_list(uri, exc);
    return !fit;
}

pub(super) fn fit(uri: &str, inc: &[Regex], exc: &[Regex]) -> bool {
    return fit_included(uri, inc) && fit_excluded(uri, exc);
}
