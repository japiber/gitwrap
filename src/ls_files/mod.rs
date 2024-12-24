use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "ls-files";

/// Show information about files in the index and the working tree.
/// This command merges the file listing in the index with the actual working directory list, and shows different combinations of the two.
/// [Git doc](https://git-scm.com/docs/git-ls-files)
pub fn ls_files() -> WrapCommand {
    git(GIT_COMMAND)
}
