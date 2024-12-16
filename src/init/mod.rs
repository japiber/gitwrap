use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// This command creates an empty Git repository - basically a .git directory with subdirectories for objects, refs/heads, refs/tags, and template files.
/// An initial branch without any commits will be created
/// [Git doc](https://git-scm.com/docs/git-init)
pub fn init(current_dir: Option<&str>) -> WrapCommand {
    git("init", current_dir)
}
