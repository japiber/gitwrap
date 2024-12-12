use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn rev_parse(current_dir: Option<&str>) -> WrapCommand {
    git("rev-parse", current_dir)
}
