use std::process::Command;
use crate::wrap_command::{git, WrapCommand};

mod options;
mod custom;

pub use options::*;
pub use custom::*;

pub fn clone(current_dir: &str) -> WrapCommand {
    let mut command = git(current_dir);
    command.option(Box::new(move |cmd: &mut Command| { cmd.arg(String::from("clone")); }));
    command
}