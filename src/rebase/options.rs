// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// Starting point at which to create the new commits.
/// If the --onto option is not specified, the starting point is <upstream>.
/// May be any valid commit, and not just an existing branch name.
/// As a special case, you may use "A...B" as a shortcut for the merge base of A and B if there is exactly one merge base.
/// You can leave out at most one of A and B, in which case it defaults to HEAD.
/// --onto <newbase>
pub fn onto(newbase_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--onto", newbase_arg)
}

/// Set the starting point at which to create the new commits to the merge base of <upstream> <branch>.
/// Running git rebase --keep-base <upstream> <branch> is equivalent to running git rebase --onto <upstream>... <upstream>.
/// 
/// This option is useful in the case where one is developing a feature on top of an upstream branch.
/// While the feature is being worked on, the upstream branch may advance and it may not be the best idea to keep rebasing on top of the upstream but to keep the base commit as-is.
/// 
/// Although both this option and --fork-point find the merge base between <upstream> and <branch>, this option uses the merge base as the starting point on which new commits will be created, whereas --fork-point uses the merge base to determine the set of commits which will be rebased.
/// --keep-base
pub fn keep_base() -> FnOptionArg {
    optionarg::simple("--keep-base")
}

/// Restart the rebasing process after having resolved a merge conflict.
/// --continue
pub fn continue_rebase() -> FnOptionArg {
    optionarg::simple("--continue")
}

/// Abort the rebase operation and reset HEAD to the original branch.
/// If <branch> was provided when the rebase operation was started, then HEAD will be reset to <branch>.
/// Otherwise HEAD will be reset to where it was when the rebase operation was started.
/// --abort
pub fn abort() -> FnOptionArg {
    optionarg::simple("--abort")
}

/// Abort the rebase operation but HEAD is not reset back to the original branch.
/// The index and working tree are also left unchanged as a result.
/// If a temporary stash entry was created using --autostash, it will be saved to the stash list.
/// --quit
pub fn quit() -> FnOptionArg {
    optionarg::simple("--quit")
}

/// Use applying strategies to rebase (calling git-am internally).
/// This option may become a no-op in the future once the merge backend handles everything the apply one does.
/// --apply
pub fn apply() -> FnOptionArg {
    optionarg::simple("--apply")
}

/// How to handle commits that are not empty to start and are not clean cherry-picks of any upstream commit, but which become empty after rebasing (because they contain a subset of already upstream changes).
/// With drop (the default), commits that become empty are dropped.
/// With keep, such commits are kept.
/// With ask (implied by --interactive), the rebase will halt when an empty commit is applied allowing you to choose whether to drop it, edit files more, or just commit the empty changes.
/// Other options, like --exec, will use the default of drop unless -i/--interactive is explicitly specified.
/// 
/// Note that commits which start empty are kept (unless --no-keep-empty is specified), and commits which are clean cherry-picks (as determined by git log --cherry-mark ...) are detected and dropped as a preliminary step (unless --reapply-cherry-picks is passed).
/// --empty={drop,keep,ask}
pub fn empty(value: &str) -> FnOptionArg {
    optionarg::equal_optional("--empty", value)
}

/// Do not keep commits that start empty before the rebase (i.e. that do not change anything from its parent) in the result.
/// The default is to keep commits which start empty, since creating such commits requires passing the --allow-empty override flag to git commit, signifying that a user is very intentionally creating such a commit and thus wants to keep it.
/// 
/// Usage of this flag will probably be rare, since you can get rid of commits that start empty by just firing up an interactive rebase and removing the lines corresponding to the commits you don’t want.
/// This flag exists as a convenient shortcut, such as for cases where
/// external tools generate many empty commits and you want them all removed.
/// 
/// For commits which do not start empty but become empty after rebasing, see the --empty flag.
/// --no-keep-empty, --keep-empty
pub fn no_keep_empty() -> FnOptionArg {
    optionarg::simple("--no-keep-empty")
}

/// Do not keep commits that start empty before the rebase (i.e. that do not change anything from its parent) in the result.
/// The default is to keep commits which start empty, since creating such commits requires passing the --allow-empty override flag to git commit, signifying that a user is very intentionally creating such a commit and thus wants to keep it.
/// 
/// Usage of this flag will probably be rare, since you can get rid of commits that start empty by just firing up an interactive rebase and removing the lines corresponding to the commits you don’t want.
/// This flag exists as a convenient shortcut, such as for cases where
/// external tools generate many empty commits and you want them all removed.
/// 
/// For commits which do not start empty but become empty after rebasing, see the --empty flag.
/// --no-keep-empty, --keep-empty
pub fn keep_empty() -> FnOptionArg {
    optionarg::simple("--keep-empty")
}

