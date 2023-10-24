pub(crate) mod traits;
mod args;
pub(super) mod functions;

use nom::{
    self,
    sequence::delimited,
    character::complete::{char, digit1, alpha1},
    bytes::complete::{is_not, is_a},
    combinator::map_res, error::Error, multi::many1,
};

use traits::*;
use args::*;
use functions::*;

use super::AuditError;

// ------------------------------------------------------------------------------------------------------------


fn next_is_string(exp: &str) -> nom::IResult<&str, &str> {
    delimited(char('"'), is_not("\""), char('"'))(exp)
}

fn next_is_operator(exp: &str) -> nom::IResult<&str, &str> {
    is_a("><=~")(exp)
}

fn next_is_integer(exp: &str) -> nom::IResult<&str, usize> {
    map_res(digit1, str::parse)(exp)
}

fn is_func_arg(exp: &str) -> nom::IResult<&str, &str> {
    is_not(", ")(exp)
}

fn get_function(name: &str) -> Result<FunctionType, AuditError> {
    let f = match name {
        "len" => FunctionType::StringLength,
        _ => {
            let err_str = format!("Unknown function name '{}'", name);
            return Err(AuditError(err_str));
        }
    };

    Ok(f)
}

fn next_is_function(exp: &str) -> Result<(&str, Function), AuditError> {
    // let a = take_while(is_alphabetic)(exp)
    //     .and_then(delimited(char('"'), is_not("\""), char('"')));
    let (exp, func_name) = match alpha1::<&str, Error<&str>>(exp) {
        Ok((exp, func_name)) => (exp, func_name),
        Err(_) => {
            let err_str = format!("Expected function name at '{}'", exp);
            return Err(AuditError(err_str));
        }
    };

    let (exp, func_args) = match delimited(char::<&str, Error<&str>>('('), is_not(")"), char(')'))(exp) {
        Ok((exp, func_args)) => (exp, func_args),
        Err(_) => {
            let err_str = format!("Expected list of arguments between \"(\" and \")\" at {}", exp);
            return Err(AuditError(err_str))
        }
    };

    let (_, arg1) = match many1(is_func_arg)(func_args) {
        Ok((exp, arg1)) => (exp, arg1),
        Err(_) => {
            ("", vec![func_args])
        }
    };

    let function_type = get_function(func_name)?;
    let mut function = Function::default();
    function.set_function(function_type)?;

    for sub_exp in arg1 {
        if let Ok((reminder, parsed_str)) = next_is_string(sub_exp) {
            if reminder.len() > 0 {
                todo!()
            }

            function.add_arg(
                FunctionArg::STRING(parsed_str.to_string()).into_generic()
            )?;
        }
        else if let Ok((reminder, integer)) = next_is_integer(sub_exp) {
            if reminder.len() > 0 {
                todo!()
            }

            function.add_arg(
                FunctionArg::INTEGER(integer).into_generic()
            )?;
        }
        else if let Ok((reminder, sub_fn)) = next_is_function(sub_exp) {
            if reminder.len() > 0 {
                todo!()
            }

            function.add_arg(GenericArg::Function(sub_fn))?;
        }
        else {
            let err_str = format!("Could not parse as a function argument: '{}'", sub_exp);
            return Err(AuditError(err_str));
        }
    }

    return Ok((exp, function));
}

fn parse(exp: &str) -> Result<Function, AuditError> {
    // let mut result = Vec::default();
    let mut function = Function::default();
    let mut reminder = exp;

    while reminder.len() > 0 {
        reminder = if let Ok((reminder, parsed_str)) = next_is_string(reminder) {
            function.add_arg( FunctionArg::STRING(parsed_str.to_string()).into_generic() )?;
            reminder
        }
        else if let Ok((reminder, parsed_operator)) = next_is_operator(reminder) {
            match parsed_operator {
                ">" => {
                    let f = FunctionType::CompareInteger(CompareIntegerFunction::Greater);
                    function.set_function(f)?;
                    reminder
                },
                "<" => {
                    let f = FunctionType::CompareInteger(CompareIntegerFunction::Lower);
                    function.set_function(f)?;
                    reminder
                },
                "=" => {
                    let f = FunctionType::CompareInteger(CompareIntegerFunction::Equal);
                    function.set_function(f)?;
                    reminder
                },
                ">=" => {
                    let f = FunctionType::CompareInteger(CompareIntegerFunction::GreaterOrEqual);
                    function.set_function(f)?;
                    reminder
                },
                "<=" => {
                    let f = FunctionType::CompareInteger(CompareIntegerFunction::LowerOrEqual);
                    function.set_function(f)?;
                    reminder
                },
                "~" => {
                    function.set_function(FunctionType::MatchString)?;
                    reminder
                }
                _ => {
                    unreachable!()
                }
            }
        }
        else if let Ok((reminder, parsed_int)) = next_is_integer(reminder) {
            function.add_arg(FunctionArg::INTEGER(parsed_int).into_generic())?;
            reminder
        }
        else if let Ok((reminder, func)) = next_is_function(reminder) {
            if func.is_boolean() {
                function = func;
            }
            else {
                function.add_arg(GenericArg::Function(func))?;
            }
            reminder
        }
        else {
            todo!("{}", reminder)
        }
    }

    // TODO: Check expression is not undefined

    return Ok(function);
}

pub(crate) fn parse_expression(exp: &str) -> Result<Function, AuditError> {
    parse(exp)
}
