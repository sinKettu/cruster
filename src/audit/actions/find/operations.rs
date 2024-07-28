mod equal;
mod greater;
mod and;
mod or;

use std::sync::Arc;

use crate::audit::types::OpArgWithRef;

use super::args::{ExecutableExpressionArgsTypes, ExecutableExpressionArgsValues};
use regex;

// fn _equal(left: &ExecutableExpressionArgsValues, right: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
//     left.equal(right)
// }

pub(super) trait Operations {
    fn equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn greater(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn less(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn greater_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn less_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn len(&self) -> ExecutableExpressionArgsValues;
    fn re_match(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn and(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn or(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
}

fn process_both_args_several(arg1: &Vec<ExecutableExpressionArgsValues>, arg2: &Vec<ExecutableExpressionArgsValues>, f: fn(&ExecutableExpressionArgsValues, &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
    let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
    for i in 0..arg1.len() {
        if i == arg2.len() {
            break
        }

        let iter_arg1 = &arg1[i];
        let iter_arg2 = &arg2[i];

        let res = f(iter_arg1, iter_arg2);
        collected.push(res);
    }

    ExecutableExpressionArgsValues::Several(collected)
}

fn process_right_arg_several(arg1: &ExecutableExpressionArgsValues, arg2: &Vec<ExecutableExpressionArgsValues>, f: fn(&ExecutableExpressionArgsValues, &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
    let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
    for i in 0..arg2.len() {
        let iter_arg2 = &arg2[i];

        let res = f(arg1, iter_arg2);
        collected.push(res);
    }

    ExecutableExpressionArgsValues::Several(collected)
}

fn process_left_arg_several(arg1: &Vec<ExecutableExpressionArgsValues>, arg2: &ExecutableExpressionArgsValues, f: fn(&ExecutableExpressionArgsValues, &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
    let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
    for i in 0..arg1.len() {
        let iter_arg1 = &arg1[i];

        let res = f(iter_arg1, arg2);
        collected.push(res);
    }

    ExecutableExpressionArgsValues::Several(collected)
}

fn process_both_args_wref(arg1_with_ref: &Arc<OpArgWithRef>, arg2_with_ref: &Arc<OpArgWithRef>, f: fn(&ExecutableExpressionArgsValues, &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
    let arg1 = &arg1_with_ref.arg;
    let arg2 = &arg2_with_ref.arg;

    let result = f(arg1, arg2);

    if arg2_with_ref.refer[0].send_action_id > arg1_with_ref.refer[0].send_action_id {
        ExecutableExpressionArgsValues::WithSendResReference(
            Arc::new(
                OpArgWithRef {
                    arg: result,
                    refer: arg2_with_ref.refer.to_owned(),
                    one_arg: arg2_with_ref.one_arg
                }
            )
        )
    }
    else {
        ExecutableExpressionArgsValues::WithSendResReference(
            Arc::new(
                OpArgWithRef {
                    arg: result,
                    refer: arg1_with_ref.refer.to_owned(),
                    one_arg: arg1_with_ref.one_arg
                }
            )
        )
    }
}

fn process_left_arg_wref(arg1_with_ref: &Arc<OpArgWithRef>, arg2: &ExecutableExpressionArgsValues, f: fn(&ExecutableExpressionArgsValues, &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
    let arg1 = &arg1_with_ref.arg;
    let result = f(arg1, arg2);

    ExecutableExpressionArgsValues::WithSendResReference(
        Arc::new(
            OpArgWithRef {
                arg: result,
                refer: arg1_with_ref.refer.to_owned(),
                one_arg: arg1_with_ref.one_arg
            }
        )
    )
}

fn process_right_arg_wref(arg1: &ExecutableExpressionArgsValues, arg2_with_ref: &Arc<OpArgWithRef>, f: fn(&ExecutableExpressionArgsValues, &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
    let arg2 = &arg2_with_ref.arg;
    let result = f(arg1, arg2);

    ExecutableExpressionArgsValues::WithSendResReference(
        Arc::new(
            OpArgWithRef {
                arg: result,
                refer: arg2_with_ref.refer.to_owned(),
                one_arg: arg2_with_ref.one_arg
            }
        )
    )   
}


impl Operations for ExecutableExpressionArgsValues {
    fn equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        equal::exec(&self, &arg)
    }

    fn greater(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        greater::exec(&self, &arg)
    }

    // TODO: rewrite other function like equal, greater

    fn greater_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::INTEGER && self.get_type() != arg.get_type() {
            unreachable!("Trying to compare (>) args with non-integer types, but it should be catched earlier");
        }

        match (self, arg) {
            (Self::Several(arg1), Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    if i == arg2.len() {
                        break
                    }

                    let iter_arg1 = &arg1[i];
                    let iter_arg2 = &arg2[i];

                    let res = iter_arg1.greater_or_equal(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::Several(arg1), _) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg1 = &arg1[i];

                    let res = iter_arg1.greater_or_equal(arg);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (_, Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
                for i in 0..arg2.len() {
                    let iter_arg2 = &arg2[i];

                    let res = self.greater_or_equal(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::WithSendResReference(arg1_with_ref), Self::WithSendResReference(arg2_with_ref)) => {
                let arg1 = &arg1_with_ref.arg;
                let arg2 = &arg2_with_ref.arg;

                let result = arg1.greater_or_equal(arg2);

                if arg2_with_ref.refer[0].send_action_id > arg1_with_ref.refer[0].send_action_id {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: result,
                                refer: arg2_with_ref.refer.to_owned(),
                                one_arg: arg2_with_ref.one_arg
                            }
                        )
                    )
                }
                else {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: result,
                                refer: arg1_with_ref.refer.to_owned(),
                                one_arg: arg1_with_ref.one_arg
                            }
                        )
                    )
                }
            },
            (_, Self::WithSendResReference(arg2_with_ref)) => {
                let arg2 = &arg2_with_ref.arg;
                let result = self.greater_or_equal(arg2);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg2_with_ref.refer.to_owned(),
                            one_arg: arg2_with_ref.one_arg
                        }
                    )
                )
            },
            (Self::WithSendResReference(arg1_with_ref), _) => {
                let arg1 = &arg1_with_ref.arg;
                let result = arg1.greater_or_equal(arg);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg1_with_ref.refer.to_owned(),
                            one_arg: arg1_with_ref.one_arg
                        }
                    )
                )
            },
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() >= arg.integer())
            },
            _ => {
                unreachable!("In 'equal' method of Operations found the case: {:?}, {:?}, but it should not exists", self, arg)
            }
        }
    }

    fn less(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::INTEGER && self.get_type() != arg.get_type() {
            unreachable!("Trying to compare (>) args with non-integer types, but it should be catched earlier");
        }

        match (self, arg) {
            (Self::Several(arg1), Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    if i == arg2.len() {
                        break
                    }

                    let iter_arg1 = &arg1[i];
                    let iter_arg2 = &arg2[i];

                    let res = iter_arg1.less(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::Several(arg1), _) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg1 = &arg1[i];

                    let res = iter_arg1.less(arg);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (_, Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
                for i in 0..arg2.len() {
                    let iter_arg2 = &arg2[i];

                    let res = self.less(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::WithSendResReference(arg1_with_ref), Self::WithSendResReference(arg2_with_ref)) => {
                let arg1 = &arg1_with_ref.arg;
                let arg2 = &arg2_with_ref.arg;

                let result = arg1.less(arg2);

                if arg2_with_ref.refer[0].send_action_id > arg1_with_ref.refer[0].send_action_id {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: result,
                                refer: arg2_with_ref.refer.to_owned(),
                                one_arg: arg2_with_ref.one_arg
                            }
                        )
                    )
                }
                else {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: result,
                                refer: arg1_with_ref.refer.to_owned(),
                                one_arg: arg1_with_ref.one_arg
                            }
                        )
                    )
                }
            },
            (_, Self::WithSendResReference(arg2_with_ref)) => {
                let arg2 = &arg2_with_ref.arg;
                let result = self.less(arg2);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg2_with_ref.refer.to_owned(),
                            one_arg: arg2_with_ref.one_arg
                        }
                    )
                )
            },
            (Self::WithSendResReference(arg1_with_ref), _) => {
                let arg1 = &arg1_with_ref.arg;
                let result = arg1.less(arg);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg1_with_ref.refer.to_owned(),
                            one_arg: arg1_with_ref.one_arg
                        }
                    )
                )
            },
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() < arg.integer())
            },
            _ => {
                unreachable!("In 'equal' method of Operations found the case: {:?}, {:?}, but it should not exists", self, arg)
            }
        }
    }

    fn less_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::INTEGER && self.get_type() != arg.get_type() {
            unreachable!("Trying to compare (>) args with non-integer types, but it should be catched earlier");
        }

        match (self, arg) {
            (Self::Several(arg1), Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    if i == arg2.len() {
                        break
                    }

                    let iter_arg1 = &arg1[i];
                    let iter_arg2 = &arg2[i];

                    let res = iter_arg1.greater_or_equal(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::Several(arg1), _) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg1 = &arg1[i];

                    let res = iter_arg1.greater_or_equal(arg);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (_, Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
                for i in 0..arg2.len() {
                    let iter_arg2 = &arg2[i];

                    let res = self.greater_or_equal(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::WithSendResReference(arg1_with_ref), Self::WithSendResReference(arg2_with_ref)) => {
                let arg1 = &arg1_with_ref.arg;
                let arg2 = &arg2_with_ref.arg;

                let result = arg1.less_or_equal(arg2);

                if arg2_with_ref.refer[0].send_action_id > arg1_with_ref.refer[0].send_action_id {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: result,
                                refer: arg2_with_ref.refer.to_owned(),
                                one_arg: arg2_with_ref.one_arg
                            }
                        )
                    )
                }
                else {
                    ExecutableExpressionArgsValues::WithSendResReference(
                        Arc::new(
                            OpArgWithRef {
                                arg: result,
                                refer: arg1_with_ref.refer.to_owned(),
                                one_arg: arg1_with_ref.one_arg
                            }
                        )
                    )
                }
            },
            (_, Self::WithSendResReference(arg2_with_ref)) => {
                let arg2 = &arg2_with_ref.arg;
                let result = self.less_or_equal(arg2);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg2_with_ref.refer.to_owned(),
                            one_arg: arg2_with_ref.one_arg
                        }
                    )
                )
            },
            (Self::WithSendResReference(arg1_with_ref), _) => {
                let arg1 = &arg1_with_ref.arg;
                let result = arg1.less_or_equal(arg);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg1_with_ref.refer.to_owned(),
                            one_arg: arg1_with_ref.one_arg
                        }
                    )
                )
            },
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() <= arg.integer())
            },
            _ => {
                unreachable!("In 'equal' method of Operations found the case: {:?}, {:?}, but it should not exists", self, arg)
            }
        }
    }

    fn len(&self) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::STRING {
            unreachable!("Tried to get length from non-string argument: {:?}", self)
        }

        match self {
            Self::Several(arg1) => {
                // println!("{:?}", arg1);
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg2 = &arg1[i];

                    let res = iter_arg2.len();
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            Self::WithSendResReference(arg_with_ref) => {
                let arg1 = &arg_with_ref.arg;
                let result = arg1.len();

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg_with_ref.refer.to_owned(), // TODO: fix it, bad way
                            one_arg: arg_with_ref.one_arg
                        }
                    )
                )
            },
            Self::String(s) => {
                ExecutableExpressionArgsValues::Integer(s.len() as i64)
            },
            _ => {
                unreachable!("In len operations found the following case that shoud not exist: {:?}", self)
            }
        }
    }

    // Self is RE and arg is string to match
    fn re_match(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::STRING && self.get_type() != arg.get_type() {
            unreachable!("Tried to match string with re, but found the following arg types: {:?}, {:?}", self, arg)
        }

        match (self, arg) {
            (Self::String(str_re), Self::String(str_arg)) => {
                let Ok(re) = regex::Regex::new(str_re.as_str()) else {
                    panic!("Cannot parse the string '{}' as regex", str_re)
                };

                ExecutableExpressionArgsValues::Boolean(re.find(str_arg).is_some())
            },
            (_, Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
                for i in 0..arg2.len() {
                    let iter_arg2 = &arg2[i];

                    let res = self.re_match(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (_, Self::WithSendResReference(arg2_with_ref)) => {
                let arg2 = &arg2_with_ref.arg;
                let result = self.re_match(arg2);

                ExecutableExpressionArgsValues::WithSendResReference(
                    Arc::new(
                        OpArgWithRef {
                            arg: result,
                            refer: arg2_with_ref.refer.to_owned(), // TODO: fix it, bad way
                            one_arg: arg2_with_ref.one_arg
                        }
                    )
                )
            }
            _ => {
                unreachable!("In re_match operation found the following case that shoud not exist: {:?}, {:?}", self, arg)
            }
        }
    }

    fn and(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        and::exec(&self, arg)
    }

    fn or(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        or::exec(&self, arg)
    }
}