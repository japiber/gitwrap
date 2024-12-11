use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn rev_parse(current_dir: &str) -> WrapCommand {
    git(current_dir, "rev-parse")
}
