use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn clean(current_dir: Option<&str>) -> WrapCommand {
    git("clean", current_dir)
}
