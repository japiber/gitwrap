use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

pub const GIT_COMMAND: &str = "tag";

/// Create, list, delete or verify a tag object signed with GPG
/// [Git doc](https://git-scm.com/docs/git-tag)
pub fn tag(current_dir: Option<&str>, options: Vec<FnOptionArg>) -> WrapCommand {
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
