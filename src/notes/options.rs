// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// When adding notes to an object that already has notes, overwrite the existing notes (instead of aborting).
/// -f, --force
pub fn force() -> FnOptionArg {
    optionarg::simple("--force")
}

/// Use the given note message (instead of prompting).
/// If multiple -m options are given, their values are concatenated as separate paragraphs.
/// Lines starting with # and empty lines other than a single line between paragraphs will be stripped out.
/// -m <msg>, --message=<msg>
pub fn message(msg_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--message", msg_arg)
}

/// Take the note message from the given file.
/// Use - to read the note message from the standard input.
/// Lines starting with # and empty lines other than a single line between paragraphs will be stripped out.
/// -F <file>, --file=<file>
pub fn file(file_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--file", file_arg)
}

/// Take the given blob object (for example, another note) as the note message.
/// (Use git notes copy <object> instead to copy notes between objects.)
/// -C <object>, --reuse-message=<object>
pub fn reuse_message(object_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--reuse-message", object_arg)
}

/// Like -C, but with -c the editor is invoked, so that the user can further edit the note message.
/// -c <object>, --reedit-message=<object>
pub fn reedit_message(object_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--reedit-message", object_arg)
}

/// Allow an empty note object to be stored. The default behavior is to automatically remove empty notes.
/// --allow-empty
pub fn allow_empty() -> FnOptionArg {
    optionarg::simple("--allow-empty")
}

/// Manipulate the notes tree in <ref>. This overrides GIT_NOTES_REF and the "core.notesRef" configuration.
/// The ref specifies the full refname when it begins with refs/notes/;
/// when it begins with notes/, refs/ and otherwise refs/notes/ is prefixed to form a full name of the ref.
/// --ref <ref>
pub fn ref_notes(ref_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--ref", ref_arg)
}

/// Do not consider it an error to request removing notes from an object that does not have notes attached to it.
/// --ignore-missing
pub fn ignore_missing() -> FnOptionArg {
    optionarg::simple("--ignore-missing")
}

/// Also read the object names to remove notes from the standard input (there is no reason you cannot combine this with object names from the command line).
/// --stdin
pub fn stdin() -> FnOptionArg {
    optionarg::simple("--stdin")
}

/// Do not remove anything; just report the object names whose notes would be removed.
/// -n, --dry-run
pub fn dry_run() -> FnOptionArg {
    optionarg::simple("--dry-run")
}

/// When merging notes, resolve notes conflicts using the given strategy.
/// The following strategies are recognized: "manual" (default), "ours", "theirs", "union" and "cat_sort_uniq".
/// This option overrides the "notes.mergeStrategy" configuration setting.
/// See the "NOTES MERGE STRATEGIES" section below for more information on each notes merge strategy.
/// -s <strategy>, --strategy=<strategy>
pub fn strategy(strategy_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--strategy", strategy_arg)
}

/// Finalize an in-progress git notes merge.
/// Use this option when you have resolved the conflicts that git notes merge stored in .git/NOTES_MERGE_WORKTREE.
/// This amends the partial merge commit created by git notes merge (stored in .git/NOTES_MERGE_PARTIAL) by adding the notes in .git/NOTES_MERGE_WORKTREE.
/// The notes ref stored in the .git/NOTES_MERGE_REF symref is updated to the resulting commit.
/// --commit
pub fn commit() -> FnOptionArg {
    optionarg::simple("--commit")
}

/// Abort/reset an in-progress git notes merge, i.e. a notes merge with conflicts.
/// This simply removes all files related to the notes merge.
/// --abort
pub fn abort() -> FnOptionArg {
    optionarg::simple("--abort")
}

/// When merging notes, operate quietly.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple("--quiet")
}

/// When merging notes, be more verbose.
/// When pruning notes, report all object names whose notes are removed.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    optionarg::simple("--verbose")
}
