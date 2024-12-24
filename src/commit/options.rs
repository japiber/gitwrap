// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const MESSAGE: &str = "--message";
pub const ALL: &str = "--all";
pub const PATCH: &str = "--patch";
pub const REUSE_MESSAGE: &str = "--reuse-message";
pub const REEDIT_MESSAGE: &str = "--reedit-message";
pub const FIXUP: &str = "--fixup";
pub const SQUASH: &str = "--squash";
pub const RESET_AUTHOR: &str = "--reset-author";
pub const SHORT: &str = "--short";
pub const BRANCH: &str = "--branch";
pub const PORCELAIN: &str = "--porcelain";
pub const LONG: &str = "--long";
pub const NULL: &str = "--null";
pub const FILE: &str = "--file";
pub const AUTHOR: &str = "--author";
pub const DATE: &str = "--date";
pub const TEMPLATE: &str = "--template";
pub const SIGNOFF: &str = "--signoff";
pub const NO_VERIFY: &str = "--no-verify";
pub const ALLOW_EMPTY: &str = "--allow-empty";
pub const ALLOW_EMPTY_MESSAGE: &str = "--allow-empty-message";
pub const CLEANUP: &str = "--cleanup";
pub const EDIT: &str = "--edit";
pub const NO_EDIT: &str = "--no-edit";
pub const AMEND: &str = "--amend";
pub const NO_POST_REWRITE: &str = "--no-post-rewrite";
pub const INCLUDE: &str = "--include";
pub const ONLY: &str = "--only";
pub const UNTRACKED_FILES: &str = "--untracked-files";
pub const VERBOSE: &str = "--verbose";
pub const QUIET: &str = "--quiet";
pub const DRY_RUN: &str = "--dry-run";
pub const STATUS: &str = "--status";
pub const NO_STATUS: &str = "--no-status";
pub const GPG_SIGN: &str = "--gpg-sign";
pub const NO_GPG_SIGN: &str = "--no-gpg-sign";
pub const HYPHEN_HYPHEN: &str = "--";

/// Use the given <msg> as the commit message.
/// If multiple -m options are given, their values are concatenated as separate paragraphs.
/// -m <msg>, --message=<msg>
pub fn message(msg_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(MESSAGE, msg_arg)
}

/// Tell the command to automatically stage files that have been modified and deleted, but new files you have not told Git about are not affected.
/// -a, --all
pub fn all() -> FnOptionArg {
    option_arg::simple(ALL)
}

/// Use the interactive patch selection interface to chose which changes to commit.
/// See git-add(1) for details.
/// -p, --patch
pub fn patch() -> FnOptionArg {
    option_arg::simple(PATCH)
}

/// Take an existing commit object, and reuse the log message and the authorship information (including the timestamp) when creating the commit.
/// -C <commit>, --reuse-message=<commit>
pub fn reuse_message(commit_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(REUSE_MESSAGE, commit_arg)
}

/// Like -C, but with -c the editor is invoked, so that the user can further edit the commit message.
/// -c <commit>, --reedit-message=<commit>
pub fn reedit_message(commit_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(REEDIT_MESSAGE, commit_arg)
}

/// Construct a commit message for use with rebase --autosquash.
/// The commit message will be the subject line from the specified commit with a prefix of 'fixup! '.
/// See git-rebase(1) for details.
/// --fixup=<commit>
pub fn fixup(commit_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(FIXUP, commit_arg)
}

/// Construct a commit message for use with rebase --autosquash.
/// The commit message subject line is taken from the specified commit with a prefix of 'squash! '.
/// Can be used with additional commit message options (-m/-c/-C/-F).
/// See git-rebase(1) for details.
/// --squash=<commit>
pub fn squash(commit_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(SQUASH, commit_arg)
}

/// When used with -C/-c/--amend options, or when committing after a conflicting cherry-pick, declare that the authorship of the resulting commit now belongs to the committer.
/// This also renews the author timestamp.
/// --reset-author
pub fn reset_author() -> FnOptionArg {
    option_arg::simple(RESET_AUTHOR)
}

