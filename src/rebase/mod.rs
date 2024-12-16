use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Reapply commits on top of another base tip
/// [Git doc](https://git-scm.com/docs/git-rebase)
pub fn rebase(current_dir: Option<&str>) -> WrapCommand {
    git("rebase", current_dir)
}
