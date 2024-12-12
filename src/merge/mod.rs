use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn merge(current_dir: Option<&str>) -> WrapCommand {
    git("merge", current_dir)
}
