// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// This is passed to both underlying git-fetch to squelch reporting of during transfer, and underlying git-merge to squelch output during merging.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple("--quiet")
}

/// Pass --verbose to git-fetch and git-merge.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    optionarg::simple("--verbose")
}

/// This option controls if new commits of all populated submodules should be fetched too.
/// --recurse-submodules[=yes|on-demand|no]
pub fn recurse_submodules(value: &str) -> FnOptionArg {
    optionarg::equal_optional("--recurse-submodules", value)
}

/// This option controls if new commits of all populated submodules should be fetched too.
/// --no-recurse-submodules[=yes|on-demand|no]
pub fn no_recurse_submodules(value: &str) -> FnOptionArg {
    optionarg::equal_optional("--no-recurse-submodules", value)
}

/// Perform the merge and commit the result.
/// This option can be used to override --no-commit.
/// --commit
pub fn commit() -> FnOptionArg {
    optionarg::simple("--commit")
}

/// With --no-commit perform the merge but pretend the merge failed and do not autocommit, to give the user a chance to inspect and further tweak the merge result before committing.
/// --no-commit
pub fn no_commit() -> FnOptionArg {
    optionarg::simple("--no-commit")
}

/// Invoke an editor before committing successful mechanical merge to further edit the auto-generated merge message, so that the user can explain and justify the merge.
/// --edit, -e
pub fn edit() -> FnOptionArg {
    optionarg::simple("--edit")
}

/// The --no-edit option can be used to accept the auto-generated message (this is generally discouraged).
/// --no-edit
pub fn no_edit() -> FnOptionArg {
    optionarg::simple("--no-edit")
}

/// When the merge resolves as a fast-forward, only update the branch pointer, without creating a merge commit.
/// This is the default behavior.
/// --ff
pub fn ff() -> FnOptionArg {
    optionarg::simple("--ff")
}

/// Create a merge commit even when the merge resolves as a fast-forward.
/// This is the default behaviour when merging an annotated (and possibly signed) tag.
/// --no-ff
pub fn no_ff() -> FnOptionArg {
    optionarg::simple("--no-ff")
}

/// Refuse to merge and exit with a non-zero status unless the current HEAD is already up-to-date or the merge can be resolved as a fast-forward.
/// --ff-only
pub fn ff_only() -> FnOptionArg {
    optionarg::simple("--ff-only")
}

/// In addition to branch names, populate the log message with one-line descriptions from at most <n> actual commits that are being merged.
/// See also git-fmt-merge-msg(1).
/// --log[=<n>]
pub fn log(n_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--log", n_arg)
}

/// With --no-log do not list one-line descriptions from the actual commits being merged.
/// --no-log
pub fn no_log() -> FnOptionArg {
    optionarg::simple("--no-log")
}

/// Show a diffstat at the end of the merge.
/// The diffstat is also controlled by the configuration option merge.stat.
/// --stat
pub fn stat() -> FnOptionArg {
    optionarg::simple("--stat")
}

/// With -n or --no-stat do not show a diffstat at the end of the merge.
/// -n, --no-stat
pub fn no_stat() -> FnOptionArg {
    optionarg::simple("--no-stat")
}

/// Produce the working tree and index state as if a real merge happened (except for the merge information), but do not actually make a commit, move the HEAD, or record $GIT_DIR/MERGE_HEAD (to cause the next git commit command to create a merge commit).
/// This allows you to create a single commit on top of the current branch whose effect is the same as merging another branch (or more in case of an octopus).
/// --squash
pub fn squash() -> FnOptionArg {
    optionarg::simple("--squash")
}

/// With --no-squash perform the merge and commit the result.
/// This option can be used to override --squash.
/// --no-squash
pub fn no_squash() -> FnOptionArg {
    optionarg::simple("--no-squash")
}

/// Use the given merge strategy; can be supplied more than once to specify them in the order they should be tried.
/// If there is no -s option, a built-in list of strategies is used instead (git merge-recursive when merging a single head, git merge-octopus otherwise).
/// -s <strategy>, --strategy=<strategy>
pub fn strategy(strategy_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--strategy", strategy_arg)
}

/// Pass merge strategy specific option through to the merge strategy.
/// -X <option>, --strategy-option=<option>
pub fn strategy_option(option_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--strategy-option", option_arg)
}

/// Verify that the tip commit of the side branch being merged is signed with a valid key, i.e. a key that has a valid uid: in the default trust model, this means the signing key has been signed by a trusted key.
/// --verify-signatures
pub fn verify_signatures() -> FnOptionArg {
    optionarg::simple("--verify-signatures")
}

/// Verify that the tip commit of the side branch being merged is signed with a valid key, i.e. a key that has a valid uid: in the default trust model, this means the signing key has been signed by a trusted key.
/// --no-verify-signatures
pub fn no_verify_signatures() -> FnOptionArg {
    optionarg::simple("--no-verify-signatures")
}

