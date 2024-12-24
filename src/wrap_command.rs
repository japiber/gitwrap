use std::process::Command;
use std::sync::Arc;
use crate::WrapError;

pub struct FnOptionArg(pub Arc<(dyn Fn() -> Vec<String>)>);

impl Clone for FnOptionArg {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}


/// A git command wrapper and manager
#[derive(Clone)]
pub struct WrapCommand {
    cmd: String,
    args: Vec<FnOptionArg>
}

impl WrapCommand {
    /// Creates a new git [cmd] command.
    /// If [current_dir] is Some, the command working directory is set to it.
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: String::from(cmd),
            args: vec![],
        }
    }

    /// Includes a new git command option
    pub fn add_option(&self, arg: FnOptionArg) -> Self {
        let mut v1 = self.args.clone();
        v1.push(arg);
        Self {
            cmd: self.cmd.to_string(),
            args: v1,
        }
    }

    pub fn add_options<I>(&self, args: I) -> Self
    where
        I: IntoIterator<Item = FnOptionArg>
    {
        let mut v1 = self.args.clone();
        let mut v2 = args.into_iter().collect::<Vec<FnOptionArg>>();
        v1.append(&mut v2);
        Self {
            cmd: self.cmd.to_string(),
            args: v1,
        }

    }

    /// Executes the git command as a child process, waiting for it to finish and collecting all of its output.
    /// By default, stdout and stderr are captured (and used to provide the result string).
    pub fn run(&self, current_dir: Option<&str>) -> Result<String, WrapError> {
        let mut cmd = self.command(current_dir);
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
        let cmd = self.command(None);
        let mut con : Vec<String> = Vec::new();
        con.push(String::from(cmd.get_program().to_str().unwrap_or("")));
        for arg in cmd.get_args() {
            con.push(String::from(arg.to_str().unwrap_or("")));
        }
        Ok(con.join(" "))
    }

    fn command(&self, current_dir: Option<&str>) -> Command {
        let mut command = Command::new(self.cmd.as_str());
        if let Some(cd) = current_dir.as_ref() {
            command.current_dir(cd);
        }
        for fn_arg in &self.args {
            let v = fn_arg.0();
            command.args(v);
        }
        command
    }

    fn get_output_string(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap_or_else(|_| String::from(""))
    }
}
