use std::process::Command;
use crate::wrap_command::{git, FnOptionArg, WrapCommand};

mod options;
pub use options::*;

/*pub fn commitone(current_dir: &str) -> WrapCommand {
    let mut command = git(current_dir);
    command.option(Box::new(move |cmd: &mut Command| { cmd.arg(String::from("commit")); }));
    command
}*/

