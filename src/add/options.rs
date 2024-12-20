// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;

pub const DRY_RUN: &str = "--dry-run";
pub const VERBOSE: &str = "--verbose";
pub const FORCE: &str = "--force";
pub const INTERACTIVE: &str = "--interactive";
pub const PATCH: &str = "--patch";
pub const EDIT: &str = "--edit";
pub const UPDATE: &str = "--update";
pub const ALL: &str = "--all";
pub const NO_ALL: &str = "--no-all";
pub const INTENT_TO_ADD: &str = "--intent-to-add";
pub const REFRESH: &str = "--refresh";
pub const IGNORE_ERRORS: &str = "--ignore-errors";
pub const IGNORE_MISSING: &str = "--ignore-missing";
pub const HYPHEN_HYPHEN: &str = "--";

/// Don't actually add the file(s), just show if they exist and/or will be ignored.
/// -n, --dry-run
pub fn dry_run() -> FnOptionArg {
    optionarg::simple(DRY_RUN)
}

/// Be verbose.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    optionarg::simple(VERBOSE)
}

/// Allow adding otherwise ignored files.
/// -f, --force
pub fn force() -> FnOptionArg {
    optionarg::simple(FORCE)
}

/// Add modified contents in the working tree interactively to the index.
/// Optional path arguments may be supplied to limit operation to a subset of the working tree.
/// See 'Interactive mode' for details.
/// -i, --interactive
pub fn interactive() -> FnOptionArg {
    optionarg::simple(INTERACTIVE)
}

/// Interactively choose hunks of patch between the index and the work tree and add them to the index.
/// This gives the user a chance to review the difference before adding modified contents to the index.
/// This effectively runs add --interactive, but bypasses the initial command menu and directly jumps to the patch subcommand.
/// See “Interactive mode” for details.
/// -p, --patch
pub fn patch() -> FnOptionArg {
    optionarg::simple(PATCH)
}

/// Open the diff vs.
/// the index in an editor and let the user edit it.
/// After the editor was closed, adjust the hunk headers and apply the patch to the index.
/// The intent of this option is to pick and choose lines of the patch to apply, or even to modify the contents of lines to be staged.
/// This can be quicker and more flexible than using the interactive hunk selector.
/// However, it is easy to confuse oneself and create a patch that does not apply to the index.
/// See EDITING PATCHES below.
/// -e, --edit
pub fn edit() -> FnOptionArg {
    optionarg::simple(EDIT)
}

/// Update the index just where it already has an entry matching <pathspec>.
/// This removes as well as modifies index entries to match the working tree, but adds no new files.
/// If no <pathspec> is given when -u option is used, all tracked files in the entire working tree are updated (old versions of Git used to limit the update to the current directory and its subdirectories).
/// -u, --update
pub fn update() -> FnOptionArg {
    optionarg::simple(UPDATE)
}

/// Update the index not only where the working tree has a file matching <pathspec> but also where the index already has an entry.
/// This adds, modifies, and removes index entries to match the working tree.
/// If no <pathspec> is given when -A option is used, all files in the entire working tree are updated (old versions of Git used to limit the update to the current directory and its subdirectories).
/// -A, --all, --no-ignore-removal
pub fn all() -> FnOptionArg {
    optionarg::simple(ALL)
}

/// Update the index by adding new files that are unknown to the index and files modified in the working tree, but ignore files that have been removed from the working tree.
/// This option is a no-op when no <pathspec> is used.
/// This option is primarily to help users who are used to older versions of Git, whose 'git add <pathspec>...' was a synonym for 'git add --no-all <pathspec>...', i.e.
/// ignored removed files.
/// --no-all, --ignore-removal
pub fn no_all() -> FnOptionArg {
    optionarg::simple(NO_ALL)
}

/// Record only the fact that the path will be added later.
/// An entry for the path is placed in the index with no content.
/// This is useful for, among other things, showing the unstaged content of such files with git diff and committing them with git commit -a.
/// -N, --intent-to-add
pub fn intent_to_add() -> FnOptionArg {
    optionarg::simple(INTENT_TO_ADD)
}

/// Don't add the file(s), but only refresh their stat() information in the index.
/// --refresh
pub fn refresh() -> FnOptionArg {
    optionarg::simple(REFRESH)
}

/// If some files could not be added because of errors indexing them, do not abort the operation, but continue adding the others.
/// The command shall still exit with non-zero status.
/// The configuration variable add.ignoreErrors can be set to true to make this the default behaviour.
/// --ignore-errors
pub fn ignore_errors() -> FnOptionArg {
    optionarg::simple(IGNORE_ERRORS)
}

/// This option can only be used together with --dry-run.
/// By using this option the user can check if any of the given files would be ignored, no matter if they are already present in the work tree or not.
/// --ignore-missing
pub fn ignore_missing() -> FnOptionArg {
    optionarg::simple(IGNORE_MISSING)
}

/// This option can be used to separate command-line options from the list of files, (useful when filenames might be mistaken for command-line options)
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    optionarg::simple(HYPHEN_HYPHEN)
}

/// Files to add content from.
/// Fileglobs (e.g.
/// *.c) can be given to add all matching files.
/// Also a leading directory name (e.g.
/// dir to add dir/file1 and dir/file2) can be given to update the index to match the current state of the directory as a whole (e.g.
/// specifying dir will record not just a file dir/file1 modified in the working tree, a file dir/file2 added to the working tree, but also a file dir/file3 removed from the working tree).
/// Note that older versions of Git used to ignore removed files; use --no-all option if you want to add modified or new files but ignore removed ones.
/// <pathspec>
pub fn pathspec(pathspec: &str) -> FnOptionArg {
    optionarg::value_parameter(pathspec)
}
