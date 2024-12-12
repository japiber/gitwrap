use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub fn ls_files(current_dir: Option<&str>) -> WrapCommand {
    git("ls-files", current_dir)
}
