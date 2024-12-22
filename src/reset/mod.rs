use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "reset";

/// Reset current HEAD to the specified state
/// [Git doc](https://git-scm.com/docs/git-reset)
pub fn reset(current_dir: Option<&str>, options: Vec<FnOptionArg>) -> WrapCommand {
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
