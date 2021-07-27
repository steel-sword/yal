use std::rc::Rc;

use crate::types::{value, DotPair, DynType, Value};

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
        .insert(String::from("nil"), value(DynType::Nil));
    global_scope
        .variables
        .insert(String::from("true"), value(DynType::Number(1.0)));
    global_scope
}

fn calculate_call(
    special_forms: Rc<SpecialForms>,
    scope: ScopeRef,
    scope_state: ScopeState,
    value: Value,
) -> Result<Value, String> {
    let pair = value.to_pair()?;
    if let DynType::Symbol(symbol) = &*pair.left {
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
    let pair = rebuilded.to_pair()?;
    if let DynType::Closure(clojure) = &*pair.left {
        (*clojure)(pair.right.clone())
    } else {
        Err(format!("{} is not a function or special form", pair.left))
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

    let right = match &*pair.right {
        DynType::Nil => pair.right.clone(),
        DynType::Pair(pair) => rebuild_list_with_calculation(special_forms, scope, pair)?,
        _ => calculate(
            special_forms,
            scope,
            ScopeState::Expression,
            pair.right.clone(),
        )?,
    };

    Ok(value(DynType::Pair(DotPair { left, right })))
}

pub fn calculate(
    special_forms: Rc<SpecialForms>,
    scope: ScopeRef,
    scope_state: ScopeState,
    value: Value,
) -> Result<Value, String> {
    Ok(match &*value {
        DynType::Pair(_) => calculate_call(special_forms, scope, scope_state, value)?,
        DynType::Symbol(symbol) => scope.borrow().variable(&symbol)?,
        DynType::Quoted(value) => value.clone(),
        _ => value,
    })
}
