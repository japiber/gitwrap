use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "clean";

/// Remove untracked files from the working tree.
/// Cleans the working tree by recursively removing files that are not under version control, starting from the current directory.
/// [Git doc](https://git-scm.com/docs/git-clean)
pub fn clean<I>(current_dir: Option<&str>, options: I) -> WrapCommand
where
    I: IntoIterator<Item = FnOptionArg>
{
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
