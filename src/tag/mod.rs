use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Create, list, delete or verify a tag object signed with GPG
/// [Git doc](https://git-scm.com/docs/git-tag)
pub fn tag(current_dir: Option<&str>) -> WrapCommand {
    git("tag", current_dir)
}
