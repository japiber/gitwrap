use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "init";

/// This command creates an empty Git repository - basically a .git directory with subdirectories for objects, refs/heads, refs/tags, and template files.
/// An initial branch without any commits will be created
/// [Git doc](https://git-scm.com/docs/git-init)
pub fn init<I>(current_dir: Option<&str>, options: I) -> WrapCommand
where
    I: IntoIterator<Item = FnOptionArg>
{
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
