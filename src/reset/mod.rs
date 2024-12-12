use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn reset(current_dir: Option<&str>) -> WrapCommand {
    git("reset", current_dir)
}
