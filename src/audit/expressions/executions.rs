use crate::audit::AuditError;

use super::args::FunctionArg;

// ALL THESE FUNCTIONS CAN PANIC!
// TYPES SHOULD BE CHECKED EARLIER

pub(super) fn exec_equal(arg1: &FunctionArg, arg2: &FunctionArg) -> FunctionArg {
    match (arg1, arg2) {
        (FunctionArg::SEVERAL(array), FunctionArg::INTEGER(single)) => {
            let result = array.iter()
                .any(|fa| {
                    fa.integer().as_ref().unwrap() == single
                });

            return FunctionArg::BOOLEAN(result);
        },
        (FunctionArg::INTEGER(single), FunctionArg::SEVERAL(array)) => {
            let result = array.iter()
                .any(|fa| {
                    single == fa.integer().as_ref().unwrap()
                });

            return FunctionArg::BOOLEAN(result)
        },
        (FunctionArg::SEVERAL(array1), FunctionArg::SEVERAL(array2)) => {
            for i in 0..array1.len() {
                for j in 0..array2.len() {
                    if *array1[i].integer().as_ref().unwrap() == *array2[j].integer().as_ref().unwrap() {
                        return FunctionArg::BOOLEAN(true);
                    }
                }
            }

            return FunctionArg::BOOLEAN(false);
        },
        (FunctionArg::INTEGER(single1), FunctionArg::INTEGER(single2)) => {
            return FunctionArg::BOOLEAN(*single1 == *single2);
        },
        _ => {
            unreachable!("A try to compare non-integer values occured!")
        }
    }
}

pub(super) fn greater_than(arg1: &FunctionArg, arg2: &FunctionArg) -> FunctionArg {
    match (arg1, arg2) {
        (FunctionArg::SEVERAL(array), FunctionArg::INTEGER(single)) => {
            let result = array.iter()
                .any(|fa| {
                    fa.integer().as_ref().unwrap() > single
                });

            return FunctionArg::BOOLEAN(result);
        },
        (FunctionArg::INTEGER(single), FunctionArg::SEVERAL(array)) => {
            let result = array.iter()
                .any(|fa| {
                    single > fa.integer().as_ref().unwrap()
                });

            return FunctionArg::BOOLEAN(result)
        },
        (FunctionArg::SEVERAL(array1), FunctionArg::SEVERAL(array2)) => {
            for i in 0..array1.len() {
                for j in 0..array2.len() {
                    if *array1[i].integer().as_ref().unwrap() > *array2[j].integer().as_ref().unwrap() {
                        return FunctionArg::BOOLEAN(true);
                    }
                }
            }

            return FunctionArg::BOOLEAN(false);
        },
        (FunctionArg::INTEGER(single1), FunctionArg::INTEGER(single2)) => {
            return FunctionArg::BOOLEAN(*single1 > *single2);
        },
        _ => {
            unreachable!("A try to compare non-integer values occured!")
        }
    }
}

pub(super) fn less_than(arg1: &FunctionArg, arg2: &FunctionArg) -> FunctionArg {
    match (arg1, arg2) {
        (FunctionArg::SEVERAL(array), FunctionArg::INTEGER(single)) => {
            let result = array.iter()
                .any(|fa| {
                    fa.integer().as_ref().unwrap() < single
                });

            return FunctionArg::BOOLEAN(result);
        },
        (FunctionArg::INTEGER(single), FunctionArg::SEVERAL(array)) => {
            let result = array.iter()
                .any(|fa| {
                    single < fa.integer().as_ref().unwrap()
                });

            return FunctionArg::BOOLEAN(result)
        },
        (FunctionArg::SEVERAL(array1), FunctionArg::SEVERAL(array2)) => {
            for i in 0..array1.len() {
                for j in 0..array2.len() {
                    if *array1[i].integer().as_ref().unwrap() < *array2[j].integer().as_ref().unwrap() {
                        return FunctionArg::BOOLEAN(true);
                    }
                }
            }

            return FunctionArg::BOOLEAN(false);
        },
        (FunctionArg::INTEGER(single1), FunctionArg::INTEGER(single2)) => {
            return FunctionArg::BOOLEAN(*single1 < *single2);
        },
        _ => {
            unreachable!("A try to compare non-integer values occured!")
        }
    }
}

