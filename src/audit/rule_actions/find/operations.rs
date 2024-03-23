use super::expression_args::{ExecutableExpressionArgsTypes, ExecutableExpressionArgsValues};
use regex;

pub(super) trait Operations {
    fn equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn greater(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn less(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn greater_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn less_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
    fn len(&self) -> ExecutableExpressionArgsValues;
    fn re_match(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues;
}

impl Operations for ExecutableExpressionArgsValues {
    fn equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != arg.get_type() {
            unreachable!("Trying to compare args with diffrerent types, but it should be catched earlier");
        }

        match (self, arg) {
            (Self::Boolean(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.boolean() == arg.boolean())
            },
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() == arg.integer())
            },
            (Self::String(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.string() == arg.string())
            },
            (Self::Several(arg1), Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    if i == arg2.len() {
                        break
                    }

                    let iter_arg1 = &arg1[i];
                    let iter_arg2 = &arg2[i];

                    let res = iter_arg1.equal(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::Several(arg1), _) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg1 = &arg1[i];

                    let res = iter_arg1.equal(arg);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (_, Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
                for i in 0..arg2.len() {
                    let iter_arg2 = &arg2[i];

                    let res = self.equal(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            _ => {
                unreachable!("In 'equal' method of Operations found the case: {:?}, {:?}, but it should not exists", self, arg)
            }
        }
    }

    fn greater(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::INTEGER && self.get_type() != arg.get_type() {
            unreachable!("Trying to compare (>) args with non-integer types, but it should be catched earlier");
        }

        match (self, arg) {
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() > arg.integer())
            },
            (Self::Several(arg1), Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    if i == arg2.len() {
                        break
                    }

                    let iter_arg1 = &arg1[i];
                    let iter_arg2 = &arg2[i];

                    let res = iter_arg1.greater(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (Self::Several(arg1), _) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg1 = &arg1[i];

                    let res = iter_arg1.greater(arg);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            (_, Self::Several(arg2)) => {
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg2.len());
                for i in 0..arg2.len() {
                    let iter_arg2 = &arg2[i];

                    let res = self.greater(iter_arg2);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            },
            _ => {
                unreachable!("In 'equal' method of Operations found the case: {:?}, {:?}, but it should not exists", self, arg)
            }
        }
    }

    fn greater_or_equal(&self, arg: &ExecutableExpressionArgsValues) -> ExecutableExpressionArgsValues {
        if self.get_type() != ExecutableExpressionArgsTypes::INTEGER && self.get_type() != arg.get_type() {
            unreachable!("Trying to compare (>) args with non-integer types, but it should be catched earlier");
        }

        match (self, arg) {
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() >= arg.integer())
            },
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

                    let res = self.greater_or_equal(arg);
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
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
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() < arg.integer())
            },
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
            (Self::Integer(_), _) => {
                ExecutableExpressionArgsValues::Boolean(self.integer() <= arg.integer())
            },
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
            Self::String(s) => {
                ExecutableExpressionArgsValues::Integer(s.len() as i64)
            },
            Self::Several(arg1) => {
                println!("{:?}", arg1);
                let mut collected: Vec<ExecutableExpressionArgsValues> = Vec::with_capacity(arg1.len());
                for i in 0..arg1.len() {
                    let iter_arg2 = &arg1[i];

                    let res = iter_arg2.len();
                    collected.push(res);
                }

                ExecutableExpressionArgsValues::Several(collected)
            }
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

                ExecutableExpressionArgsValues::Boolean(re.is_match(str_arg))
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
            _ => {
                unreachable!("In re_match operation found the following case that shoud not exist: {:?}, {:?}", self, arg)
            }
        }
    }
}