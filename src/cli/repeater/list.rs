use crate::cli::CrusterCLIError;
use crate::siv_ui::repeater::{
    RepeaterState,
    // RepeaterParameters,
    // RepeaterStateSerializable,
};

use super::RepeaterIterator;

pub(super) fn print_repeater_state(repeater_state: &RepeaterState, number: usize) {
    println!("\n{:<18}: {}", "Number", number + 1);
    println!("{:<18}: {}", "Name", &repeater_state.name);
    println!("{:<18}: {}", "Host", &repeater_state.parameters.address);
    println!("{:<18}: {}", "HTTPS", &repeater_state.parameters.https);
    println!("{:<18}: {}", "Redirects", &repeater_state.parameters.redirects);
    if repeater_state.parameters.redirects {
        println!("{:<18}: {}", "Maximum redirects", repeater_state.parameters.max_redirects)
    }
}


pub(crate) fn execute(repeater_state_path: &str) -> Result<(), CrusterCLIError> {
    if ! std::path::Path::new(repeater_state_path).is_file() {
        return Err(
            CrusterCLIError::from(
                format!("Cannot find file with repeater's state at {}", repeater_state_path)
            )
        );
    }

    let repeater_iter = RepeaterIterator::new(repeater_state_path);
    for (i, repeater) in repeater_iter.enumerate() {
        print_repeater_state(&repeater, i);
    }

    Ok(())
}