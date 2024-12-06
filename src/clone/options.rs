// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// When the repository to clone from is on a local machine, this flag bypasses the normal 'Git aware' transport mechanism and clones the repository by making a copy of HEAD and everything under objects and refs directories. The files under .git/objects/ directory are hardlinked to save space when possible.
/// --local, -l
pub fn local() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--local");
    })
}

/// Force the cloning process from a repository on a local filesystem to copy the files under the .git/objects directory instead of using hardlinks. This may be desirable if you are trying to make a back-up of your repository.
/// --no-hardlinks
pub fn no_hardlinks() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-hardlinks");
    })
}

/// When the repository to clone is on the local machine, instead of using hard links, automatically setup .git/objects/info/alternates to share the objects with the source repository. The resulting repository starts out without any object of its own.
/// --shared, -s
pub fn shared() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--shared");
    })
}

/// Borrow the objects from reference repositories specified with the --reference options only to reduce network transfer, and stop borrowing from them after a clone is made by making necessary local copies of borrowed objects. This option can also be used when cloning locally from a repository that already borrows objects from another repository—the new repository will borrow objects from the same repository, and this option can be used to stop the borrowing.
/// --dissociate
pub fn dissociate() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--dissociate");
    })
}

/// Operate quietly. Progress is not reported to the standard error stream.
/// --quiet, -q
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// Run verbosely. Does not affect the reporting of progress status to the standard error stream.
/// --verbose, -v
pub fn verbose() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verbose");
    })
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified. This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--progress");
    })
}

/// No checkout of HEAD is performed after the clone is complete.
/// --no-checkout, -n
pub fn no_checkout() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-checkout");
    })
}

/// Make a bare Git repository. That is, instead of creating <directory> and placing the administrative files in <directory>/.git, make the <directory> itself the $GIT_DIR. This obviously implies the -n because there is nowhere to check out the working tree. Also the branch heads at the remote are copied directly to corresponding local branch heads, without mapping them to refs/remotes/origin/. When this option is used, neither remote-tracking branches nor the related configuration variables are created.
/// --bare
pub fn bare() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--bare");
    })
}

/// Set up a mirror of the source repository. This implies --bare. Compared to --bare, --mirror not only maps local branches of the source to local branches of the target, it maps all refs (including remote-tracking branches, notes etc.) and sets up a refspec configuration such that all these refs are overwritten by a git remote update in the target repository.
/// --mirror
pub fn mirror() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--mirror");
    })
}

/// Instead of using the remote name origin to keep track of the upstream repository, use <name>.
/// --origin <name>, -o <name>
pub fn origin(name_arg: &str) -> FnOptionArg {
    let l_name_arg = String::from(name_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--origin");
        cmd.arg(l_name_arg.as_str());
    })
}

/// Instead of pointing the newly created HEAD to the branch pointed to by the cloned repository’s HEAD, point to <name> branch instead. In a non-bare repository, this is the branch that will be checked out.  --branch can also take tags and detaches the HEAD at that commit in the resulting repository.
/// --branch <name>, -b <name>
pub fn branch(name_arg: &str) -> FnOptionArg {
    let l_name_arg = String::from(name_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--branch");
        cmd.arg(l_name_arg.as_str());
    })
}

/// When given, and the repository to clone from is accessed via ssh, this specifies a non-default path for the command run on the other end.
/// --upload-pack <upload-pack>, -u <upload-pack>
pub fn upload_pack(upload_pack_arg: &str) -> FnOptionArg {
    let l_upload_pack_arg = String::from(upload_pack_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--upload-pack");
        cmd.arg(l_upload_pack_arg.as_str());
    })
}

/// Specify the directory from which templates will be used; (See the 'TEMPLATE DIRECTORY' section of git-init(1).)
/// --template=<template_directory>
pub fn template(template_directory_arg: &str) -> FnOptionArg {
    let l_template_directory_arg = format!("--template={}", template_directory_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_template_directory_arg.as_str());
    })
}

/// Create a shallow clone with a history truncated to the specified number of commits. Implies --single-branch unless --no-single-branch is given to fetch the histories near the tips of all branches. If you want to clone submodules shallowly, also pass --shallow-submodules.
/// --depth <depth>
pub fn depth(depth_arg: &str) -> FnOptionArg {
    let l_depth_arg = String::from(depth_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--depth");
        cmd.arg(l_depth_arg.as_str());
    })
}

/// Create a shallow clone with a history after the specified time.
/// --shallow-since=<date>
pub fn shallow_since(date_arg: &str) -> FnOptionArg {
    let l_date_arg = format!("--shallow-since={}", date_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_date_arg.as_str());
    })
}

/// Create a shallow clone with a history, excluding commits reachable from a specified remote branch or tag. This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg: &str) -> FnOptionArg {
    let l_revision_arg = format!("--shallow-exclude={}", revision_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_revision_arg.as_str());
    })
}

/// Clone only the history leading to the tip of a single branch, either specified by the --branch option or the primary branch remote’s HEAD points at. Further fetches into the resulting repository will only update the remote-tracking branch for the branch this option was used for the initial cloning. If the HEAD at the remote did not point at any branch when --single-branch clone was made, no remote-tracking branch is created.
/// --single-branch
pub fn single_branch() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--single-branch");
    })
}

/// Clone only the history leading to the tip of a single branch, either specified by the --branch option or the primary branch remote’s HEAD points at. Further fetches into the resulting repository will only update the remote-tracking branch for the branch this option was used for the initial cloning. If the HEAD at the remote did not point at any branch when --single-branch clone was made, no remote-tracking branch is created.
/// --no-single-branch
pub fn no_single_branch() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-single-branch");
    })
}

/// After the clone is created, initialize and clone submodules within based on the provided pathspec. If no pathspec is provided, all submodules are initialized and cloned. Submodules are initialized and cloned using their default settings. The resulting clone has submodule.active set to the provided pathspec, or '.' (meaning all submodules) if no pathspec is provided. This is equivalent to running git submodule update --init --recursive immediately after the clone is finished. This option is ignored if the cloned repository does not have a worktree/checkout (i.e. if any of --no-checkout/-n, --bare, or --mirror is given)
/// --recurse-submodules[=<pathspec>]
pub fn recurse_submodules(pathspec_arg: &str) -> FnOptionArg {
    let l_pathspec_arg = format!("--recurse-submodules={}", pathspec_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_pathspec_arg.as_str());
    })
}

/// All submodules which are cloned will be shallow with a depth of 1.
/// --shallow-submodules
pub fn shallow_submodules() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--shallow-submodules");
    })
}

/// All submodules which are cloned will be shallow with a depth of 1.
/// --no-shallow-submodules
pub fn no_shallow_submodules() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-shallow-submodules");
    })
}

/// Instead of placing the cloned repository where it is supposed to be, place the cloned repository at the specified directory, then make a filesystem-agnostic Git symbolic link to there. The result is Git repository can be separated from working tree.
/// --separate-git-dir=<git-dir>
pub fn separate_git_dir(git_dir_arg: &str) -> FnOptionArg {
    let l_git_dir_arg = format!("--separate-git-dir={}", git_dir_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_git_dir_arg.as_str());
    })
}

/// The number of submodules fetched at the same time. Defaults to the submodule.fetchJobs option.
/// -j <n>, --jobs <n>
pub fn jobs(n_arg: &str) -> FnOptionArg {
    let l_n_arg = String::from(n_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--jobs");
        cmd.arg(l_n_arg.as_str());
    })
}