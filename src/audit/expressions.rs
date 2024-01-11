pub(crate) mod traits;
mod args;
pub(super) mod functions;
mod executions;

use std::str::FromStr;

use nom::{
    self,
    sequence::{delimited, tuple, pair},
    character::complete::{char, digit1, space0, space1},
    bytes::complete::{is_not, is_a, tag},
    IResult, multi::many0,
};

use args::*;
use functions::*;

use super::AuditError;

fn is_negotiation(reminder: &str) -> IResult<&str, FunctionType> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, _) = char('!')(reminder)?;
    let (reminder, _) = space0(reminder)?;

    return Ok((reminder, FunctionType::Negotiation));
}

fn is_operator(reminder: &str) -> IResult<&str, FunctionType> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, operator) = is_a("><=~")(reminder)?;
    let (reminder, _) = space0(reminder)?;

    let operator = match operator {
        ">" => {
            FunctionType::CompareInteger(CompareIntegerFunction::Greater)
        },
        "<" => {
            FunctionType::CompareInteger(CompareIntegerFunction::Lower)
        },
        "=" => {
            FunctionType::CompareInteger(CompareIntegerFunction::Equal)
        },
        "~" => {
            FunctionType::MatchString
        },
        ">=" => {
            FunctionType::CompareInteger(CompareIntegerFunction::GreaterOrEqual)
        },
        "<=" => {
            FunctionType::CompareInteger(CompareIntegerFunction::LowerOrEqual)
        },
        _ => {
            return Err(nom::Err::Failure(nom::error::Error::new(operator, nom::error::ErrorKind::NoneOf)));
        }
    };

    return Ok((reminder, operator));
}

fn is_string(reminder: &str) -> IResult<&str, FunctionArg> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, parsed_string) = delimited(tag("\""), is_not("\""), tag("\""))(reminder)?;

    Ok((reminder, FunctionArg::STRING(parsed_string.to_string())))
}

fn is_integer(reminder: &str) -> IResult<&str, FunctionArg> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, str_integer) = digit1(reminder)?;
    let farg = FunctionArg::INTEGER(str_integer.parse::<usize>().unwrap());

    return Ok((reminder, farg));
}

fn is_function_1(reminder: &str) -> IResult<&str, FunctionType> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, fname) = is_a("abcdefghijklmnopqrstuvwxyz0123456789_")(reminder)?;
    let (reminder, _) = tag("(")(reminder)?;

    match fname {
        "len" => {
            return Ok((reminder, FunctionType::StringLength));
        },
        _ => {
            return Err(
                nom::Err::Failure(
                    nom::error::Error::new(fname, nom::error::ErrorKind::Fail)
                )
            );
        }
    };
}

fn is_args_delimiter(reminder: &str) -> IResult<&str, char> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, comma) = char(',')(reminder)?;
    
    Ok((reminder, comma))
}

fn is_end_of_function(reminder: &str) -> IResult<&str, char> {
    let (reminder, _) = space0(reminder)?;
    let (reminder, bracket) = char(')')(reminder)?;

    Ok((reminder, bracket))
}

fn is_whitespace(reminder: &str) -> IResult<&str, &str> {
    Ok(space1(reminder)?)
}

fn is_reference<'a>(initial_reminder: &'a str) -> IResult<&'a str, Reference> { 
    let (reminder, _) = space0(initial_reminder)?;

    let mut parser = tuple(
        (
            is_a("abcdefghijklmnopqrstuvwxyz0123456789_"),
            pair(char('.'), is_a("abcdefghijklmnopqrstuvwxyz0123456789_")),
            many0(
                pair(char('.'), is_a("abcdefghijklmnopqrstuvwxyz0123456789_"))
            )
        )
    );

    let (reminder, (send_id, (dot, pair_part), message_parts)) = parser(reminder)?;
    let pair_part_parsed = match pair_part {
        "request" => {
            PairPart::REQUEST
        },
        "response" => {
            PairPart::RESPONSE
        },
        _ => {
            return Err(
                nom::Err::Failure(
                    nom::error::Error::new(initial_reminder, nom::error::ErrorKind::Fail)
                )
            );
        }
    };
    
    let message_part = if message_parts.len() == 1 {
        let str_message_part = message_parts[0].1;
        match pair_part_parsed {
            PairPart::REQUEST => {
                match str_message_part {
                    "method" => { MessagePart::METHOD },
                    "path" => { MessagePart::PATH },
                    "version" => { MessagePart::VERSION },
                    "body" => { MessagePart::BODY },
                    _ => {
                        return Err(
                            nom::Err::Failure(
                                nom::error::Error::new(initial_reminder, nom::error::ErrorKind::Fail)
                            )
                        );
                    }
                }
            },
            PairPart::RESPONSE => {
                match str_message_part {
                    "status" => { MessagePart::STATUS },
                    "version" => { MessagePart::VERSION },
                    "body" => { MessagePart::BODY },
                    _ => {
                        return Err(
                            nom::Err::Failure(
                                nom::error::Error::new(initial_reminder, nom::error::ErrorKind::Fail)
                            )
                        );
                    }
                }
            }
        }
    }
    else if message_parts.len() == 2 {
        let message_part = message_parts[0].1;
        if message_part != "headers" {
            return Err(
                nom::Err::Failure(
                    nom::error::Error::new(initial_reminder, nom::error::ErrorKind::Fail)
                )
            );
        }

        let header_name = message_parts[1].1;
        MessagePart::HEADER(header_name.to_string())
    }
    else {
        return Err(
            nom::Err::Failure(
                nom::error::Error::new(initial_reminder, nom::error::ErrorKind::Fail)
            )
        );
    };

    let reference = Reference {
        id: send_id.to_string(),
        pair_part: pair_part_parsed,
        message_part
    };

    return Ok((reminder, reference));
}

