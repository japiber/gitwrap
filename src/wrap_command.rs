use std::process::Command;
use crate::WrapError;

pub type FnOptionArg = Box<dyn Fn(&mut Command)>;

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

    pub fn execute(&self) -> Result<String, WrapError> {
        let mut cmd = self.command();
        match cmd.output() {
            Ok(o) => {
                if o.status.success() {
                    Ok(format!("{}{}", Self::get_output_string(o.stdout), Self::get_output_string(o.stderr)))
                } else {
                    Err(WrapError::ExitStatus(format!("{}{}", Self::get_output_string(o.stdout), Self::get_output_string(o.stderr)), o.status.code().unwrap_or(0)))
                }
            }
            Err(_) => Err(WrapError::FailedExecuteProcess(format!("{:?}", cmd))),
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
