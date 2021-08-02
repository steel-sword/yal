use super::value::Value;

#[derive(Debug)]
pub struct Exception {
    pub thrown_object: Value,
    pub traceback: Vec<Option<(u32, u16)>>,
    pub previous_exception: Option<Box<Exception>>,
}
