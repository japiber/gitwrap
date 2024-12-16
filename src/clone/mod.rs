use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

mod custom;
pub use custom::*;

/// Clones a repository into a newly created directory, creates remote-tracking branches for each branch in the cloned repository, and creates and checks out an initial branch that is forked from the cloned repository’s currently active branch
/// [Git doc](https://git-scm.com/docs/git-clone)
pub fn clone(current_dir: Option<&str>) -> WrapCommand {
    git("clone", current_dir)
}
