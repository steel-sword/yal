use std::{collections::HashMap, rc::Rc};

use crate::{runtime::{custom_function::CustomFunction, scope::{ScopeRef, ScopeState}}, types::{DynType, List, ListItem, StructType, Value, value}};

use super::calculators::calculate;

pub type SpecialForms = HashMap<String, SpecialForm>;

pub struct SpecialForm {
    pub name: &'static str,
    calculator: Rc<dyn Fn(Rc<SpecialForms>, ScopeRef, Value) -> Result<Value, String>>,
    possible_scope_state: ScopeState,
}

impl SpecialForm {
    pub fn calculate(
        &self,
        special_forms: Rc<SpecialForms>,
        scope: ScopeRef,
        scope_state: ScopeState,
        args: Value,
    ) -> Result<Value, String> {
        if scope_state > self.possible_scope_state {
            Err(format!(
                "{} special form is allowed for {:?} scope but {:?} scope is given",
                self.name, self.possible_scope_state, scope_state,
            ))
        } else {
            (self.calculator)(special_forms, scope, args)
        }
    }
}

fn let_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let name = list.next().to_middle()?.to_symbol()?;
    let value_expr = list.next().to_middle()?;
    list.next().to_end()?;
    let new_var = calculate(
        special_forms,
        scope.clone(),
        ScopeState::Expression,
        value_expr,
    )?;
    (&mut *scope.borrow_mut()).define_variable(name, new_var)?;
    scope.borrow().variable(&String::from("nil"))
}

fn def_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String>{
    let mut list = List::new(args);

    let mut arguments = List::new(list.next().to_middle()?);
    let name = arguments.next().to_middle()?.to_symbol()?;
    let mut body = List::new(list.next().to_middle()?);
    list.next().to_end()?;

    let mut statements = vec![];
    while let ListItem::Middle(value) = body.next() {
        statements.push(value);
    }
    body.next().to_end()?;

    let function = CustomFunction::new(statements, scope.clone(), arguments.current_value);
    
    scope
        .borrow_mut()
        .define_variable(
            name,
            value(DynType::Closure(
                Rc::new(move |args| { function.call(special_forms.clone(), args) })
            ))
        )?;

    Ok(value(DynType::Nil))
}

fn lambda_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String>{
    let mut list = List::new(args);

    let arguments = list.next().to_middle()?;
    let mut body = List::new(list.next().to_middle()?);
    list.next().to_end()?;

    let mut statements = vec![];
    while let ListItem::Middle(value) = body.next() {
        statements.push(value);
    }
    body.next().to_end()?;

    let function = CustomFunction::new(statements, scope.clone(), arguments);

    Ok(value(DynType::Closure(
        Rc::new(move |args| { function.call(special_forms.clone(), args) })
    )))
}

fn struct_form(_: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let name = list.next().to_middle()?.to_symbol()?;
    let mut fields_list = List::new(list.next().to_middle()?);
    list.next().to_end()?;

    let mut fields = vec![];
    while let ListItem::Middle(field) = fields_list.next() {
        fields.push(field.to_symbol()?)
    }
    scope
        .borrow_mut()
        .define_variable(
            name.clone(),
            value(DynType::StructDeclare(Rc::new(StructType { name, fields })))
        )?;
    Ok(value(DynType::Nil))
}

fn get_field_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let value_expr = list.next().to_middle()?;
    let required_field = list.next().to_middle()?.to_symbol()?;
    list.next().to_end()?;

    let new_var = calculate(
        special_forms,
        scope.clone(),
        ScopeState::Expression,
        value_expr,
    )?;

    new_var.to_struct()?.get_field(required_field)
}

fn if_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    let condition = list.next().to_middle()?;
    let main_body = list.next().to_middle()?;
    let else_body = list.next().to_middle()?;
    list.next().to_end()?;
    if let DynType::Nil = &*calculate(
        special_forms.clone(),
        scope.clone(),
        ScopeState::Expression,
        condition,
    )? {
        calculate(special_forms, scope, ScopeState::Expression, else_body)
    } else {
        calculate(special_forms, scope, ScopeState::Expression, main_body)
    }
}

fn and_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    while let ListItem::Middle(parameter) = list.next() {
        if let DynType::Nil = &*calculate(
            special_forms.clone(),
            scope.clone(),
            ScopeState::Expression,
            parameter,
        )? {
            return Ok(value(DynType::Nil));
        }
    }
    Ok(value(DynType::Number(1.0)))
}

fn or_form(special_forms: Rc<SpecialForms>, scope: ScopeRef, args: Value) -> Result<Value, String> {
    let mut list = List::new(args);
    while let ListItem::Middle(parameter) = list.next() {
        if let DynType::Nil = &*calculate(
            special_forms.clone(),
            scope.clone(),
            ScopeState::Expression,
            parameter,
        )? {
        } else {
            return Ok(value(DynType::Number(1.0)));
        }
    }
    Ok(value(DynType::Nil))
}

pub fn all_special_forms() -> Rc<SpecialForms> {
    let mut special_forms = HashMap::new();

    let let_form_name = "let";
    special_forms.insert(
        let_form_name.to_string(),
        SpecialForm {
            name: let_form_name,
            calculator: Rc::new(|special_forms, scope, args| let_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Local,
        },
    );

    let def_form_name = "def";
    special_forms.insert(
        def_form_name.to_string(),
        SpecialForm {
            name: def_form_name,
            calculator: Rc::new(|special_forms, scope, args| def_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Local,
        },
    );

    let lambda_form_name = "lambda";
    special_forms.insert(
        lambda_form_name.to_string(),
        SpecialForm {
            name: lambda_form_name,
            calculator: Rc::new(|special_forms, scope, args| lambda_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Expression,
        },
    );

    let struct_form_name = "struct";
    special_forms.insert(
        struct_form_name.to_string(),
        SpecialForm {
            name: struct_form_name,
            calculator: Rc::new(|special_forms, scope, args| struct_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Local,
        },
    );

    let if_form_name = "if";
    special_forms.insert(
        if_form_name.to_string(),
        SpecialForm {
            name: if_form_name,
            calculator: Rc::new(|special_forms, scope, args| if_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Expression,
        },
    );

    let and_form_name = "and";
    special_forms.insert(
        and_form_name.to_string(),
        SpecialForm {
            name: and_form_name,
            calculator: Rc::new(|special_forms, scope, args| and_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Expression,
        },
    );

    let or_form_name = "or";
    special_forms.insert(
        or_form_name.to_string(),
        SpecialForm {
            name: or_form_name,
            calculator: Rc::new(|special_forms, scope, args| or_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Expression,
        },
    );

    let get_field_form_name = "::";
    special_forms.insert(
        get_field_form_name.to_string(),
        SpecialForm {
            name: get_field_form_name,
            calculator: Rc::new(|special_forms, scope, args| get_field_form(special_forms, scope, args)),
            possible_scope_state: ScopeState::Expression,
        },
    );

    Rc::new(special_forms)
}
