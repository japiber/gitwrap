use crate::wrap_command::{WrapCommand, FnOptionArg};
use crate::git;

mod options;
pub use options::*;

mod custom;
pub use custom::*;

pub const GIT_COMMAND: &str = "config";

/// Get and set repository or global options.
/// You can query/set/replace/unset options with this command.
/// The name is actually the section and the key separated by a dot, and the value will be escaped.
/// [Git doc](https://git-scm.com/docs/git-config)
pub fn config<I>(current_dir: Option<&str>, options: I) -> WrapCommand
where
    I: IntoIterator<Item = FnOptionArg>
{
    let mut gc = git(GIT_COMMAND, current_dir);
    for opt in options {
        gc.option(opt);
    }
    gc
}
