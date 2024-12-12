use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn pull(current_dir: Option<&str>) -> WrapCommand {
    git("pull", current_dir)
}