/// Reapply all clean cherry-picks of any upstream commit instead of preemptively dropping them.
/// (If these commits then become empty after rebasing, because they contain a subset of already upstream changes, the behavior towards them is controlled by the --empty flag.)
/// 
/// By default (or if --no-reapply-cherry-picks is given), these commits will be automatically dropped.
/// Because this necessitates reading all upstream commits, this can be expensive in repos with a large number of upstream commits that need to be read.
/// When using the merge backend, warnings will be issued for each dropped commit (unless --quiet is given).
/// Advice will also be issued unless advice.skippedCherryPicks is set to false (see git-config(1)).
/// --reapply-cherry-picks allows rebase to forgo reading all upstream commits, potentially improving performance.
/// --reapply-cherry-picks, --no-reapply-cherry-picks
pub fn reapply_cherry_picks() -> FnOptionArg {
    optionarg::simple("--reapply-cherry-picks")
}

/// Reapply all clean cherry-picks of any upstream commit instead of preemptively dropping them.
/// (If these commits then become empty after rebasing, because they contain a subset of already upstream changes, the behavior towards them is controlled by the --empty flag.)
/// 
/// By default (or if --no-reapply-cherry-picks is given), these commits will be automatically dropped.
/// Because this necessitates reading all upstream commits, this can be expensive in repos with a large number of upstream commits that need to be read.
/// When using the merge backend, warnings will be issued for each dropped commit (unless --quiet is given).
/// Advice will also be issued unless advice.skippedCherryPicks is set to false (see git-config(1)).
/// --reapply-cherry-picks allows rebase to forgo reading all upstream commits, potentially improving performance.
/// --reapply-cherry-picks, --no-reapply-cherry-picks
pub fn no_reapply_cherry_picks() -> FnOptionArg {
    optionarg::simple("--no-reapply-cherry-picks")
}

/// No-op.
/// Rebasing commits with an empty message used to fail and this option would override that behavior, allowing commits with empty messages to be rebased.
/// Now commits with an empty message do not cause rebasing to halt.
/// --allow-empty-message
pub fn allow_empty_message() -> FnOptionArg {
    optionarg::simple("--allow-empty-message")
}

/// Restart the rebasing process by skipping the current patch.
/// --skip
pub fn skip() -> FnOptionArg {
    optionarg::simple("--skip")
}

/// Edit the todo list during an interactive rebase.
/// --edit-todo
pub fn edit_todo() -> FnOptionArg {
    optionarg::simple("--edit-todo")
}

/// Show the current patch in an interactive rebase or when rebase is stopped because of conflicts.
/// This is the equivalent of git show REBASE_HEAD.
/// --show-current-patch
pub fn show_current_patch() -> FnOptionArg {
    optionarg::simple("--show-current-patch")
}

/// Using merging strategies to rebase (default).
/// 
/// Note that a rebase merge works by replaying each commit from the working branch on top of the <upstream> branch.
/// Because of this, when a merge conflict happens, the side reported as ours is the so-far rebased series, starting with <upstream>, and theirs is the working branch.
/// In other words, the sides are swapped.
/// -m, --merge
pub fn merge() -> FnOptionArg {
    optionarg::simple("--merge")
}

/// Use the given merge strategy, instead of the default ort.
/// This implies --merge.
/// 
/// Because git rebase replays each commit from the working branch on top of the <upstream> branch using the given strategy, using the ours strategy simply empties all patches from the <branch>, which makes little sense.
/// -s <strategy>, --strategy=<strategy>
pub fn strategy(strategy_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--strategy", strategy_arg)
}

/// Pass the <strategy-option> through to the merge strategy.
/// This implies --merge and, if no strategy has been specified, -s ort.
/// Note the reversal of ours and theirs as noted above for the -m option.
/// -X <strategy-option>, --strategy-option=<strategy-option>
pub fn strategy_option(strategy_option_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--strategy-option", strategy_option_arg)
}

/// Allow the rerere mechanism to update the index with the result of auto-conflict resolution if possible.
/// --rerere-autoupdate, --no-rerere-autoupdate
pub fn rerere_autoupdate() -> FnOptionArg {
    optionarg::simple("--rerere-autoupdate")
}

