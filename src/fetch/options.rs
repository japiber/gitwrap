// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const ALL: &str = "--all";
pub const APPEND: &str = "--append";
pub const DEPTH: &str = "--depth";
pub const DEEPEN: &str = "--deepen";
pub const SHALLOW_SINCE: &str = "--shallow-since";
pub const SHALLOW_EXCLUDE: &str = "--shallow-exclude";
pub const UNSHALLOW: &str = "--unshallow";
pub const UPDATE_SHALLOW: &str = "--update-shallow";
pub const DRY_RUN: &str = "--dry-run";
pub const FORCE: &str = "--force";
pub const KEEP: &str = "--keep";
pub const MULTIPLE: &str = "--multiple";
pub const PRUNE: &str = "--prune";
pub const NO_TAGS: &str = "--no-tags";
pub const REFMAP: &str = "--refmap";
pub const TAGS: &str = "--tags";
pub const RECURSE_SUBMODULES: &str = "--recurse-submodules";
pub const JOBS: &str = "--jobs";
pub const NO_RECURSE_SUBMODULES: &str = "--no-recurse-submodules";
pub const SUBMODULE_PREFIX: &str = "--submodule-prefix";
pub const RECURSE_SUBMODULES_DEFAULT: &str = "--recurse-submodules-default";
pub const UPDATE_HEAD_OK: &str = "--update-head-ok";
pub const UPLOAD_PACK: &str = "--upload-pack";
pub const QUIET: &str = "--quiet";
pub const VERBOSE: &str = "--verbose";
pub const PROGRESS: &str = "--progress";
pub const IPV4: &str = "--ipv4";
pub const IPV6: &str = "--ipv6";

/// Fetch all remotes.
/// --all
pub fn all() -> FnOptionArg {
    option_arg::simple(ALL)
}

/// Append ref names and object names of fetched refs to the existing contents of .git/FETCH_HEAD.
/// Without this option old data in .git/FETCH_HEAD will be overwritten.
/// -a, --append
pub fn append() -> FnOptionArg {
    option_arg::simple(APPEND)
}

/// Limit fetching to the specified number of commits from the tip of each remote branch history.
/// If fetching to a shallow repository created by git clone with --depth=<depth> option (see git-clone(1)), deepen or shorten the history to the specified number of commits.
/// Tags for the deepened commits are not fetched.
/// --depth=<depth>
pub fn depth(depth_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(DEPTH, depth_arg)
}

/// Similar to --depth, except it specifies the number of commits from the current shallow boundary instead of from the tip of each remote branch history.
/// --deepen=<depth>
pub fn deepen(depth_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(DEEPEN, depth_arg)
}

/// Deepen or shorten the history of a shallow repository to include all reachable commits after <date>.
/// --shallow-since=<date>
pub fn shallow_since(date_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(SHALLOW_SINCE, date_arg)
}

/// Deepen or shorten the history of a shallow repository to exclude commits reachable from a specified remote branch or tag.
/// This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(SHALLOW_EXCLUDE, revision_arg)
}

/// If the source repository is complete, convert a shallow repository to a complete one, removing all the limitations imposed by shallow repositories.
/// If the source repository is shallow, fetch as much as possible so that the current repository has the same history as the source repository.
/// --unshallow
pub fn unshallow() -> FnOptionArg {
    option_arg::simple(UNSHALLOW)
}

/// By default when fetching from a shallow repository, git fetch refuses refs that require updating .git/shallow.
/// This option updates .git/shallow and accept such refs.
/// --update-shallow
pub fn update_shallow() -> FnOptionArg {
    option_arg::simple(UPDATE_SHALLOW)
}

/// Show what would be done, without making any changes.
/// --dry-run
pub fn dry_run() -> FnOptionArg {
    option_arg::simple(DRY_RUN)
}

/// When git fetch is used with <rbranch>:<lbranch> refspec, it refuses to update the local branch <lbranch> unless the remote branch <rbranch> it fetches is a descendant of <lbranch>.
/// This option overrides that check.
/// -f, --force
pub fn force() -> FnOptionArg {
    option_arg::simple(FORCE)
}

/// Keep downloaded pack.
/// -k, --keep
pub fn keep() -> FnOptionArg {
    option_arg::simple(KEEP)
}