fn parse_1(exp: &str) -> Result<Function, AuditError> {
    // Priority: !, <>=~, functions()
    // Unary and binary operators, any amount of arguments in functions

    let mut reminder = exp;
    let mut functions_stack: Vec<Function> = Vec::with_capacity(10);
    let mut depth: usize = 0;

    let add_arg = |depth: usize, stack: &mut Vec<Function>, arg: GenericArg| -> Result<(), AuditError> {
        if depth == 0 {
            return Err(
                AuditError(
                    format!("You have tried to use argument outside any function: '{:?}'", arg)
                )
            );
        }
        else {
            stack[depth - 1].add_arg(arg)?;
            return Ok(());
        }
    };

    while reminder.len() > 0 {

        reminder = if let Ok((reminder, f)) = is_negotiation(reminder) {
            let mut func = Function::default();
            func.set_function(f)?;
            functions_stack.push(func);
            depth += 1;

            reminder

        }
        else if let Ok((reminder, f)) = is_operator(reminder) {
            let mut func = Function::default();
            func.set_function(f)?;

            if functions_stack.len() > 0 && functions_stack[depth - 1].priority < func.priority {
                let func_arg = functions_stack.pop().unwrap();
                func.add_arg(GenericArg::Function(func_arg))?;
                depth -= 1;
            }

            functions_stack.push(func);
            depth += 1;

            reminder

        }
        else if let Ok((reminder, arg)) = is_string(reminder) {
            add_arg(depth, &mut functions_stack, arg.into_generic())?;

            reminder

        }
        else if let Ok((reminder, arg)) = is_reference(reminder) {
            add_arg(depth, &mut functions_stack, FunctionArg::REF(arg).into_generic());

            reminder
        }
        else if let Ok((reminder, arg)) = is_integer(reminder) {
            add_arg(depth, &mut functions_stack, arg.into_generic())?;

            reminder

        }
        else if let Ok((reminder, f)) = is_function_1(reminder) {
            let mut func = Function::default();
            func.set_function(f)?;
            functions_stack.push(func);
            depth += 1;

            reminder

        }
        else if let Ok((reminder, _)) = is_args_delimiter(reminder) {
            reminder

        }
        else if let Ok((reminder, _)) = is_end_of_function(reminder) {
            if functions_stack.len() == 0 {
                return Err(AuditError::from_str("Found function ending without function itself").unwrap());
            }
            else if functions_stack.len() > 1 {
                if functions_stack[depth - 1].priority == functions_stack[depth - 2].priority {
                    let f = functions_stack.pop().unwrap();
                    let arg = GenericArg::Function(f);
                    add_arg(depth, &mut functions_stack, arg)?;
                    depth -= 1;
                }
            }

            reminder
            
        }
        else if let Ok((reminder, _)) = is_whitespace(reminder) {
            reminder

        }
        else {
            let err_str = format!("Could not parse expression from {}; check syntax.", exp);
            return Err(AuditError(err_str));
        };

    }

    let mut index = if functions_stack.len() > 0 {
        functions_stack.len() - 1
    }
    else {
        let err_str = format!("Could not parse expression '{}', seems like it is empty.", exp);
        return Err(AuditError(err_str));
    };

    let mut result = Function::default();
    while ! functions_stack.is_empty() {
        if index == 0 && functions_stack.len() == 1 {
            result = functions_stack.pop().unwrap();
        }
        else if index == 0 && functions_stack.len() > 1 {
            let err_str = format!("Could not parse the expression '{}', several functions/operators are used sequentually without connection", exp);
            return Err(AuditError(err_str));
        }
        else {
            while functions_stack[index].priority == functions_stack[index - 1].priority {
                if index == 0 {
                    let err_str = format!("Could not parse the expression '{}', several functions/operators are used sequentually without connection", exp);
                    return Err(AuditError(err_str));
                }

                index -= 1;                
            }

            let higher_priority_func_index = index - 1;
            while higher_priority_func_index != functions_stack.len() - 1 {
                let func = functions_stack.remove(index);
                functions_stack[higher_priority_func_index].add_arg(GenericArg::Function(func))?;
            }

            index = higher_priority_func_index;
        }
    }

    return Ok(result)
}

pub(crate) fn parse_expression(exp: &str) -> Result<Function, AuditError> {
    parse_1(exp)
}
