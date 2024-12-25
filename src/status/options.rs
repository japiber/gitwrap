// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const SHORT: &str = "--short";
pub const BRANCH: &str = "--branch";
pub const SHOW_STASH: &str = "--show-stash";
pub const PORCELAIN: &str = "--porcelain";
pub const LONG: &str = "--long";
pub const VERBOSE: &str = "--verbose";
pub const UNTRACKED_FILES: &str = "--untracked-files";
pub const IGNORE_SUBMODULES: &str = "--ignore-submodules";
pub const IGNORED: &str = "--ignored";
pub const NULL: &str = "--null";
pub const COLUMN: &str = "--column";
pub const NO_COLUMN: &str = "--no-column";
pub const AHEAD_BEHIND: &str = "--ahead-behind";
pub const NO_AHEAD_BEHIND: &str = "--no-ahead-behind";
pub const RENAMES: &str = "--renames";
pub const NO_RENAMES: &str = "--no-renames";
pub const FIND_RENAMES: &str = "--find-renames";

/// Give the output in the short-format.
/// -s, --short
pub fn short() -> FnOptionArg {
    option_arg::simple(SHORT)
}

/// Show the branch and tracking info even in short-format.
/// -b, --branch
pub fn branch() -> FnOptionArg {
    option_arg::simple(BRANCH)
}

/// Show the number of entries currently stashed away.
/// --show-stash
pub fn show_stash() -> FnOptionArg {
    option_arg::simple(SHOW_STASH)
}

/// Give the output in an easy-to-parse format for scripts.
/// This is similar to the short output, but will remain stable across Git versions and regardless of user configuration.
/// See below for details.
/// The version parameter is used to specify the format version.
/// This is optional and defaults to the original version v1 format.
/// --porcelain[=<version>]
pub fn porcelain(version_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(PORCELAIN, version_arg)
}

/// Give the output in the long-format.
/// This is the default.
/// --long
pub fn long() -> FnOptionArg {
    option_arg::simple(LONG)
}

/// In addition to the names of files that have been changed,
/// also show the textual changes that are staged to be committed (i.e., like the output of git diff --cached).
/// If -v is specified twice, then also show the changes in the working tree that have not yet been staged (i.e., like the output of git diff).
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    option_arg::simple(VERBOSE)
}

/// Show untracked files.
/// The mode parameter is used to specify the handling of untracked files.
/// It is optional: it defaults to all, and if specified, it must be stuck to the option (e.g.
///  -uno, but not -u no).
/// The possible options are:
/// •   no - Show no untracked files.
/// •   normal - Shows untracked files and directories.
/// •   all - Also shows individual files in untracked directories.
/// When -u option is not used, untracked files and directories are shown (i.e.
/// the same as specifying normal), to help you avoid forgetting to add newly created files.
/// Because it takes extra
/// work to find untracked files in the filesystem, this mode may take some time in a large working tree.
/// Consider enabling untracked cache and split index if supported (see git update-index
/// --untracked-cache and git update-index --split-index), Otherwise you can use no to have git status return more quickly without showing untracked files.
/// The default can be changed using the status.showUntrackedFiles configuration variable documented in git-config(1).
/// -u[<mode>], --untracked-files[=<mode>]
pub fn untracked_files(mode_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(UNTRACKED_FILES, mode_arg)
}

/// Ignore changes to submodules when looking for changes.
/// <when> can be either "none", "untracked", "dirty" or "all", which is the default.
/// Using "none" will consider the submodule modified when it either contains untracked or modified files or its HEAD differs from the commit recorded in the superproject and can be used to override any settings of the ignore option in git- config(1) or gitmodules(5).
/// When "untracked" is used submodules are not considered dirty when they only contain untracked content (but they are still scanned for modified content).
/// Using "dirty" ignores all changes to the work tree of submodules, only changes to the commits stored in the superproject are shown (this was the behavior before 1.7.0).
/// Using "all" hides all changes to submodules (and suppresses the output of submodule summaries when the config option status.submoduleSummary is set).
/// --ignore-submodules[=<when>]
pub fn ignore_submodules(when_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(IGNORE_SUBMODULES, when_arg)
}

/// Show ignored files as well.
/// The mode parameter is used to specify the handling of ignored files.
/// It is optional: it defaults to traditional.
/// The possible options are:
/// •   traditional - Shows ignored files and directories, unless --untracked-files=all is specified, in which case individual files in ignored directories are displayed.
/// •   no - Show no ignored files.
/// •   matching - Shows ignored files and directories matching an ignore pattern.
/// When matching mode is specified, paths that explicitly match an ignored pattern are shown.
/// If a directory matches an ignore pattern, then it is shown, but not paths contained in the ignored directory.
/// If a directory does not match an ignore pattern, but all contents are ignored, then the directory is not shown, but all contents are shown.
/// --ignored[=<mode>]
pub fn ignored(mode_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(IGNORED, mode_arg)
}

/// Terminate entries with NUL, instead of LF.
/// This implies the --porcelain=v1 output format if no other format is given.
/// -z, --null
pub fn null() -> FnOptionArg {
    option_arg::simple(NULL)
}

/// Display untracked files in columns.
/// See configuration variable column.status for option syntax.
/// --column and --no-column without options are equivalent to always and never respectively.
/// --column[=<options>], --no-column
pub fn column(options_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(COLUMN, options_arg)
}

/// Display untracked files in columns.
/// See configuration variable column.status for option syntax.
/// --column and --no-column without options are equivalent to always and never respectively.
/// --column[=<options>], --no-column
pub fn no_column() -> FnOptionArg {
    option_arg::simple(NO_COLUMN)
}

/// Display or do not display detailed ahead/behind counts for the branch relative to its upstream branch.
/// Defaults to true.
/// --ahead-behind, --no-ahead-behind
pub fn ahead_behind() -> FnOptionArg {
    option_arg::simple(AHEAD_BEHIND)
}

/// Display or do not display detailed ahead/behind counts for the branch relative to its upstream branch.
/// Defaults to true.
/// --ahead-behind, --no-ahead-behind
pub fn no_ahead_behind() -> FnOptionArg {
    option_arg::simple(NO_AHEAD_BEHIND)
}

/// Turn on/off rename detection regardless of user configuration.
/// See also git-diff(1) --no-renames.
/// --renames, --no-renames
pub fn renames() -> FnOptionArg {
    option_arg::simple(RENAMES)
}

/// Turn on/off rename detection regardless of user configuration.
/// See also git-diff(1) --no-renames.
/// --renames, --no-renames
pub fn no_renames() -> FnOptionArg {
    option_arg::simple(NO_RENAMES)
}

/// Turn on rename detection, optionally setting the similarity threshold.
/// See also git-diff(1) --find-renames.
/// -M, --find-renames[=<n>]
pub fn find_renames(n_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(FIND_RENAMES, n_arg)
}
