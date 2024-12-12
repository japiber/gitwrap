use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn tag(current_dir: Option<&str>) -> WrapCommand {
    git("tag", current_dir)
}
