use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

mod custom;
pub use custom::*;

pub fn config(current_dir: &str) -> WrapCommand {
    git(current_dir, "config")
}
