use std::process::Command;

pub enum ExecError {
    InvalidOutput,
    FailedExecuteProcess,
    ExitStatus(i32),
}

pub type ExecResult = Result<String, ExecError>;

pub type CommandOption<'a> = Box<dyn Fn(&mut CommandExecutor) + 'a>;

pub type Executor = fn(&str, Vec<String>) -> ExecResult;


pub struct CommandExecutor {
    base: String,
    options: Vec<String>,
    executor: Executor,
}

pub fn command<'a, I>(base: &str, cmd: &str, options: I) -> CommandExecutor
where
    I:  IntoIterator<Item=CommandOption<'a>>,
{
    let mut g = CommandExecutor {
        base: base.to_string(),
        options: vec![cmd.to_string()],
        executor: default_executor,
    };
    g.apply_options(options);
    g
}


pub fn default_executor(name: &str, args: Vec<String>) -> ExecResult {
    match Command::new(name)
        .args(args)
        .output() {
        Ok(o) => {
            if o.status.success() {
                match String::from_utf8(o.stdout) {
                    Ok(s) => Ok(s),
                    Err(_) => Err(ExecError::InvalidOutput)
                }
            } else {
                Err(ExecError::ExitStatus(o.status.code().unwrap()))
            }
        }
        Err(_) => Err(ExecError::FailedExecuteProcess)
    }
}


impl CommandExecutor {
    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string())
    }

    pub fn add_option_string(&mut self, option: String) {
        self.options.push(option)
    }

    pub fn apply_options<'a, I>(&mut self, options: I)
    where
        I: IntoIterator<Item=CommandOption<'a>>,
    {
        for opt in options {
            opt(self)
        }
    }

    pub fn exec(&self) -> ExecResult {
        (self.executor)(self.base.as_str(), self.options.clone())
    }
}
