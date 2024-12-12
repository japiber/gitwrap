use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn rebase(current_dir: Option<&str>) -> WrapCommand {
    git("rebase", current_dir)
}
