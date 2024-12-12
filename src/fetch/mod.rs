use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn fetch(current_dir: Option<&str>) -> WrapCommand {
    git("fetch", current_dir)
}