/// Allow several <repository> and <group> arguments to be specified.
/// No <refspec>s may be specified.
/// --multiple
pub fn multiple() -> FnOptionArg {
    option_arg::simple(MULTIPLE)
}

/// Before fetching, remove any remote-tracking references that no longer exist on the remote.
/// -p, --prune
pub fn prune() -> FnOptionArg {
    option_arg::simple(PRUNE)
}

/// By default, tags that point at objects that are downloaded from the remote repository are fetched and stored locally.
/// This option disables this automatic tag following.
/// The default behavior for a remote may be specified with the remote.<name>.tagOpt setting.
/// -n, --no-tags
pub fn no_tags() -> FnOptionArg {
    option_arg::simple(NO_TAGS)
}

/// When fetching refs listed on the command line, use the specified refspec (can be given more than once) to map the refs to remote-tracking branches, instead of the values of remote.*.fetch configuration variables for the remote repository.
/// --refmap=<refspec>
pub fn refmap(refspec_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(REFMAP, refspec_arg)
}

/// Fetch all tags from the remote (i.e., fetch remote tags refs/tags/* into local tags with the same name), in addition to whatever else would otherwise be fetched.
/// -t, --tags
pub fn tags() -> FnOptionArg {
    option_arg::simple(TAGS)
}

/// This option controls if and under what conditions new commits of populated submodules should be fetched too.
/// --recurse-submodules[=yes|on-demand|no]
pub fn recurse_submodules(value: &str) -> FnOptionArg {
    option_arg::equal_optional(RECURSE_SUBMODULES, value)
}

/// Number of parallel children to be used for fetching submodules.
/// Each will fetch from different submodules, such that fetching many submodules will be faster.
/// By default submodules will be fetched one at a time.
/// -j, --jobs=<n>
pub fn jobs(n_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(JOBS, n_arg)
}

/// Disable recursive fetching of submodules (this has the same effect as using the --recurse-submodules=no option).
/// --no-recurse-submodules
pub fn no_recurse_submodules() -> FnOptionArg {
    option_arg::simple(NO_RECURSE_SUBMODULES)
}

/// Prepend <path> to paths printed in informative messages.
/// --submodule-prefix=<path>
pub fn submodule_prefix(path_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(SUBMODULE_PREFIX, path_arg)
}

/// This option is used internally to temporarily provide a non-negative default value for the --recurse-submodules option.
/// All other methods of configuring fetch’s submodule recursion (such as settings in gitmodules(5) and git-config(1)) override this option, as does specifying --[no-]recurse-submodules directly.
/// --recurse-submodules-default=[yes|on-demand]
pub fn recurse_submodules_default(value: &str) -> FnOptionArg {
    option_arg::equal_optional(RECURSE_SUBMODULES_DEFAULT, value)
}

/// By default git fetch refuses to update the head which corresponds to the current branch.
/// This flag disables the check.
/// This is purely for the internal use for git pull to communicate with git fetch, and unless you are implementing your own Porcelain you are not supposed to use it.
/// -u, --update-head-ok
pub fn update_head_ok() -> FnOptionArg {
    option_arg::simple(UPDATE_HEAD_OK)
}

/// When given, and the repository to fetch from is handled by git fetch-pack, --exec=<upload-pack> is passed to the command to specify non-default path for the command run on the other end.
/// --upload-pack <upload-pack>
pub fn upload_pack(upload_pack_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(UPLOAD_PACK, upload_pack_arg)
}

/// Pass --quiet to git-fetch-pack and silence any other internally used git commands.
/// Progress is not reported to the standard error stream.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    option_arg::simple(QUIET)
}

/// Be verbose.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    option_arg::simple(VERBOSE)
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> FnOptionArg {
    option_arg::simple(PROGRESS)
}

/// Use IPv4 addresses only, ignoring IPv6 addresses.
/// -4, --ipv4
pub fn ipv4() -> FnOptionArg {
    option_arg::simple(IPV4)
}

/// Use IPv6 addresses only, ignoring IPv4 addresses.
/// -6, --ipv6
pub fn ipv6() -> FnOptionArg {
    option_arg::simple(IPV6)
}
