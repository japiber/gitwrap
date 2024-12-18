use std::process::Command;
use crate::WrapError;

pub type FnOptionArg = Box<dyn Fn(&mut Command)>;

/// A git command wrapper and manager
pub struct WrapCommand {
    cmd: String,
    args: Vec<FnOptionArg>,
    current_dir: Option<String>,
}

impl WrapCommand {
    /// Creates a new git [cmd] command.
    /// If [current_dir] is Some, the command working directory is set to it.
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

    /// Includes a new git command option
    pub fn option(&mut self, arg: FnOptionArg) {
        self.args.push(arg);
    }

    /// Executes the git command as a child process, waiting for it to finish and collecting all of its output.
    /// By default, stdout and stderr are captured (and used to provide the result string).
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

    /// Dry-runs the git command returning the full command specification provided
    pub fn dry_run(&self) -> Result<String, WrapError> {
        let cmd = self.command();
        let mut con : Vec<String> = Vec::new();
        con.push(String::from(cmd.get_program().to_str().unwrap_or("")));
        for arg in cmd.get_args() {
            con.push(String::from(arg.to_str().unwrap_or("")));
        }
        Ok(con.join(" "))
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