/// Allow the rerere mechanism to update the index with the result of auto-conflict resolution if possible.
/// --rerere-autoupdate, --no-rerere-autoupdate
pub fn no_rerere_autoupdate() -> FnOptionArg {
    optionarg::simple("--no-rerere-autoupdate")
}

/// GPG-sign commits.
/// The keyid argument is optional and defaults to the committer identity; if specified, it must be stuck to the option without a space.
/// --no-gpg-sign is useful to countermand both commit.gpgSign configuration variable, and earlier --gpg-sign.
/// -S[<keyid>], --gpg-sign[=<keyid>], --no-gpg-sign
pub fn gpg_sign(keyid_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--gpg-sign", keyid_arg)
}

/// GPG-sign commits.
/// The keyid argument is optional and defaults to the committer identity; if specified, it must be stuck to the option without a space.
/// --no-gpg-sign is useful to countermand both commit.gpgSign configuration variable, and earlier --gpg-sign.
/// -S[<keyid>], --gpg-sign[=<keyid>], --no-gpg-sign
pub fn no_gpg_sign() -> FnOptionArg {
    optionarg::simple("--no-gpg-sign")
}

/// Be quiet.
/// Implies --no-stat.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple("--quiet")
}

/// Be verbose.
/// Implies --stat.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    optionarg::simple("--verbose")
}

/// Show a diffstat of what changed upstream since the last rebase.
/// The diffstat is also controlled by the configuration option rebase.stat.
/// --stat
pub fn stat() -> FnOptionArg {
    optionarg::simple("--stat")
}

/// Do not show a diffstat as part of the rebase process.
/// -n, --no-stat
pub fn no_stat() -> FnOptionArg {
    optionarg::simple("--no-stat")
}

/// This option bypasses the pre-rebase hook.
/// See also githooks(5).
/// --no-verify
pub fn no_verify() -> FnOptionArg {
    optionarg::simple("--no-verify")
}

/// Allows the pre-rebase hook to run, which is the default.
/// This option can be used to override --no-verify.
/// See also githooks(5).
/// --verify
pub fn verify() -> FnOptionArg {
    optionarg::simple("--verify")
}

/// Ensure at least <n> lines of surrounding context match before and after each change.
/// When fewer lines of surrounding context exist they all must match.
/// By default no context is ever ignored.
/// Implies --apply.
/// -C<n>
pub fn ensure_context(n_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("-C", n_arg)
}

/// Individually replay all rebased commits instead of fast-forwarding over the unchanged ones.
/// This ensures that the entire history of the rebased branch is composed of new commits.
/// 
/// You may find this helpful after reverting a topic branch merge, as this option recreates the topic branch with fresh commits so it can be remerged successfully without needing to "revert the reversion" (see the revert-a-faulty-merge How-To[1] for details).
/// --no-ff, --force-rebase, -f
pub fn no_ff() -> FnOptionArg {
    optionarg::simple("--no-ff")
}

/// Individually replay all rebased commits instead of fast-forwarding over the unchanged ones.
/// This ensures that the entire history of the rebased branch is composed of new commits.
/// 
/// You may find this helpful after reverting a topic branch merge, as this option recreates the topic branch with fresh commits so it can be remerged successfully without needing to "revert the reversion" (see the revert-a-faulty-merge How-To[1] for details).
/// --no-ff, --force-rebase, -f
pub fn force_rebase() -> FnOptionArg {
    optionarg::simple("--force-rebase")
}

/// Use reflog to find a better common ancestor between <upstream> and <branch> when calculating which commits have been introduced by <branch>.
/// 
/// When --fork-point is active, fork_point will be used instead of <upstream> to calculate the set of commits to rebase, where fork_point is the result of git merge-base --fork-point <upstream> <branch> command (see git-merge-base(1)).
/// If fork_point ends up being empty, the <upstream> will be used as a fallback.
/// 
/// If <upstream> is given on the command line, then the default is --no-fork-point, otherwise the default is --fork-point.
/// See also rebase.forkpoint in git-config(1).
/// 
/// If your branch was based on <upstream> but <upstream> was rewound and your branch contains commits which were dropped, this option can be used with --keep-base in order to drop those commits from your branch.
/// --fork-point, --no-fork-point
pub fn fork_point() -> FnOptionArg {
    optionarg::simple("--fork-point")
}

