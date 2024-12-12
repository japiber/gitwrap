use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn notes(current_dir: Option<&str>) -> WrapCommand {
    git("notes", current_dir)
}