/// When doing a dry-run, give the output in the short-format.
/// See git-status(1) for details.
/// Implies --dry-run.
/// --short
pub fn short() -> FnOptionArg {
    option_arg::simple(SHORT)
}

/// Show the branch and tracking info even in short-format.
/// --branch
pub fn branch() -> FnOptionArg {
    option_arg::simple(BRANCH)
}

/// When doing a dry-run, give the output in a porcelain-ready format.
/// See git-status(1) for details.
/// Implies --dry-run.
/// --porcelain
pub fn porcelain() -> FnOptionArg {
    option_arg::simple(PORCELAIN)
}

/// When doing a dry-run, give the output in the long-format.
/// Implies --dry-run.
/// --long
pub fn long() -> FnOptionArg {
    option_arg::simple(LONG)
}

/// When showing short or porcelain status output, print the filename verbatim and terminate the entries with NUL, instead of LF.
/// If no format is given, implies the --porcelain output format.
/// Without the -z option, filenames with 'unusual' characters are quoted as explained for the configuration variable core.quotePath (see git-config(1)).
/// -z, --null
pub fn null() -> FnOptionArg {
    option_arg::simple(NULL)
}

/// Take the commit message from the given file.
/// Use - to read the message from the standard input.
/// -F <file>, --file=<file>
pub fn file(file_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(FILE, file_arg)
}

/// Override the commit author.
/// Specify an explicit author using the standard A U Thor <author@example.com> format.
/// Otherwise <author> is assumed to be a pattern and is used to search for an existing commit by that author (i.e.
/// rev-list --all -i --author=<author>); the commit author is then copied from the first such commit found.
/// --author=<author>
pub fn author(author_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(AUTHOR, author_arg)
}

/// Override the author date used in the commit.
/// --date=<date>
pub fn date(date_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(DATE, date_arg)
}

/// When editing the commit message, start the editor with the contents in the given file.
/// The commit.template configuration variable is often used to give this option implicitly to the command.
/// This mechanism can be used by projects that want to guide participants with some hints on what to write in the message in what order.
/// If the user exits the editor without editing the message, the commit is aborted.
/// This has no effect when a message is given by other means, e.g.
/// with the -m or -F options.
/// -t <file>, --template=<file>
pub fn template(file_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(TEMPLATE, file_arg)
}

/// Add Signed-off-by line by the committer at the end of the commit log message.
/// The meaning of a signoff depends on the project, but it typically certifies that committer has the rights to submit this work under the same license and agrees to a Developer Certificate of Origin (see https://developercertificate.org/ for more information).
/// -s, --signoff
pub fn signoff() -> FnOptionArg {
    option_arg::simple(SIGNOFF)
}

/// This option bypasses the pre-commit and commit-msg hooks.
/// See also githooks(5).
/// -n, --no-verify
pub fn no_verify() -> FnOptionArg {
    option_arg::simple(NO_VERIFY)
}

/// Usually recording a commit that has the exact same tree as its sole parent commit is a mistake, and the command prevents you from making such a commit.
/// This option bypasses the safety, and is primarily for use by foreign SCM interface scripts.
/// --allow-empty
pub fn allow_empty() -> FnOptionArg {
    option_arg::simple(ALLOW_EMPTY)
}

/// Like --allow-empty this command is primarily for use by foreign SCM interface scripts.
/// It allows you to create a commit with an empty commit message without using plumbing commands like git-commit-tree(1).
/// --allow-empty-message
pub fn allow_empty_message() -> FnOptionArg {
    option_arg::simple(ALLOW_EMPTY_MESSAGE)
}

/// This option determines how the supplied commit message should be cleaned up before committing.
/// The <mode> can be strip, whitespace, verbatim, scissors or default.
/// --cleanup=<mode>
pub fn cleanup(mode_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(CLEANUP, mode_arg)
}

/// The message taken from file with -F, command line with -m, and from commit object with -C are usually used as the commit log message unmodified.
/// This option lets you further edit the message taken from these sources.
/// -e, --edit
pub fn edit() -> FnOptionArg {
    option_arg::simple(EDIT)
}

