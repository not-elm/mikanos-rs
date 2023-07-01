use alloc::rc::Rc;
use alloc::string::{String, ToString};

pub enum CommandAction {
    Clear,
    Output(String),
}


impl CommandAction {
    pub fn output(message: impl Into<String>) -> Self {
        CommandAction::Output(message.into())
    }
}


pub type CommandArgs<'a> = &'a [&'a str];

pub type CommandResult = Result<CommandAction, String>;


#[derive(Clone)]
pub struct Command {
    name: String,

    f: Rc<dyn Executable>,
}


impl Command {
    pub fn new(name: &str, execute: impl Fn(CommandArgs) -> CommandResult + 'static) -> Self {
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
    fn execute(&self, args: CommandArgs) -> CommandResult;
}


impl<F> Executable for F
where
    F: Fn(CommandArgs) -> CommandResult,
{
    fn execute(&self, args: CommandArgs) -> CommandResult {
        self(args)
    }
}