/// By default, git merge command refuses to merge histories that do not share a common ancestor.
/// This option can be used to override this safety when merging histories of two projects that started their lives independently.
/// As that is a very rare occasion, no configuration variable to enable this by default exists and will not be added.
/// --allow-unrelated-histories
pub fn allow_unrelated_histories() -> FnOptionArg {
    optionarg::simple("--allow-unrelated-histories")
}

/// When true, rebase the current branch on top of the upstream branch after fetching.
/// If there is a remote-tracking branch corresponding to the upstream branch and the upstream branch was rebased since last fetched, the rebase uses that information to avoid rebasing non-local changes.
/// -r, --rebase[=false|true|preserve|interactive]
pub fn rebase(value: &str) -> FnOptionArg {
    optionarg::equal_optional("--rebase", value)
}

/// Override earlier --rebase.
/// --no-rebase
pub fn no_rebase() -> FnOptionArg {
    optionarg::simple("--no-rebase")
}

/// Before starting rebase, stash local modifications away if needed, and apply the stash when done.
/// --autostash
pub fn autostash() -> FnOptionArg {
    optionarg::simple("--autostash")
}

/// --no-autostash is useful to override the rebase.autoStash configuration variable.
/// --no-autostash
pub fn no_autostash() -> FnOptionArg {
    optionarg::simple("--no-autostash")
}

/// Fetch all remotes.
/// --all
pub fn all() -> FnOptionArg {
    optionarg::simple("--all")
}

/// Append ref names and object names of fetched refs to the existing contents of .git/FETCH_HEAD.
/// Without this option old data in .git/FETCH_HEAD will be overwritten.
/// -a, --append
pub fn append() -> FnOptionArg {
    optionarg::simple("--append")
}

/// Limit fetching to the specified number of commits from the tip of each remote branch history.
/// If fetching to a shallow repository created by git clone with --depth=<depth> option (see git-clone(1)), deepen or shorten the history to the specified number of commits.
/// Tags for the deepened commits are not fetched.
/// --depth=<depth>
pub fn depth(depth_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--depth", depth_arg)
}

/// Similar to --depth, except it specifies the number of commits from the current shallow boundary instead of from the tip of each remote branch history.
/// --deepen=<depth>
pub fn deepen(depth_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--deepen", depth_arg)
}

/// Deepen or shorten the history of a shallow repository to include all reachable commits after <date>.
/// --shallow-since=<date>
pub fn shallow_since(date_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--shallow-since", date_arg)
}

/// Deepen or shorten the history of a shallow repository to exclude commits reachable from a specified remote branch or tag.
/// This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--shallow-exclude", revision_arg)
}

/// If the source repository is complete, convert a shallow repository to a complete one, removing all the limitations imposed by shallow repositories.
/// If the source repository is shallow, fetch as much as possible so that the current repository has the same history as the source repository.
/// --unshallow
pub fn unshallow() -> FnOptionArg {
    optionarg::simple("--unshallow")
}

/// By default when fetching from a shallow repository, git fetch refuses refs that require updating .git/shallow.
/// This option updates .git/shallow and accept such refs.
/// --update-shallow
pub fn update_shallow() -> FnOptionArg {
    optionarg::simple("--update-shallow")
}

/// When git fetch is used with <rbranch>:<lbranch> refspec, it refuses to update the local branch <lbranch> unless the remote branch <rbranch> it fetches is a descendant of <lbranch>.
/// This option overrides that check.
/// -f, --force
pub fn force() -> FnOptionArg {
    optionarg::simple("--force")
}

/// Keep downloaded pack.
/// -k, --keep
pub fn keep() -> FnOptionArg {
    optionarg::simple("--keep")
}

/// By default, tags that point at objects that are downloaded from the remote repository are fetched and stored locally.
/// This option disables this automatic tag following.
/// The default behavior for a remote may be specified with the remote.<name>.tagOpt setting.
/// --no-tags
pub fn no_tags() -> FnOptionArg {
    optionarg::simple("--no-tags")
}

/// By default git fetch refuses to update the head which corresponds to the current branch.
/// This flag disables the check.
/// This is purely for the internal use for git pull to communicate with git fetch, and unless you are implementing your own Porcelain you are not supposed to use it.
/// -u, --update-head-ok
pub fn update_head_ok() -> FnOptionArg {
    optionarg::simple("--update-head-ok")
}

/// When given, and the repository to fetch from is handled by git fetch-pack, --exec=<upload-pack> is passed to the command to specify non-default path for the command run on the other end.
/// --upload-pack <upload-pack>
pub fn upload_pack(upload_pack_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--upload-pack", upload_pack_arg)
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> FnOptionArg {
    optionarg::simple("--progress")
}

/// Use IPv4 addresses only, ignoring IPv6 addresses.
/// -4, --ipv4
pub fn ipv4() -> FnOptionArg {
    optionarg::simple("--ipv4")
}

/// Use IPv6 addresses only, ignoring IPv4 addresses.
/// -6, --ipv6
pub fn ipv6() -> FnOptionArg {
    optionarg::simple("--ipv6")
}
