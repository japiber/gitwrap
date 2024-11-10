// Code generated automatically

// This file must not be edited by hand

use crate::command_executor::{CommandExecutor, CommandOption};

/// Fetch all remotes.
/// --all
pub fn all() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--all"))
}

/// Append ref names and object names of fetched refs to the existing contents of .git/FETCH_HEAD.
/// Without this option old data in .git/FETCH_HEAD will be overwritten.
/// -a, --append
pub fn append() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--append"))
}

/// Limit fetching to the specified number of commits from the tip of each remote branch history.
/// If fetching to a shallow repository created by git clone with --depth=<depth> option (see git-clone(1)), deepen or shorten the history to the specified number of commits.
/// Tags for the deepened commits are not fetched.
/// --depth=<depth>
pub fn depth(depth_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--depth={}", depth_arg)))
}

/// Similar to --depth, except it specifies the number of commits from the current shallow boundary instead of from the tip of each remote branch history.
/// --deepen=<depth>
pub fn deepen(depth_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--deepen={}", depth_arg)))
}

/// Deepen or shorten the history of a shallow repository to include all reachable commits after <date>.
/// --shallow-since=<date>
pub fn shallow_since(date_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--shallow-since={}", date_arg)))
}

/// Deepen or shorten the history of a shallow repository to exclude commits reachable from a specified remote branch or tag.
/// This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--shallow-exclude={}", revision_arg)))
}

/// If the source repository is complete, convert a shallow repository to a complete one, removing all the limitations imposed by shallow repositories.
/// If the source repository is shallow, fetch as much as possible so that the current repository has the same history as the source repository.
/// --unshallow
pub fn unshallow() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--unshallow"))
}

/// By default when fetching from a shallow repository, git fetch refuses refs that require updating .git/shallow.
/// This option updates .git/shallow and accept such refs.
/// --update-shallow
pub fn update_shallow() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--update-shallow"))
}

/// Show what would be done, without making any changes.
/// --dry-run
pub fn dry_run() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--dry-run"))
}

/// When git fetch is used with <rbranch>:<lbranch> refspec, it refuses to update the local branch <lbranch> unless the remote branch <rbranch> it fetches is a descendant of <lbranch>.
/// This option overrides that check.
/// -f, --force
pub fn force() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--force"))
}

/// Keep downloaded pack.
/// -k, --keep
pub fn keep() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--keep"))
}

/// Allow several <repository> and <group> arguments to be specified.
/// No <refspec>s may be specified.
/// --multiple
pub fn multiple() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--multiple"))
}

/// Before fetching, remove any remote-tracking references that no longer exist on the remote.
/// -p, --prune
pub fn prune() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--prune"))
}

/// By default, tags that point at objects that are downloaded from the remote repository are fetched and stored locally.
/// This option disables this automatic tag following.
/// The default behavior for a remote may be specified with the remote.<name>.tagOpt setting.
/// -n, --no-tags
pub fn no_tags() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-tags"))
}

/// When fetching refs listed on the command line, use the specified refspec (can be given more than once) to map the refs to remote-tracking branches, instead of the values of remote.*.fetch configuration variables for the remote repository.
/// --refmap=<refspec>
pub fn refmap(refspec_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--refmap={}", refspec_arg)))
}

/// Fetch all tags from the remote (i.e., fetch remote tags refs/tags/* into local tags with the same name), in addition to whatever else would otherwise be fetched.
/// -t, --tags
pub fn tags() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--tags"))
}

/// This option controls if and under what conditions new commits of populated submodules should be fetched too.
/// --recurse-submodules[=yes|on-demand|no]
pub fn recurse_submodules(value :&str) -> CommandOption {
    if value.is_empty() {
        Box::new(|g: &mut CommandExecutor| g.add_option("--recurse-submodules"))
    } else {
        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--recurse-submodules={}", value)))
    }
}


/// Number of parallel children to be used for fetching submodules.
/// Each will fetch from different submodules, such that fetching many submodules will be faster.
/// By default submodules will be fetched one at a time.
/// -j, --jobs=<n>
pub fn jobs(n_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--jobs={}", n_arg)))
}

/// Disable recursive fetching of submodules (this has the same effect as using the --recurse-submodules=no option).
/// --no-recurse-submodules
pub fn no_recurse_submodules() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-recurse-submodules"))
}

/// Prepend <path> to paths printed in informative messages.
/// --submodule-prefix=<path>
pub fn submodule_prefix(path_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--submodule-prefix={}", path_arg)))
}

/// This option is used internally to temporarily provide a non-negative default value for the --recurse-submodules option.
/// All other methods of configuring fetch’s submodule recursion (such as settings in gitmodules(5) and git-config(1)) override this option, as does specifying --[no-]recurse-submodules directly.
/// --recurse-submodules-default=[yes|on-demand]
pub fn recurse_submodules_default(value :&str) -> CommandOption {
    if value.is_empty() {
        Box::new(|g: &mut CommandExecutor| g.add_option("--recurse-submodules-default"))
    } else {
        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--recurse-submodules-default={}", value)))
    }
}


/// By default git fetch refuses to update the head which corresponds to the current branch.
/// This flag disables the check.
/// This is purely for the internal use for git pull to communicate with git fetch, and unless you are implementing your own Porcelain you are not supposed to use it.
/// -u, --update-head-ok
pub fn update_head_ok() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--update-head-ok"))
}

/// When given, and the repository to fetch from is handled by git fetch-pack, --exec=<upload-pack> is passed to the command to specify non-default path for the command run on the other end.
/// --upload-pack <upload-pack>
pub fn upload_pack(upload_pack_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--upload-pack");
        g.add_option(upload_pack_arg);
    })
}


/// Pass --quiet to git-fetch-pack and silence any other internally used git commands.
/// Progress is not reported to the standard error stream.
/// -q, --quiet
pub fn quiet() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}

/// Be verbose.
/// -v, --verbose
pub fn verbose() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--verbose"))
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--progress"))
}

/// Use IPv4 addresses only, ignoring IPv6 addresses.
/// -4, --ipv4
pub fn ipv4() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ipv4"))
}

/// Use IPv6 addresses only, ignoring IPv4 addresses.
/// -6, --ipv6
pub fn ipv6() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ipv6"))
}