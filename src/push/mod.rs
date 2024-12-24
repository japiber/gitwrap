use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "push";

/// Updates remote refs using local refs, while sending objects necessary to complete the given refs.
/// [Git doc](https://git-scm.com/docs/git-push)
pub fn push() -> WrapCommand {
    git(GIT_COMMAND)
}
