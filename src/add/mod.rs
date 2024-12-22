use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "add";

/// Add file contents to the index.
/// This command updates the index using the current content found in the working tree, to prepare the content staged for the next commit.
/// It typically adds the current content of existing paths as a whole, but with some options it can also be used to add content with only part of the changes made to the working tree files applied, or remove paths that do not exist in the working tree anymore.
/// [Git doc](https://git-scm.com/docs/git-add)
pub fn add<I>(current_dir: Option<&str>, options: I) -> WrapCommand
where
    I: IntoIterator<Item = FnOptionArg>
{
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
