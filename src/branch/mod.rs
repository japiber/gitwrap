use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn branch(current_dir: Option<&str>) -> WrapCommand {
    git("branch", current_dir)
}
