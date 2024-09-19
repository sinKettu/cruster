use crate::audit::actions::find::args::{ExecutableExpressionArgsTypes, ExecutableExpressionArgsValues as Avalue};

use super::{process_both_args_several, process_both_args_wref, process_left_arg_several, process_left_arg_wref, process_right_arg_several, process_right_arg_wref, Operations};

pub(super) fn exec(left: &Avalue, right: &Avalue) -> Avalue {
    if ! (left.get_type() == ExecutableExpressionArgsTypes::BOOLEAN && left.get_type() == right.get_type()) {
        unreachable!("Trying to compare args with diffrerent types, but it should be catched earlier");
    }

    let f = |left: &Avalue, right: &Avalue| -> Avalue {
        left.and(right)
    };

    match (left, right) {
        (Avalue::Several(arg1), Avalue::Several(arg2)) => {
            process_both_args_several(arg1, arg2, f)
        },
        (Avalue::Several(arg1), _) => {
            process_left_arg_several(arg1, right, f)
        },
        (_, Avalue::Several(arg2)) => {
            process_right_arg_several(left, arg2, f)
        },
        (Avalue::WithSendResReference(arg1_with_ref), Avalue::WithSendResReference(arg2_with_ref)) => {
            process_both_args_wref(arg1_with_ref, arg2_with_ref, f)
        },
        (_, Avalue::WithSendResReference(arg2_with_ref)) => {
            process_right_arg_wref(left, arg2_with_ref, f)
        },
        (Avalue::WithSendResReference(arg1_with_ref), _) => {
            process_left_arg_wref(arg1_with_ref, right, f)
        },
        (Avalue::Boolean(_), _) => {
            Avalue::Boolean(left.boolean() && right.boolean())
        },
        _ => {
            unreachable!("In 'equal' method of Operations found the case: {:?}, {:?}, but it should not exists", left, right)
        }
    }
}
