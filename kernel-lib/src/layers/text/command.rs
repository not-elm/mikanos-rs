use alloc::rc::Rc;
use alloc::string::{String, ToString};

pub type CommandResult = Result<String, String>;


#[derive(Clone)]
pub struct Command {
    name: String,

    f: Rc<dyn Executable>,
}


impl Command {
    pub fn new(name: &str, execute: impl Fn(&[&str]) -> CommandResult + 'static) -> Self {
        Self {
            name: name.to_string(),
            f: Rc::new(execute),
        }
    }


    pub fn name(&self) -> &str {
        &self.name
    }


    pub fn execute(&self, args: &[&str]) -> CommandResult {
        self.f.execute(args)
    }
}


pub trait Executable {
    fn execute(&self, args: &[&str]) -> CommandResult;
}


impl<F> Executable for F
where
    F: Fn(&[&str]) -> CommandResult,
{
    fn execute(&self, args: &[&str]) -> CommandResult {
        self(args)
    }
}
