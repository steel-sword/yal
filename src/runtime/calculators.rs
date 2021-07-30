use std::rc::Rc;

use crate::types::{DotPair, DynType, Value};

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
) -> Result<Value, String> {
    let pair = value.content.to_pair()?;
    if let DynType::Symbol(symbol) = &*pair.left.content {
        if let Some(special_form) = special_forms.clone().get(symbol) {
            return special_form.calculate(
                special_forms,
                scope.clone(),
                scope_state,
                pair.right.clone(),
            );
        }
    }

    let rebuilded = rebuild_list_with_calculation(special_forms, scope, &pair)?;
    let pair = rebuilded.content.to_pair()?;
    if let DynType::Closure(clojure) = &*pair.left.content {
        (*clojure)(pair.right.clone())
    } else {
        Err(format!("{} is not a function or special form", pair.left.content))
    }
}

fn rebuild_list_with_calculation(
    special_forms: Rc<SpecialForms>,
    scope: ScopeRef,
    pair: &DotPair,
) -> Result<Value, String> {
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
) -> Result<Value, String> {
    Ok(match &*given_value.content {
        DynType::Pair(_) => calculate_call(special_forms, scope, scope_state, given_value)?,
        DynType::Symbol(symbol) => scope.borrow().variable(&symbol)?,
        DynType::Quoted(quoted) => match &*(quoted.content) {
            DynType::Pair(pair) => rebuild_list_with_calculation(special_forms, scope, pair)?,
            _ => return Err(format!("Only pair could be quoted"))
        },
        _ => given_value,
    })
}
