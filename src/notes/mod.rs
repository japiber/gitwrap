use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Add or inspect object notes.
/// A typical use of notes is to supplement a commit message without changing the commit itself. Notes can be shown by git log along with the original commit message.
/// [Git doc](https://git-scm.com/docs/git-notes)
pub fn notes(current_dir: Option<&str>) -> WrapCommand {
    git("notes", current_dir)
}
