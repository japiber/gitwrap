use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Updates remote refs using local refs, while sending objects necessary to complete the given refs.
/// [Git doc](https://git-scm.com/docs/git-push)
pub fn push(current_dir: Option<&str>) -> WrapCommand {
    git("push", current_dir)
}