/// Use reflog to find a better common ancestor between <upstream> and <branch> when calculating which commits have been introduced by <branch>.
/// 
/// When --fork-point is active, fork_point will be used instead of <upstream> to calculate the set of commits to rebase, where fork_point is the result of git merge-base --fork-point <upstream> <branch> command (see git-merge-base(1)).
/// If fork_point ends up being empty, the <upstream> will be used as a fallback.
/// 
/// If <upstream> is given on the command line, then the default is --no-fork-point, otherwise the default is --fork-point.
/// See also rebase.forkpoint in git-config(1).
/// 
/// If your branch was based on <upstream> but <upstream> was rewound and your branch contains commits which were dropped, this option can be used with --keep-base in order to drop those commits from your branch.
/// --fork-point, --no-fork-point
pub fn no_fork_point() -> FnOptionArg {
    optionarg::simple("--no-fork-point")
}

/// Ignore whitespace differences when trying to reconcile differences.
/// Currently, each backend implements an approximation of this behavior:
/// 
/// apply backend: When applying a patch, ignore changes in whitespace in context lines.
/// Unfortunately, this means that if the "old" lines being replaced by the patch differ only in whitespace from the existing file, you will get a merge conflict instead of a successful patch application.
/// 
/// merge backend: Treat lines with only whitespace changes as unchanged when merging.
/// Unfortunately, this means that any patch hunks that were intended to modify whitespace and nothing else will be dropped, even if the other side had no changes that conflicted.
/// --ignore-whitespace
pub fn ignore_whitespace() -> FnOptionArg {
    optionarg::simple("--ignore-whitespace")
}

/// This flag is passed to the git apply program (see git-apply(1)) that applies the patch.
/// Implies --apply.
/// --whitespace=<option>
pub fn whitespace(option_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--whitespace", option_arg)
}

/// Instead of using the current time as the committer date, use the author date of the commit being rebased as the committer date.
/// This option implies --force-rebase.
/// --committer-date-is-author-date
pub fn committer_date_is_author_date() -> FnOptionArg {
    optionarg::simple("--committer-date-is-author-date")
}

/// Instead of using the author date of the original commit, use the current time as the author date of the rebased commit.
/// This option implies --force-rebase.
/// --ignore-date, --reset-author-date
pub fn ignore_date() -> FnOptionArg {
    optionarg::simple("--ignore-date")
}

/// Instead of using the author date of the original commit, use the current time as the author date of the rebased commit.
/// This option implies --force-rebase.
/// --ignore-date, --reset-author-date
pub fn reset_author_date() -> FnOptionArg {
    optionarg::simple("--reset-author-date")
}

/// Add a Signed-off-by trailer to all the rebased commits.
/// Note that if --interactive is given then only commits marked to be picked, edited or reworded will have the trailer added.
/// --signoff
pub fn signoff() -> FnOptionArg {
    optionarg::simple("--signoff")
}

/// Make a list of the commits which are about to be rebased.
/// Let the user edit that list before rebasing.
/// This mode can also be used to split commits (see SPLITTING COMMITS below).
/// 
/// The commit list format can be changed by setting the configuration option rebase.instructionFormat.
/// A customized instruction format will automatically have the long commit hash prepended to the format.
/// -i, --interactive
pub fn interactive() -> FnOptionArg {
    optionarg::simple("--interactive")
}

/// By default, a rebase will simply drop merge commits from the todo list, and put the rebased commits into a single, linear branch.
/// With --rebase-merges, the rebase will instead try to preserve the branching structure within the commits that are to be rebased, by recreating the merge commits.
/// Any resolved merge conflicts or manual amendments in these merge commits will have to be resolved/re-applied manually.
/// 
/// By default, or when no-rebase-cousins was specified, commits which do not have <upstream> as direct ancestor will keep their original branch point, i.e. commits that would be excluded by git-log(1)'s --ancestry-path option will keep their original ancestry by default.
/// If the rebase-cousins mode is turned on, such commits are instead rebased onto <upstream> (or <onto>, if specified).
/// 
/// It is currently only possible to recreate the merge commits using the ort merge strategy; different merge strategies can be used only via explicit exec git merge -s <strategy> [...]  commands.
/// -r, --rebase-merges[=(rebase-cousins|no-rebase-cousins)]
pub fn rebase_merges(value: &str) -> FnOptionArg {
    optionarg::equal_optional("--rebase-merges", value)
}

