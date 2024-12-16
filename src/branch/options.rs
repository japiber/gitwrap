// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// use <n> digits to display SHA-1s
/// --abbrev[=<n>]
pub fn abbrev(n_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--abbrev", n_arg)
}

/// All list both remote-tracking and local branches
/// -a, --all
pub fn all() -> FnOptionArg {
    optionarg::simple("--all")
}

/// specify a valid branch name
/// <branch_name>
pub fn branch_name(branch_name: &str) -> FnOptionArg {
    optionarg::value_parameter(branch_name)
}

/// use colored output
/// --color[=<when>]
pub fn color(when_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--color", when_arg)
}

/// list branches in columns
/// --column[=<style>]
pub fn column(style_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--column", style_arg)
}

/// print only branches that contain the commit
/// --contains <commit>
pub fn contains(commit_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--contains", commit_arg)
}

/// print only branches that don't contain the commit
/// --no-contains <commit>
pub fn no_contains(commit_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--no-contains", commit_arg)
}

/// create the branch's reflog
/// -l, --create-reflog
pub fn create_reflog() -> FnOptionArg {
    optionarg::simple("--create-reflog")
}

/// delete fully merged branch
/// -d, --delete
pub fn delete() -> FnOptionArg {
    optionarg::simple("--delete")
}

/// delete branch (even if not merged)
/// -D
pub fn delete_force() -> FnOptionArg {
    optionarg::simple("-D")
}

/// edit the description for the branch
/// --edit-description
pub fn edit_description() -> FnOptionArg {
    optionarg::simple("--edit-description")
}

/// force creation, move/rename, deletion
/// -f, --force
pub fn force() -> FnOptionArg {
    optionarg::simple("--force")
}

/// format to use for the output
/// --format <format>
pub fn format(format_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--format", format_arg)
}

/// sorting and filtering are case insensitive
/// -i, --ignore-case
pub fn ignore_case() -> FnOptionArg {
    optionarg::simple("--ignore-case")
}

/// list branch names
/// --list
pub fn list() -> FnOptionArg {
    optionarg::simple("--list")
}

/// print only branches of the object
/// --points-at <object>
pub fn points_at(object_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--points-at", object_arg)
}

/// print only branches that are merged
/// --merged <commit>
pub fn merged(commit_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--merged", commit_arg)
}

/// print only branches that are not merged
/// --no-merged <commit>
pub fn no_merged(commit_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--no-merged", commit_arg)
}

/// move/rename a branch and its reflog
/// -m, --move
pub fn move_branch() -> FnOptionArg {
    optionarg::simple("--move")
}

/// move/rename a branch, even if target exists
/// -M
pub fn move_force() -> FnOptionArg {
    optionarg::simple("-M")
}

/// suppress informational messages
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple("--quiet")
}

/// act on remote-tracking branches
/// -r, --remotes
pub fn remotes() -> FnOptionArg {
    optionarg::simple("--remotes")
}

/// print the name of the current branch. In detached HEAD state, nothing is printed
/// --show-current
pub fn show_current() -> FnOptionArg {
    optionarg::simple("--show-current")
}

/// SetUpstream change upstream info
/// --unset-upstream
pub fn unset_upstream(branchname_arg: &str) -> FnOptionArg {
    optionarg::with_optional_parameter("--unset-upstream", branchname_arg)
}

/// change the upstream info to upstream
/// -u, --set-upstream-to <upstream>
pub fn set_upstream_to(upstream_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--set-upstream-to", upstream_arg)
}

/// field name to sort on
/// --sort <key>
pub fn sort(key_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--sort", key_arg)
}

/// set up tracking mode (see git-pull(1)
/// -t, --track
pub fn track() -> FnOptionArg {
    optionarg::simple("--track")
}

/// show hash and subject, give twice for upstream branch
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    optionarg::simple("--verbose")
}
