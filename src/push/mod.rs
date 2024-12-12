use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn push(current_dir: Option<&str>) -> WrapCommand {
    git("push", current_dir)
}