/// Append "exec <cmd>" after each line creating a commit in the final history.
/// <cmd> will be interpreted as one or more shell commands.
/// Any command that fails will interrupt the rebase, with exit code 1.
/// -x <cmd>, --exec <cmd>
pub fn exec(cmd_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--exec", cmd_arg)
}

/// Rebase all commits reachable from <branch>, instead of limiting them with an <upstream>.
/// This allows you to rebase the root commit(s) on a branch.
/// When used with --onto, it will skip changes already contained in <newbase> (instead of <upstream>) whereas without
/// --onto it will operate on every change.
/// --root
pub fn root() -> FnOptionArg {
    optionarg::simple("--root")
}

/// When the commit log message begins with "squash! ..." or "fixup! ..." or "amend! ...", and there is already a commit in the todo list that matches the same ..., automatically modify the todo list of rebase -i, so that the commit marked for squashing comes right after the commit to be modified, and change the action of the moved commit from pick to squash or fixup or fixup -C respectively.
/// A commit matches the ...  if the commit subject matches, or if the ...  refers to the commit’s hash.
/// As a fall-back, partial matches of the commit subject work, too.
/// The recommended way to create fixup/amend/squash commits is by using the --fixup, --fixup=amend: or --fixup=reword: and --squash options respectively of git-commit(1).
/// 
/// If the --autosquash option is enabled by default using the configuration variable rebase.autoSquash, this option can be used to override and disable this setting.
/// --autosquash, --no-autosquash
pub fn autosquash() -> FnOptionArg {
    optionarg::simple("--autosquash")
}

/// When the commit log message begins with "squash! ..." or "fixup! ..." or "amend! ...", and there is already a commit in the todo list that matches the same ..., automatically modify the todo list of rebase -i, so that the commit marked for squashing comes right after the commit to be modified, and change the action of the moved commit from pick to squash or fixup or fixup -C respectively.
/// A commit matches the ...  if the commit subject matches, or if the ...  refers to the commit’s hash.
/// As a fall-back, partial matches of the commit subject work, too.
/// The recommended way to create fixup/amend/squash commits is by using the --fixup, --fixup=amend: or --fixup=reword: and --squash options respectively of git-commit(1).
/// 
/// If the --autosquash option is enabled by default using the configuration variable rebase.autoSquash, this option can be used to override and disable this setting.
/// --autosquash, --no-autosquash
pub fn no_autosquash() -> FnOptionArg {
    optionarg::simple("--no-autosquash")
}

/// Automatically create a temporary stash entry before the operation begins, and apply it after the operation ends.
/// This means that you can run rebase on a dirty worktree.
/// However, use with care: the final stash application after a successful rebase might result in non-trivial conflicts.
/// --autostash, --no-autostash
pub fn autostash() -> FnOptionArg {
    optionarg::simple("--autostash")
}

/// Automatically create a temporary stash entry before the operation begins, and apply it after the operation ends.
/// This means that you can run rebase on a dirty worktree.
/// However, use with care: the final stash application after a successful rebase might result in non-trivial conflicts.
/// --autostash, --no-autostash
pub fn no_autostash() -> FnOptionArg {
    optionarg::simple("--no-autostash")
}

/// Automatically reschedule exec commands that failed.
/// This only makes sense in interactive mode (or when an --exec option was provided).
/// 
/// Even though this option applies once a rebase is started, it’s set for the whole rebase at the start based on either the rebase.rescheduleFailedExec configuration (see git-config(1) or "CONFIGURATION" below) or whether this option is provided.
/// Otherwise an explicit --no-reschedule-failed-exec at the start would be overridden by the presence of rebase.rescheduleFailedExec=true configuration.
/// --reschedule-failed-exec, --no-reschedule-failed-exec
pub fn reschedule_failed_exec() -> FnOptionArg {
    optionarg::simple("--reschedule-failed-exec")
}

/// Automatically reschedule exec commands that failed.
/// This only makes sense in interactive mode (or when an --exec option was provided).
/// 
/// Even though this option applies once a rebase is started, it’s set for the whole rebase at the start based on either the rebase.rescheduleFailedExec configuration (see git-config(1) or "CONFIGURATION" below) or whether this option is provided.
/// Otherwise an explicit --no-reschedule-failed-exec at the start would be overridden by the presence of rebase.rescheduleFailedExec=true configuration.
/// --reschedule-failed-exec, --no-reschedule-failed-exec
pub fn no_reschedule_failed_exec() -> FnOptionArg {
    optionarg::simple("--no-reschedule-failed-exec")
}
