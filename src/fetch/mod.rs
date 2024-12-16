use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Fetch branches and/or tags (collectively, "refs") from one or more other repositories, along with the objects necessary to complete their histories.
/// Remote-tracking branches are updated.
/// [Git doc](https://git-scm.com/docs/git-fetch)
pub fn fetch(current_dir: Option<&str>) -> WrapCommand {
    git("fetch", current_dir)
}
