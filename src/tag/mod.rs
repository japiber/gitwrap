use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "tag";

/// Create, list, delete or verify a tag object signed with GPG
/// [Git doc](https://git-scm.com/docs/git-tag)
pub fn tag() -> WrapCommand {
    git(GIT_COMMAND)
}
