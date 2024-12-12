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
    pub fn new(cmd: &str, current_dir: Option<&str>) -> Self {
        let cd = if let Some(d) = current_dir  {
            Some(String::from(d))
        } else {
            None
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

    pub fn execute(&self) -> Result<String, ExecError> {
        let mut cmd = self.command();
        match cmd.output() {
            Ok(o) => {
                if o.status.success() {
                    Ok(format!("{}{}", Self::get_output_string(o.stdout), Self::get_output_string(o.stderr)))
                } else {
                    Err(ExecError::ExitStatus(format!("{}{}", Self::get_output_string(o.stdout), Self::get_output_string(o.stderr)), o.status.code().unwrap_or(0)))
                }
            }
            Err(_) => Err(ExecError::FailedExecuteProcess(format!("{:?}", cmd))),
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::new(self.cmd.as_str());
        if let Some(cd) = self.current_dir.as_ref() {
            command.current_dir(cd.as_str());
        }
        for fn_arg in &self.args {
            fn_arg(&mut command);
        }
        command
    }

    fn get_output_string(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap_or_else(|_| String::from(""))
    }
}
