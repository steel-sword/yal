use std::{cell::RefCell, rc::Rc};

use crate::types::{List, ListItem, Value};

use super::{calculators::calculate, scope::{Scope, ScopeRef}, special_forms::SpecialForms};

pub struct CustomFunction {
    name: String,
    statements: Vec<Value>,
    outer_scope: ScopeRef,
    arg_symbols: Value,
}

impl CustomFunction {
    pub fn new(name: String, statements: Vec<Value>, outer_scope: ScopeRef, arg_symbols: Value) -> Self {
        Self { name, statements, outer_scope, arg_symbols }
    }

    pub fn call(&self, special_forms: Rc<SpecialForms>, args: Value) -> Result<Value, String> {
        let scope = Rc::new(RefCell::new(
            Scope::new(Some(self.outer_scope.clone()))
        ));
        self.define_parameters_in_scope(scope.clone(), args)?;
        let mut last = Err(format!("Empty body"));
        for statement in self.statements.iter() {
            last = Ok(calculate(special_forms.clone(), scope.clone(), super::scope::ScopeState::Local, statement.clone())?);
        }
        last
    }

    fn define_parameters_in_scope(&self, scope: ScopeRef, args: Value) -> Result<(), String> {
        let mut defined_args = List::new(self.arg_symbols.clone());
        let mut got_args = List::new(args);

        while let ListItem::Middle(value) = defined_args.next() {
            let next = got_args.next().to_middle()?;
            scope.borrow_mut().define_variable(value.to_symbol()?, next)?;
        }
        match (defined_args.next(), got_args.next()) {
            (ListItem::Last(last), ListItem::Middle(got))
            | (ListItem::Last(last), ListItem::Last(got)) => {
                scope.borrow_mut().define_variable(last.to_symbol()?, got)?;
            }
            (ListItem::End, ListItem::End) => {},
            _ => return Err(format!("Arguments count error, given more or less"))
        }

        Ok(())
    }
}
