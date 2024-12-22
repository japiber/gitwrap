use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "ls-files";

/// Show information about files in the index and the working tree.
/// This command merges the file listing in the index with the actual working directory list, and shows different combinations of the two.
/// [Git doc](https://git-scm.com/docs/git-ls-files)
pub fn ls_files<I>(current_dir: Option<&str>, options: I) -> WrapCommand
where
    I: IntoIterator<Item = FnOptionArg>
{
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
