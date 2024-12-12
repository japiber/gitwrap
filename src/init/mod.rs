use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn init(current_dir: Option<&str>) -> WrapCommand {
    git("init", current_dir)
}
