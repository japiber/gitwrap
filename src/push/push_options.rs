use crate::command_executor::{CommandExecutor, CommandOption};

/// Push all branches (i.e. refs under refs/heads/); cannot be used with other <refspec>.
/// --all
pub fn all_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--all"))
}

/// Remove remote branches that don’t have a local counterpart.
/// For example a remote branch tmp will be removed if a local branch with the same name doesn’t exist any more. This also respects refspecs, e.g.  git push --prune remote refs/heads/*:refs/tmp/* would make sure that remote refs/tmp/foo will be removed if refs/heads/foo doesn’t exist.
/// --prune
pub fn prune_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--prune"))
}

/// Instead of naming each ref to push, specifies that all refs under refs/ (which includes but is not limited to refs/heads/, refs/remotes/, and refs/tags/) be mirrored to the remote repository.
/// Newly created local refs will be pushed to the remote end, locally updated refs will be force updated on the remote end, and deleted refs will be removed from the remote end.
/// This is the default if the configuration option remote.<remote>.mirror is set.
/// --mirror
pub fn mirror_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--mirror"))
}

/// Do everything except actually send the updates.
/// -n, --dry-run
pub fn dry_run_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--dry-run"))
}

/// Produce machine-readable output.
/// The output status line for each ref will be tab-separated and sent to stdout instead of stderr.
/// The full symbolic names of the refs will be given.
/// --porcelain
pub fn porcelain_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--porcelain"))
}

/// All listed refs are deleted from the remote repository.
/// This is the same as prefixing all refs with a colon.
/// --delete
pub fn delete_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--delete"))
}

/// All refs under refs/tags are pushed, in addition to refspecs explicitly listed on the command line.
/// --tags
pub fn tags_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--tags"))
}

/// Push all the refs that would be pushed without this option, and also push annotated tags in refs/tags that are missing from the remote but are pointing at commit-ish that are reachable from the refs being pushed.
/// This can also be specified with configuration variable push.followTags.
/// For more information, see push.followTags in git-config(1).
/// --follow-tags
pub fn follow_tags_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--follow-tags"))
}

/// GPG-sign the push request to update refs on the receiving side, to allow it to be checked by the hooks and/or be logged.
/// If false or --no-signed, no signing will be attempted.
/// If true or --signed, the push will fail if the server does not support signed pushes.
/// If set to if-asked, sign if and only if the server supports signed pushes.
/// The push will also fail if the actual call to gpg --sign fails.
/// See git-receive-pack(1) for the details on the receiving end.
/// --signed
pub fn signed_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--signed"))
}

/// GPG-sign the push request to update refs on the receiving side, to allow it to be checked by the hooks and/or be logged.
/// If false or --no-signed, no signing will be attempted.
/// If true or --signed, the push will fail if the server does not support signed pushes.
/// If set to if-asked, sign if and only if the server supports signed pushes.
/// The push will also fail if the actual call to gpg --sign fails.
/// See git-receive-pack(1) for the details on the receiving end.
/// --no-signed
pub fn no_signed_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-signed"))
}

/// GPG-sign the push request to update refs on the receiving side, to allow it to be checked by the hooks and/or be logged.
/// If false or --no-signed, no signing will be attempted.
/// If true or --signed, the push will fail if the server does not support signed pushes.
/// If set to if-asked, sign if and only if the server supports signed pushes.
/// The push will also fail if the actual call to gpg --sign fails.
/// See git-receive-pack(1) for the details on the receiving end.
/// --sign=(true|false|if-asked)
pub fn sign_option(value :&str) -> CommandOption {
    if value.is_empty() {
        Box::new(|g: &mut CommandExecutor| g.add_option("--sign"))
    } else {
        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--sign={}", value)))
    }
}


/// Use an atomic transaction on the remote side if available.
/// Either all refs are updated, or on error, no refs are updated.
/// If the server does not support atomic pushes the push will fail.
/// --atomic
pub fn atomic_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--atomic"))
}

/// Use an atomic transaction on the remote side if available.
/// Either all refs are updated, or on error, no refs are updated.
/// If the server does not support atomic pushes the push will fail.
/// --no-atomic
pub fn no_atomic_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-atomic"))
}

/// Transmit the given string to the server, which passes them to the pre-receive as well as the post-receive hook.
/// The given string must not contain a NUL or LF character.
/// -o, --push-option
pub fn push_option_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--push-option"))
}