pub(super) fn greater_than_or_equal(arg1: &FunctionArg, arg2: &FunctionArg) -> FunctionArg {
    match (arg1, arg2) {
        (FunctionArg::SEVERAL(array), FunctionArg::INTEGER(single)) => {
            let result = array.iter()
                .any(|fa| {
                    fa.integer().as_ref().unwrap() >= single
                });

            return FunctionArg::BOOLEAN(result);
        },
        (FunctionArg::INTEGER(single), FunctionArg::SEVERAL(array)) => {
            let result = array.iter()
                .any(|fa| {
                    single >= fa.integer().as_ref().unwrap()
                });

            return FunctionArg::BOOLEAN(result)
        },
        (FunctionArg::SEVERAL(array1), FunctionArg::SEVERAL(array2)) => {
            for i in 0..array1.len() {
                for j in 0..array2.len() {
                    if *array1[i].integer().as_ref().unwrap() >= *array2[j].integer().as_ref().unwrap() {
                        return FunctionArg::BOOLEAN(true);
                    }
                }
            }

            return FunctionArg::BOOLEAN(false);
        },
        (FunctionArg::INTEGER(single1), FunctionArg::INTEGER(single2)) => {
            return FunctionArg::BOOLEAN(*single1 >= *single2);
        },
        _ => {
            unreachable!("A try to compare non-integer values occured!")
        }
    }
}

pub(super) fn less_than_or_equal(arg1: &FunctionArg, arg2: &FunctionArg) -> FunctionArg {
    match (arg1, arg2) {
        (FunctionArg::SEVERAL(array), FunctionArg::INTEGER(single)) => {
            let result = array.iter()
                .any(|fa| {
                    fa.integer().as_ref().unwrap() <= single
                });

            return FunctionArg::BOOLEAN(result);
        },
        (FunctionArg::INTEGER(single), FunctionArg::SEVERAL(array)) => {
            let result = array.iter()
                .any(|fa| {
                    single <= fa.integer().as_ref().unwrap()
                });

            return FunctionArg::BOOLEAN(result)
        },
        (FunctionArg::SEVERAL(array1), FunctionArg::SEVERAL(array2)) => {
            for i in 0..array1.len() {
                for j in 0..array2.len() {
                    if *array1[i].integer().as_ref().unwrap() <= *array2[j].integer().as_ref().unwrap() {
                        return FunctionArg::BOOLEAN(true);
                    }
                }
            }

            return FunctionArg::BOOLEAN(false);
        },
        (FunctionArg::INTEGER(single1), FunctionArg::INTEGER(single2)) => {
            return FunctionArg::BOOLEAN(*single1 <= *single2);
        },
        _ => {
            unreachable!("A try to compare non-integer values occured!")
        }
    }
}

pub(super) fn exec_string_length(arg: &FunctionArg) -> Result<FunctionArg, AuditError> {
    match arg {
        FunctionArg::STRING(s) => {
            return Ok(FunctionArg::INTEGER(s.len()));
        },
        FunctionArg::SEVERAL(sev) => {
            let lens = sev
                .iter()
                .map(|fa| {
                    FunctionArg::INTEGER(fa.string().as_ref().unwrap().len())
                })
                .collect::<Vec<FunctionArg>>();

            return Ok(FunctionArg::SEVERAL(lens));
        },
        _ => {
            let err_str = format!("Cannot compute len for data of type {:?}", arg);
            return Err(AuditError(err_str));
        }
    }
}

pub(super) fn exec_str_match_regex(arg1: &FunctionArg, arg2: &FunctionArg) -> Result<FunctionArg, AuditError> {
    let str_re = arg1.string().unwrap();
    let re = match regex::Regex::new(&str_re) {
        Ok(re) => re,
        Err(err) => return Err(AuditError(format!("Could not parse regex {} in match function: {}", &str_re, err)))
    };

    match arg2 {
        FunctionArg::STRING(s) => {
            return Ok(FunctionArg::BOOLEAN(re.is_match(s)));
        },
        FunctionArg::SEVERAL(array) => {
            let result = array
                .iter()
                .any(|fa| {
                    re.is_match(fa.string().as_ref().unwrap())
                });

            return Ok(FunctionArg::BOOLEAN(result));
        },
        _ => {
            let err_str = format!("Cannot execute regex match for data of type {:?}", arg2);
            return Err(AuditError(err_str));
        }
    }
}