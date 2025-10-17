use std::cell::RefCell;

thread_local! {
    pub(crate) static PREFIX: RefCell<Vec<&'static str>> = RefCell::new(Vec::new());
}

pub struct StackTrace;

impl StackTrace {
    pub fn print_current() {
        PREFIX.with(|stk| {
            let stk = stk.borrow();
            for frame in stk.iter() {
                ::std::print!("[{}] ", frame);
            }
        })
    }
}

impl From<&'static str> for StackTrace{
    fn from(value: &'static str) -> Self {
        PREFIX.with(|stk| stk.borrow_mut().push(value));
        StackTrace
    }
}

impl Drop for StackTrace {
    fn drop(&mut self) {
        PREFIX.with(|stk| stk.borrow_mut().pop());
    }
}