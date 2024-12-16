use crate::wrap_command::WrapCommand;
use crate::git;

mod options;
pub use options::*;

/// Pick out and massage parameters. 
/// [Git doc](https://git-scm.com/docs/git-rev-parse)
pub fn rev_parse(current_dir: Option<&str>) -> WrapCommand {
    git("rev-parse", current_dir)
}
