// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;

pub const LOCAL: &str = "--local";
pub const NO_HARDLINKS: &str = "--no-hardlinks";
pub const SHARED: &str = "--shared";
pub const DISSOCIATE: &str = "--dissociate";
pub const QUIET: &str = "--quiet";
pub const VERBOSE: &str = "--verbose";
pub const PROGRESS: &str = "--progress";
pub const NO_CHECKOUT: &str = "--no-checkout";
pub const BARE: &str = "--bare";
pub const MIRROR: &str = "--mirror";
pub const ORIGIN: &str = "--origin";
pub const BRANCH: &str = "--branch";
pub const UPLOAD_PACK: &str = "--upload-pack";
pub const TEMPLATE: &str = "--template";
pub const DEPTH: &str = "--depth";
pub const SHALLOW_SINCE: &str = "--shallow-since";
pub const SHALLOW_EXCLUDE: &str = "--shallow-exclude";
pub const SINGLE_BRANCH: &str = "--single-branch";
pub const NO_SINGLE_BRANCH: &str = "--no-single-branch";
pub const RECURSE_SUBMODULES: &str = "--recurse-submodules";
pub const SHALLOW_SUBMODULES: &str = "--shallow-submodules";
pub const NO_SHALLOW_SUBMODULES: &str = "--no-shallow-submodules";
pub const SEPARATE_GIT_DIR: &str = "--separate-git-dir";
pub const JOBS: &str = "--jobs";

/// When the repository to clone from is on a local machine, this flag bypasses the normal 'Git aware' transport mechanism and clones the repository by making a copy of HEAD and everything under objects and refs directories.
/// The files under .git/objects/ directory are hardlinked to save space when possible.
/// --local, -l
pub fn local() -> FnOptionArg {
    optionarg::simple(LOCAL)
}

/// Force the cloning process from a repository on a local filesystem to copy the files under the .git/objects directory instead of using hardlinks.
/// This may be desirable if you are trying to make a back-up of your repository.
/// --no-hardlinks
pub fn no_hardlinks() -> FnOptionArg {
    optionarg::simple(NO_HARDLINKS)
}

/// When the repository to clone is on the local machine, instead of using hard links, automatically setup .git/objects/info/alternates to share the objects with the source repository.
/// The resulting repository starts out without any object of its own.
/// --shared, -s
pub fn shared() -> FnOptionArg {
    optionarg::simple(SHARED)
}

/// Borrow the objects from reference repositories specified with the --reference options only to reduce network transfer,
/// and stop borrowing from them after a clone is made by making necessary local copies of borrowed objects.
/// This option can also be used when cloning locally from a repository that already borrows objects from another repository—the new repository will borrow objects from the same repository,
/// and this option can be used to stop the borrowing.
/// --dissociate
pub fn dissociate() -> FnOptionArg {
    optionarg::simple(DISSOCIATE)
}

/// Operate quietly.
/// Progress is not reported to the standard error stream.
/// --quiet, -q
pub fn quiet() -> FnOptionArg {
    optionarg::simple(QUIET)
}

/// Run verbosely.
/// Does not affect the reporting of progress status to the standard error stream.
/// --verbose, -v
pub fn verbose() -> FnOptionArg {
    optionarg::simple(VERBOSE)
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> FnOptionArg {
    optionarg::simple(PROGRESS)
}

/// No checkout of HEAD is performed after the clone is complete.
/// --no-checkout, -n
pub fn no_checkout() -> FnOptionArg {
    optionarg::simple(NO_CHECKOUT)
}

/// Make a bare Git repository.
/// That is, instead of creating <directory> and placing the administrative files in <directory>/.git, make the <directory> itself the $GIT_DIR.
/// This obviously implies the -n because there is nowhere to check out the working tree.
/// Also the branch heads at the remote are copied directly to corresponding local branch heads, without mapping them to refs/remotes/origin/.
/// When this option is used, neither remote-tracking branches nor the related configuration variables are created.
/// --bare
pub fn bare() -> FnOptionArg {
    optionarg::simple(BARE)
}

/// Set up a mirror of the source repository.
/// This implies --bare.
/// Compared to --bare, --mirror not only maps local branches of the source to local branches of the target,
/// it maps all refs (including remote-tracking branches, notes etc.) and sets up a refspec configuration such that all these refs are overwritten by a git remote update in the target repository.
/// --mirror
pub fn mirror() -> FnOptionArg {
    optionarg::simple(MIRROR)
}

/// Instead of using the remote name origin to keep track of the upstream repository, use <name>.
/// --origin <name>, -o <name>
pub fn origin(name_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(ORIGIN, name_arg)
}

/// Instead of pointing the newly created HEAD to the branch pointed to by the cloned repository’s HEAD, point to <name> branch instead.
/// In a non-bare repository, this is the branch that will be checked out.
/// --branch can also take tags and detaches the HEAD at that commit in the resulting repository.
/// --branch <name>, -b <name>
pub fn branch(name_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(BRANCH, name_arg)
}

/// When given, and the repository to clone from is accessed via ssh, this specifies a non-default path for the command run on the other end.
/// --upload-pack <upload-pack>, -u <upload-pack>
pub fn upload_pack(upload_pack_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(UPLOAD_PACK, upload_pack_arg)
}

/// Specify the directory from which templates will be used; (See the 'TEMPLATE DIRECTORY' section of git-init(1).)
/// --template=<template_directory>
pub fn template(template_directory_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(TEMPLATE, template_directory_arg)
}

/// Create a shallow clone with a history truncated to the specified number of commits.
/// Implies --single-branch unless --no-single-branch is given to fetch the histories near the tips of all branches.
/// If you want to clone submodules shallowly, also pass --shallow-submodules.
/// --depth <depth>
pub fn depth(depth_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(DEPTH, depth_arg)
}

/// Create a shallow clone with a history after the specified time.
/// --shallow-since=<date>
pub fn shallow_since(date_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(SHALLOW_SINCE, date_arg)
}

/// Create a shallow clone with a history, excluding commits reachable from a specified remote branch or tag.
/// This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(SHALLOW_EXCLUDE, revision_arg)
}