/// Path to the git-receive-pack program on the remote end.
/// Sometimes useful when pushing to a remote repository over ssh, and you do not have the program in a directory on the default $PATH.
/// --receive-pack=<git-receive-pack>
pub fn receive_pack_option(git_receive_pack_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--receive-pack={}", git_receive_pack_arg)))
}

/// Path to the git-receive-pack program on the remote end.
/// Sometimes useful when pushing to a remote repository over ssh, and you do not have the program in a directory on the default $PATH.
/// --exec=<git-receive-pack>
pub fn exec_option(git_receive_pack_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--exec={}", git_receive_pack_arg)))
}

/// Usually, the command refuses to update a remote ref that is not an ancestor of the local ref used to overwrite it.
/// Also, when --force-with-lease option is used, the command refuses to update a remote ref whose current value does not match what is expected.
/// -f, --force
pub fn force_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--force"))
}

/// This option is equivalent to the <repository> argument.
/// If both are specified, the command-line argument takes precedence.
/// --repo=<repository>
pub fn repo_option(repository_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--repo={}", repository_arg)))
}

/// For every branch that is up to date or successfully pushed, add upstream (tracking) reference, used by argument-less git-pull(1) and other commands.
/// For more information, see branch.<name>.merge in git-config(1).
/// -u, --set-upstream
pub fn set_upstream_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--set-upstream"))
}

/// These options are passed to git-send-pack(1).
/// A thin transfer significantly reduces the amount of sent data when the sender and receiver share many of the same objects in common.
/// The default is --thin.
/// --thin
pub fn thin_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--thin"))
}

/// These options are passed to git-send-pack(1).
/// A thin transfer significantly reduces the amount of sent data when the sender and receiver share many of the same objects in common.
/// The default is --thin.
/// --no-thin
pub fn no_thin_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-thin"))
}

/// Suppress all output, including the listing of updated refs, unless an error occurs.
/// Progress is not reported to the standard error stream.
/// -q, --quiet
pub fn quiet_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}

/// Run verbosely.
/// -v, --verbose
pub fn verbose_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--verbose"))
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--progress"))
}

/// May be used to make sure all submodule commits used by the revisions to be pushed are available on a remote-tracking branch.
/// If check is used Git will verify that all submodule commits that changed in the revisions to be pushed are available on at least one remote of the submodule.
/// If any commits are missing the push will be aborted and exit with non-zero status.
/// If on-demand is used all submodules that changed in the revisions to be pushed will be pushed.
/// If on-demand was not able to push all necessary revisions it will also be aborted and exit with non-zero status.
/// If only is used all submodules will be recursively pushed while the superproject is left unpushed.
/// A value of no or using --no-recurse-submodules can be used to override the push.recurseSubmodules configuration variable when no submodule recursion is required.
/// --no-recurse-submodules
pub fn no_recurse_submodules_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-recurse-submodules"))
}

/// May be used to make sure all submodule commits used by the revisions to be pushed are available on a remote-tracking branch.
/// If check is used Git will verify that all submodule commits that changed in the revisions to be pushed are available on at least one remote of the submodule.
/// If any commits are missing the push will be aborted and exit with non-zero status.
/// If on-demand is used all submodules that changed in the revisions to be pushed will be pushed.
/// If on-demand was not able to push all necessary revisions it will also be aborted and exit with non-zero status.
/// If only is used all submodules will be recursively pushed while the superproject is left unpushed.
/// A value of no or using --no-recurse-submodules can be used to override the push.recurseSubmodules configuration variable when no submodule recursion is required.
/// --recurse-submodules=(check|on-demand|only|no)
pub fn recurse_submodules_option(value :&str) -> CommandOption {
    if value.is_empty() {
        Box::new(|g: &mut CommandExecutor| g.add_option("--recurse-submodules"))
    } else {
        Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--recurse-submodules={}", value)))
    }
}


/// Toggle the pre-push hook (see githooks(5)).
/// The default is --verify, giving the hook a chance to prevent the push.
/// With --no-verify, the hook is bypassed completely.
/// --verify
pub fn verify_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--verify"))
}

/// Toggle the pre-push hook (see githooks(5)).
/// The default is --verify, giving the hook a chance to prevent the push.
/// With --no-verify, the hook is bypassed completely.
/// --no-verify
pub fn no_verify_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-verify"))
}

/// Use IPv4 addresses only, ignoring IPv6 addresses.
/// -4, --ipv4
pub fn ipv4_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ipv4"))
}

/// Use IPv6 addresses only, ignoring IPv4 addresses.
/// -6, --ipv6
pub fn ipv6_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ipv6"))
}