use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "checkout";

/// Switch branches or restore working tree files
/// [Git doc](https://git-scm.com/docs/git-checkout)
pub fn checkout() -> WrapCommand {
    git(GIT_COMMAND)
}
