use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Show information about files in the index and the working tree.
/// This command merges the file listing in the index with the actual working directory list, and shows different combinations of the two.
/// [Git doc](https://git-scm.com/docs/git-ls-files)
pub fn ls_files(current_dir: Option<&str>) -> WrapCommand {
    git("ls-files", current_dir)
}
