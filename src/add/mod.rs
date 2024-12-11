use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn add(current_dir: &str) -> WrapCommand {
    git(current_dir, "add")
}
