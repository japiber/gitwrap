use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn commit(current_dir: Option<&str>) -> WrapCommand {
    git("commit", current_dir)
}
