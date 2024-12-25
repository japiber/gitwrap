use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "fetch";

/// Fetch branches and/or tags (collectively, "refs") from one or more other repositories, along with the objects necessary to complete their histories.
/// Remote-tracking branches are updated.
/// [Git doc](https://git-scm.com/docs/git-fetch)
pub fn fetch() -> WrapCommand {
    git(GIT_COMMAND)
}
