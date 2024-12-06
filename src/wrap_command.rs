use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::process::Command;

pub type FnOptionArg = Box<dyn Fn(&mut Command)>;


pub enum ExecError {
    FailedExecuteProcess(String),
    ExitStatus(String, i32),
}

impl Error for ExecError {}

impl ExecError {
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecError::FailedExecuteProcess(s) => write!(f, "failed to execute process: {}", s),
            ExecError::ExitStatus(o, x) => write!(f, "exit status: {}: {}", x, o),
        }
    }
}

impl Display for ExecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl Debug for ExecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

pub struct WrapCommand {
    cmd: String,
    args: Vec<FnOptionArg>,
    current_dir: Option<String>,
}

impl WrapCommand {
    pub fn new(cmd: &str, current_dir: &str) -> Self {
        let cd = if current_dir.is_empty() {
            None
        } else {
            Some(String::from(current_dir))
        };
        Self {
            cmd: String::from(cmd),
            args: Vec::new(),
            current_dir: cd,
        }
    }

    pub fn option(&mut self, arg: FnOptionArg) {
        self.args.push(arg);
    }

    pub fn execute (&self) -> Result<String, ExecError> {
        let mut command = Command::new(self.cmd.as_str());
        for arg in &self.args {
            arg(&mut command);
        }
        if let Some(current_dir) = &self.current_dir {
            command.current_dir(current_dir.as_str());
        }
        match command.output() {
            Ok(o) => {
                if o.status.success() {
                    Ok(format!("{}{}", Self::get_output_string(o.stdout), Self::get_output_string(o.stderr)))
                } else {
                    Err(ExecError::ExitStatus(format!("{}{}", Self::get_output_string(o.stdout), Self::get_output_string(o.stderr)), o.status.code().unwrap_or(0)))
                }
            }
            Err(_) => Err(ExecError::FailedExecuteProcess(format!("{:?}", command))),
        }
    }

    fn get_output_string(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap_or_else(|_| String::from(""))
    }
}

pub fn git(current_dir: &str) -> WrapCommand {
    WrapCommand::new("git", current_dir)
}