/// Clone only the history leading to the tip of a single branch, either specified by the --branch option or the primary branch remote’s HEAD points at.
/// Further fetches into the resulting repository will only update the remote-tracking branch for the branch this option was used for the initial cloning.
/// If the HEAD at the remote did not point at any branch when --single-branch clone was made, no remote-tracking branch is created.
/// --single-branch
pub fn single_branch() -> FnOptionArg {
    optionarg::simple(SINGLE_BRANCH)
}

/// Clone only the history leading to the tip of a single branch, either specified by the --branch option or the primary branch remote’s HEAD points at.
/// Further fetches into the resulting repository will only update the remote-tracking branch for the branch this option was used for the initial cloning.
/// If the HEAD at the remote did not point at any branch when --single-branch clone was made, no remote-tracking branch is created.
/// --no-single-branch
pub fn no_single_branch() -> FnOptionArg {
    optionarg::simple(NO_SINGLE_BRANCH)
}

/// After the clone is created, initialize and clone submodules within based on the provided pathspec.
/// If no pathspec is provided, all submodules are initialized and cloned.
/// Submodules are initialized and cloned using their default settings.
/// The resulting clone has submodule.active set to the provided pathspec, or '.' (meaning all submodules) if no pathspec is provided.
/// This is equivalent to running git submodule update --init --recursive immediately after the clone is finished.
/// This option is ignored if the cloned repository does not have a worktree/checkout (i.e.
/// if any of --no-checkout/-n, --bare, or --mirror is given)
/// --recurse-submodules[=<pathspec>]
pub fn recurse_submodules(pathspec_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(RECURSE_SUBMODULES, pathspec_arg)
}

/// All submodules which are cloned will be shallow with a depth of 1.
/// --shallow-submodules
pub fn shallow_submodules() -> FnOptionArg {
    optionarg::simple(SHALLOW_SUBMODULES)
}

/// All submodules which are cloned will be shallow with a depth of 1.
/// --no-shallow-submodules
pub fn no_shallow_submodules() -> FnOptionArg {
    optionarg::simple(NO_SHALLOW_SUBMODULES)
}

/// Instead of placing the cloned repository where it is supposed to be, place the cloned repository at the specified directory,
/// then make a filesystem-agnostic Git symbolic link to there.
/// The result is Git repository can be separated from working tree.
/// --separate-git-dir=<git-dir>
pub fn separate_git_dir(git_dir_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(SEPARATE_GIT_DIR, git_dir_arg)
}

/// The number of submodules fetched at the same time.
/// Defaults to the submodule.fetchJobs option.
/// -j <n>, --jobs <n>
pub fn jobs(n_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(JOBS, n_arg)
}

/// The (possibly remote) repository to clone from.
/// <url>
pub fn repository(url: &str) -> FnOptionArg {
    optionarg::value_parameter(url)
}

/// The name of a new directory to clone into.
/// The "humanish" part of the source repository is used if no directory is explicitly given (repo for /path/to/repo.git and foo for host.xz:foo/.git).
/// Cloning into an existing directory is only allowed if the directory is empty.
/// <path>
pub fn directory(path: &str) -> FnOptionArg {
    optionarg::value_parameter(path)
}
