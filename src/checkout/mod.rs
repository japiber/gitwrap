use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn checkout(current_dir: Option<&str>) -> WrapCommand {
    git("checkout", current_dir)
}
