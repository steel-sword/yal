use std::{cell::RefCell, rc::Rc};

use crate::types::{DynType, dot_pair::DotPair, exception::Exception, list::{List, ListItem}, value::Value};

use super::{calculators::calculate, scope::{Scope, ScopeRef}, special_forms::SpecialForms};

pub struct CustomFunction {
    statements: Vec<Value>,
    outer_scope: ScopeRef,
    arg_symbols: Value,
}

impl CustomFunction {
    pub fn new(statements: Vec<Value>, outer_scope: ScopeRef, arg_symbols: Value) -> Self {
        Self { statements, outer_scope, arg_symbols }
    }

    pub fn call(&self, special_forms: Rc<SpecialForms>, args: Value) -> Result<Value, Exception> {
        let scope = Rc::new(RefCell::new(
            Scope::new(Some(self.outer_scope.clone()))
        ));
        self.define_parameters_in_scope(scope.clone(), args)?;
        let mut last = Err(Exception {
            thrown_object: Value::new(DynType::Str(format!("Empty body")), None),
            traceback: vec![],
            previous_exception: None
        });
        for statement in self.statements.iter() {
            let value = calculate(special_forms.clone(), scope.clone(), super::scope::ScopeState::Local, statement.clone());
            if let Err(err) = value {
                return Err(err);
            }
            last = value;
        }
        last
    }

    fn define_parameters_in_scope(&self, scope: ScopeRef, args: Value) -> Result<(), Exception> {
        let mut defined_args = List::new(self.arg_symbols.clone());
        let mut got_args = List::new(args);
        let prepared_exception = Err(Exception {
            thrown_object: Value::new(DynType::Str(format!("Arguments count error, given more or less")), None),
            traceback: vec![],
            previous_exception: None
        });

        while let ListItem::Middle(value) = defined_args.next() {
            let next = match got_args.next().to_middle() {
                Ok(v) => v,
                Err(_) => return prepared_exception,
            };
            scope.borrow_mut().define_variable(value.content.to_symbol()?, next)?;
        }
        match (defined_args.next(), got_args.next()) {
            (ListItem::Last(last), ListItem::Middle(got))
            | (ListItem::Last(last), ListItem::Last(got)) => {
                scope.borrow_mut().define_variable(
                    last.content.to_symbol()?,
                   Value::new(DynType::Pair(DotPair { left: got, right: got_args.current_value }), None)
                )?;
            }
            (ListItem::End, ListItem::End) => {},
            _ => return prepared_exception
        }

        Ok(())
    }
}
