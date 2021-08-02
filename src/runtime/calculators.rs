use std::{rc::Rc, vec};

use crate::types::{DynType, dot_pair::DotPair, exception::Exception, value::Value};

use super::{
    functions::all_base_functions,
    scope::{Scope, ScopeRef, ScopeState},
    special_forms::SpecialForms,
};

pub fn create_global_scope() -> Scope {
    let mut global_scope = Scope::new(None);
    global_scope.variables.extend(all_base_functions());
    global_scope
        .variables
        .insert(String::from("nil"), Value::new(DynType::Nil, None));
    global_scope
        .variables
        .insert(String::from("true"), Value::new(DynType::Number(1.0), None));
    global_scope
}

fn calculate_call(
    special_forms: Rc<SpecialForms>,
    scope: ScopeRef,
    scope_state: ScopeState,
    value: Value,
) -> Result<Value, Exception> {
    let pair = value.content.to_pair()?;
    if let DynType::Symbol(symbol) = &*pair.left.content {
        if let Some(special_form) = special_forms.clone().get(symbol) {
            return special_form.calculate(
                special_forms,
                scope.clone(),
                scope_state,
                pair.right.clone(),
                value.position,
            );
        }
    }

    let rebuilded = rebuild_list_with_calculation(special_forms, scope, &pair)?;
    let pair = rebuilded.content.to_pair()?;
    if let DynType::Closure(clojure) = &*pair.left.content {
        match (*clojure)(pair.right.clone()) {
            Ok(ok) => Ok(ok),
            Err(mut err) => {
                err.traceback.push(value.position);
                Err(err)
            },
        }
    } else {
        Err(Exception {
            thrown_object: Value::new(
                DynType::Str(format!("{} is not a function or special form", pair.left.content)), None
            ),
            traceback: vec![value.position],
            previous_exception: None,
        })
    }
}

fn rebuild_list_with_calculation(
    special_forms: Rc<SpecialForms>,
    scope: ScopeRef,
    pair: &DotPair,
) -> Result<Value, Exception> {
    let left = calculate(
        special_forms.clone(),
        scope.clone(),
        ScopeState::Expression,
        pair.left.clone(),
    )?;

    let right = match &*pair.right.content {
        DynType::Nil => pair.right.clone(),
        DynType::Pair(pair) => rebuild_list_with_calculation(special_forms, scope, &pair)?,
        _ => calculate(
            special_forms,
            scope,
            ScopeState::Expression,
            pair.right.clone(),
        )?,
    };

    Ok(Value::new(DynType::Pair(DotPair { left, right }), None))
}

pub fn calculate(
    special_forms: Rc<SpecialForms>,
    scope: ScopeRef,
    scope_state: ScopeState,
    given_value: Value,
) -> Result<Value, Exception> {
    Ok(match &*given_value.content {
        DynType::Pair(_) => calculate_call(special_forms, scope, scope_state, given_value)?,
        DynType::Symbol(symbol) => match scope.borrow().variable(&symbol) {
            Ok(variable) => variable,
            Err(mut err) => {
                err.traceback.push(given_value.position);
                return Err(err);
            }
        },
        DynType::Quoted(quoted) => match &*(quoted.content) {
            DynType::Pair(pair) => rebuild_list_with_calculation(special_forms, scope, pair)?,
            _ => return Err(Exception {
                thrown_object: Value::new(DynType::Str(format!("Only pair could be quoted")), None),
                traceback: vec![given_value.position],
                previous_exception: None,
            })
        },
        _ => given_value,
    })
}