/// Use the selected commit message without launching an editor.
/// For example, git commit --amend --no-edit amends a commit without changing its commit message.
/// --no-edit
pub fn no_edit() -> FnOptionArg {
    option_arg::simple(NO_EDIT)
}

/// Replace the tip of the current branch by creating a new commit.
/// The recorded tree is prepared as usual (including the effect of the -i and -o options and explicit pathspec), and the message from the original commit is used as the starting point, instead of an empty message, when no other message is specified from the command line via options such as -m, -F, -c, etc.
/// The new commit has the same parents and author as the current one (the --reset-author option can countermand this).
/// --amend
pub fn amend() -> FnOptionArg {
    option_arg::simple(AMEND)
}

/// Bypass the post-rewrite hook.
/// --no-post-rewrite
pub fn no_post_rewrite() -> FnOptionArg {
    option_arg::simple(NO_POST_REWRITE)
}

/// Before making a commit out of staged contents so far, stage the contents of paths given on the command line as well.
/// This is usually not what you want unless you are concluding a conflicted merge.
/// -i, --include
pub fn include() -> FnOptionArg {
    option_arg::simple(INCLUDE)
}

/// Make a commit by taking the updated working tree contents of the paths specified on the command line, disregarding any contents that have been staged for other paths.
/// This is the default mode of operation of git commit if any paths are given on the command line, in which case this option can be omitted.
/// If this option is specified together with --amend, then no paths need to be specified, which can be used to amend the last commit without committing changes that have already been staged.
/// If used together with --allow-empty paths are also not required, and an empty commit will be created.
/// -o, --only
pub fn only() -> FnOptionArg {
    option_arg::simple(ONLY)
}

/// Show untracked files.
/// The mode parameter is optional (defaults to all), and is used to specify the handling of untracked files; when -u is not used, the default is normal, i.e.
/// show untracked files and directories.
/// -u[<mode>], --untracked-files[=<mode>]
pub fn untracked_files(mode_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(UNTRACKED_FILES, mode_arg)
}

/// Show unified diff between the HEAD commit and what would be committed at the bottom of the commit message template to help the user describe the commit by reminding what changes the commit has.
/// Note that this diff output doesn’t have its lines prefixed with #.
/// This diff will not be a part of the commit message.
/// See the commit.verbose configuration variable in git-config(1).
/// If specified twice, show in addition the unified diff between what would be committed and the worktree files, i.e.
/// the unstaged changes to tracked files.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    option_arg::simple(VERBOSE)
}

/// Suppress commit summary message.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    option_arg::simple(QUIET)
}

/// Do not create a commit, but show a list of paths that are to be committed,
/// paths with local changes that will be left uncommitted and paths that are untracked.
/// --dry-run
pub fn dry_run() -> FnOptionArg {
    option_arg::simple(DRY_RUN)
}

/// Include the output of git-status(1) in the commit message template when using an editor to prepare the commit message.
/// Defaults to on, but can be used to override configuration variable commit.status.
/// --status
pub fn status() -> FnOptionArg {
    option_arg::simple(STATUS)
}

/// Do not include the output of git-status(1) in the commit message template when using an editor to prepare the default commit message.
/// --no-status
pub fn no_status() -> FnOptionArg {
    option_arg::simple(NO_STATUS)
}

/// GPG-sign commits.
/// The keyid argument is optional and defaults to the committer identity; if specified, it must be stuck to the option without a space.
/// -S[<keyid>], --gpg-sign[=<keyid>]
pub fn gpg_sign(keyid_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(GPG_SIGN, keyid_arg)
}

/// Countermand commit.gpgSign configuration variable that is set to force each and every commit to be signed.
/// --no-gpg-sign
pub fn no_gpg_sign() -> FnOptionArg {
    option_arg::simple(NO_GPG_SIGN)
}

/// Do not interpret any more arguments as options
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    option_arg::simple(HYPHEN_HYPHEN)
}

/// When pathspec is given on the command line, commit the contents of the files that match the pathspec without recording the changes already added to the index.
/// The contents of these files are also staged for the next commit on top of what have been staged before.
/// <pathspec>
pub fn pathspec(pathspec: &str) -> FnOptionArg {
    option_arg::value_parameter(pathspec)
}
