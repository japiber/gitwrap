use crate::command_executor::{CommandExecutor, CommandOption};

/// When the repository to clone from is on a local machine, this flag bypasses the normal 'Git aware' transport mechanism and clones the repository by making a copy of HEAD and everything under objects and refs directories. The files under .git/objects/ directory are hardlinked to save space when possible.
/// --local, -l
pub fn local_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--local"))
}

/// Force the cloning process from a repository on a local filesystem to copy the files under the .git/objects directory instead of using hardlinks. This may be desirable if you are trying to make a back-up of your repository.
/// --no-hardlinks
pub fn no_hardlinks_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-hardlinks"))
}

/// When the repository to clone is on the local machine, instead of using hard links, automatically setup .git/objects/info/alternates to share the objects with the source repository. The resulting repository starts out without any object of its own.
/// --shared, -s
pub fn shared_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--shared"))
}

/// Borrow the objects from reference repositories specified with the --reference options only to reduce network transfer, and stop borrowing from them after a clone is made by making necessary local copies of borrowed objects. This option can also be used when cloning locally from a repository that already borrows objects from another repository—the new repository will borrow objects from the same repository, and this option can be used to stop the borrowing.
/// --dissociate
pub fn dissociate_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--dissociate"))
}

/// Operate quietly. Progress is not reported to the standard error stream.
/// --quiet, -q
pub fn quiet_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}

/// Run verbosely. Does not affect the reporting of progress status to the standard error stream.
/// --verbose, -v
pub fn verbose_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--verbose"))
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified. This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--progress"))
}

/// No checkout of HEAD is performed after the clone is complete.
/// --no-checkout, -n
pub fn no_checkout_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-checkout"))
}

/// Make a bare Git repository. That is, instead of creating <directory> and placing the administrative files in <directory>/.git, make the <directory> itself the $GIT_DIR. This obviously implies the -n because there is nowhere to check out the working tree. Also the branch heads at the remote are copied directly to corresponding local branch heads, without mapping them to refs/remotes/origin/. When this option is used, neither remote-tracking branches nor the related configuration variables are created.
/// --bare
pub fn bare_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--bare"))
}

/// Set up a mirror of the source repository. This implies --bare. Compared to --bare, --mirror not only maps local branches of the source to local branches of the target, it maps all refs (including remote-tracking branches, notes etc.) and sets up a refspec configuration such that all these refs are overwritten by a git remote update in the target repository.
/// --mirror
pub fn mirror_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--mirror"))
}

/// Instead of using the remote name origin to keep track of the upstream repository, use <name>.
/// --origin <name>, -o <name>
pub fn origin_option(name_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--origin");
        g.add_option_string(format!("{}", name_arg ));
    })
}


/// Instead of pointing the newly created HEAD to the branch pointed to by the cloned repository’s HEAD, point to <name> branch instead. In a non-bare repository, this is the branch that will be checked out.  --branch can also take tags and detaches the HEAD at that commit in the resulting repository.
/// --branch <name>, -b <name>
pub fn branch_option(name_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--branch");
        g.add_option_string(format!("{}", name_arg ));
    })
}


/// When given, and the repository to clone from is accessed via ssh, this specifies a non-default path for the command run on the other end.
/// --upload-pack <upload-pack>, -u <upload-pack>
pub fn upload_pack_option(upload_pack_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--upload-pack");
        g.add_option_string(format!("{}", upload_pack_arg ));
    })
}


/// Specify the directory from which templates will be used; (See the 'TEMPLATE DIRECTORY' section of git-init(1).)
/// --template=<template_directory>
pub fn template_option(template_directory_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--template={}", template_directory_arg)))
}

/// Create a shallow clone with a history truncated to the specified number of commits. Implies --single-branch unless --no-single-branch is given to fetch the histories near the tips of all branches. If you want to clone submodules shallowly, also pass --shallow-submodules.
/// --depth <depth>
pub fn depth_option(depth_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--depth");
        g.add_option_string(format!("{}", depth_arg ));
    })
}


/// Create a shallow clone with a history after the specified time.
/// --shallow-since=<date>
pub fn shallow_since_option(date_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--shallow-since={}", date_arg)))
}

/// Create a shallow clone with a history, excluding commits reachable from a specified remote branch or tag. This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude_option(revision_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--shallow-exclude={}", revision_arg)))
}

/// Clone only the history leading to the tip of a single branch, either specified by the --branch option or the primary branch remote’s HEAD points at. Further fetches into the resulting repository will only update the remote-tracking branch for the branch this option was used for the initial cloning. If the HEAD at the remote did not point at any branch when --single-branch clone was made, no remote-tracking branch is created.
/// --single-branch
pub fn single_branch_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--single-branch"))
}

/// Clone only the history leading to the tip of a single branch, either specified by the --branch option or the primary branch remote’s HEAD points at. Further fetches into the resulting repository will only update the remote-tracking branch for the branch this option was used for the initial cloning. If the HEAD at the remote did not point at any branch when --single-branch clone was made, no remote-tracking branch is created.
/// --no-single-branch
pub fn no_single_branch_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-single-branch"))
}

/// After the clone is created, initialize and clone submodules within based on the provided pathspec. If no pathspec is provided, all submodules are initialized and cloned. Submodules are initialized and cloned using their default settings. The resulting clone has submodule.active set to the provided pathspec, or '.' (meaning all submodules) if no pathspec is provided. This is equivalent to running git submodule update --init --recursive immediately after the clone is finished. This option is ignored if the cloned repository does not have a worktree/checkout (i.e. if any of --no-checkout/-n, --bare, or --mirror is given)
/// --recurse-submodules[=<pathspec>]
pub fn recurse_submodules_option(pathspec_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--recurse-submodules={}", pathspec_arg)))
}

/// All submodules which are cloned will be shallow with a depth of 1.
/// --shallow-submodules
pub fn shallow_submodules_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--shallow-submodules"))
}

/// All submodules which are cloned will be shallow with a depth of 1.
/// --no-shallow-submodules
pub fn no_shallow_submodules_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-shallow-submodules"))
}

/// Instead of placing the cloned repository where it is supposed to be, place the cloned repository at the specified directory, then make a filesystem-agnostic Git symbolic link to there. The result is Git repository can be separated from working tree.
/// --separate-git-dir=<git-dir>
pub fn separate_git_dir_option(git_dir_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--separate-git-dir={}", git_dir_arg)))
}

/// The number of submodules fetched at the same time. Defaults to the submodule.fetchJobs option.
/// -j <n>, --jobs <n>
pub fn jobs_option(n_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--jobs");
        g.add_option_string(format!("{}", n_arg ));
    })
}
