use crate::wrap_command::WrapCommand;
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
pub fn config(current_dir: Option<&str>) -> WrapCommand {
    git(GIT_COMMAND, current_dir)
}
