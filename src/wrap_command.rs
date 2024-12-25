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
    options: Vec<FnOptionArg>,
    current_dir: Option<String>
}

impl WrapCommand {
    /// Creates a new git [cmd] command.
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: String::from(cmd),
            options: vec![],
            current_dir: None
        }
    }

    /// Set this WrapCommand current_dir to [dir]
    /// When [current_dir] is set, the command working directory is set to it at run.
    pub fn current_dir(&self, dir: &str) -> Self {
        Self {
            cmd: self.cmd.to_string(),
            options: self.options.clone(),
            current_dir: Some(dir.to_string())
        }
    }

    /// Includes a new git command option
    pub fn add_option(&self, arg: FnOptionArg) -> Self {
        let mut v1 = self.options.clone();
        v1.push(arg);
        Self {
            cmd: self.cmd.to_string(),
            options: v1,
            current_dir: self.current_dir.clone()
        }
    }

    /// Includes several new git command options at once
    pub fn add_options<I>(&self, args: I) -> Self
    where
        I: IntoIterator<Item = FnOptionArg>
    {
        let mut v1 = self.options.clone();
        let mut v2 = args.into_iter().collect::<Vec<FnOptionArg>>();
        v1.append(&mut v2);
        Self {
            cmd: self.cmd.to_string(),
            options: v1,
            current_dir: self.current_dir.clone()
        }

    }

    /// Executes the git command as a child process, waiting for it to finish and collecting all of its output.
    /// By default, stdout and stderr are captured (and used to provide the result string).
    pub fn run(&self) -> Result<String, WrapError> {
        let mut cmd = self.prepare();
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
        let cmd = self.prepare();
        let mut con : Vec<String> = Vec::new();
        con.push(String::from(cmd.get_program().to_str().unwrap_or("")));
        for arg in cmd.get_args() {
            con.push(String::from(arg.to_str().unwrap_or("")));
        }
        Ok(con.join(" "))
    }

    fn prepare(&self) -> Command {
        let mut command = Command::new(self.cmd.as_str());
        if let Some(cd) = &self.current_dir {
            command.current_dir(cd.as_str());
        }
        for fn_arg in &self.options {
            let v = fn_arg.0();
            command.args(v);
        }
        command
    }

    fn get_output_string(out: Vec<u8>) -> String {
        String::from_utf8(out).unwrap_or_else(|_| String::from(""))
    }
}
