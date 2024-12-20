use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "add";

/// Add file contents to the index.
/// This command updates the index using the current content found in the working tree, to prepare the content staged for the next commit.
/// It typically adds the current content of existing paths as a whole, but with some options it can also be used to add content with only part of the changes made to the working tree files applied, or remove paths that do not exist in the working tree anymore.
/// [Git doc](https://git-scm.com/docs/git-add)
pub fn add(current_dir: Option<&str>) -> WrapCommand {
    git(GIT_COMMAND, current_dir)
}
