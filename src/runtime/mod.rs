mod calculators;
mod custom_function;
pub mod functions;
mod scope;
pub mod special_forms;

use crate::types::exception::Exception;
use crate::types::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

use self::calculators::calculate;
use self::calculators::create_global_scope;
use self::scope::ScopeState;
use self::special_forms::all_special_forms;

pub fn execute(values: &mut dyn Iterator<Item = Value>) -> Result<(), Exception> {
    let global_scope = Rc::new(RefCell::new(create_global_scope()));
    let special_forms = all_special_forms();
    for value in values {
        calculate(
            special_forms.clone(),
            global_scope.clone(),
            ScopeState::Global,
            value,
        )?;
    }

    Ok(())
}
