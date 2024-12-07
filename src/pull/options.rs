// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// This is passed to both underlying git-fetch to squelch reporting of during transfer, and underlying git-merge to squelch output during merging.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// Pass --verbose to git-fetch and git-merge.
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verbose");
    })
}

/// This option controls if new commits of all populated submodules should be fetched too.
/// --recurse-submodules[=yes|on-demand|no]
pub fn recurse_submodules(value: &str) -> FnOptionArg {
    let l_value = if value.is_empty() {
        String::from("--recurse-submodules")
    } else {
        format!("--recurse-submodules={}", value)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}

/// This option controls if new commits of all populated submodules should be fetched too.
/// --no-recurse-submodules[=yes|on-demand|no]
pub fn no_recurse_submodules(value: &str) -> FnOptionArg {
    let l_value = if value.is_empty() {
        String::from("--no-recurse-submodules")
    } else {
        format!("--no-recurse-submodules={}", value)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}

/// Perform the merge and commit the result.
/// This option can be used to override --no-commit.
/// --commit
pub fn commit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--commit");
    })
}

/// With --no-commit perform the merge but pretend the merge failed and do not autocommit, to give the user a chance to inspect and further tweak the merge result before committing.
/// --no-commit
pub fn no_commit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-commit");
    })
}

/// Invoke an editor before committing successful mechanical merge to further edit the auto-generated merge message, so that the user can explain and justify the merge.
/// --edit, -e
pub fn edit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--edit");
    })
}

/// The --no-edit option can be used to accept the auto-generated message (this is generally discouraged).
/// --no-edit
pub fn no_edit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-edit");
    })
}

/// When the merge resolves as a fast-forward, only update the branch pointer, without creating a merge commit.
/// This is the default behavior.
/// --ff
pub fn ff() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ff");
    })
}

/// Create a merge commit even when the merge resolves as a fast-forward.
/// This is the default behaviour when merging an annotated (and possibly signed) tag.
/// --no-ff
pub fn no_ff() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-ff");
    })
}

/// Refuse to merge and exit with a non-zero status unless the current HEAD is already up-to-date or the merge can be resolved as a fast-forward.
/// --ff-only
pub fn ff_only() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ff-only");
    })
}

/// In addition to branch names, populate the log message with one-line descriptions from at most <n> actual commits that are being merged.
/// See also git-fmt-merge-msg(1).
/// --log[=<n>]
pub fn log(n_arg: &str) -> FnOptionArg {
    let l_n_arg = format!("--log={}", n_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_n_arg.as_str());
    })
}

/// With --no-log do not list one-line descriptions from the actual commits being merged.
/// --no-log
pub fn no_log() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-log");
    })
}

/// Show a diffstat at the end of the merge.
/// The diffstat is also controlled by the configuration option merge.stat.
/// --stat
pub fn stat() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--stat");
    })
}

/// With -n or --no-stat do not show a diffstat at the end of the merge.
/// -n, --no-stat
pub fn no_stat() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-stat");
    })
}

/// Produce the working tree and index state as if a real merge happened (except for the merge information), but do not actually make a commit, move the HEAD, or record $GIT_DIR/MERGE_HEAD (to cause the next git commit command to create a merge commit).
/// This allows you to create a single commit on top of the current branch whose effect is the same as merging another branch (or more in case of an octopus).
/// --squash
pub fn squash() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--squash");
    })
}

/// With --no-squash perform the merge and commit the result.
/// This option can be used to override --squash.
/// --no-squash
pub fn no_squash() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-squash");
    })
}

/// Use the given merge strategy; can be supplied more than once to specify them in the order they should be tried.
/// If there is no -s option, a built-in list of strategies is used instead (git merge-recursive when merging a single head, git merge-octopus otherwise).
/// -s <strategy>, --strategy=<strategy>
pub fn strategy(strategy_arg: &str) -> FnOptionArg {
    let l_strategy_arg = format!("--strategy={}", strategy_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_strategy_arg.as_str());
    })
}

/// Pass merge strategy specific option through to the merge strategy.
/// -X <option>, --strategy-option=<option>
pub fn strategy_option(option_arg: &str) -> FnOptionArg {
    let l_option_arg = format!("--strategy-option={}", option_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_option_arg.as_str());
    })
}

/// Verify that the tip commit of the side branch being merged is signed with a valid key, i.e. a key that has a valid uid: in the default trust model, this means the signing key has been signed by a trusted key.
/// --verify-signatures
pub fn verify_signatures() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verify-signatures");
    })
}

/// Verify that the tip commit of the side branch being merged is signed with a valid key, i.e. a key that has a valid uid: in the default trust model, this means the signing key has been signed by a trusted key.
/// --no-verify-signatures
pub fn no_verify_signatures() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-verify-signatures");
    })
}

/// By default, git merge command refuses to merge histories that do not share a common ancestor.
/// This option can be used to override this safety when merging histories of two projects that started their lives independently.
/// As that is a very rare occasion, no configuration variable to enable this by default exists and will not be added.
/// --allow-unrelated-histories
pub fn allow_unrelated_histories() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--allow-unrelated-histories");
    })
}

/// When true, rebase the current branch on top of the upstream branch after fetching.
/// If there is a remote-tracking branch corresponding to the upstream branch and the upstream branch was rebased since last fetched, the rebase uses that information to avoid rebasing non-local changes.
/// -r, --rebase[=false|true|preserve|interactive]
pub fn rebase(value: &str) -> FnOptionArg {
    let l_value = if value.is_empty() {
        String::from("--rebase")
    } else {
        format!("--rebase={}", value)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}

/// Override earlier --rebase.
/// --no-rebase
pub fn no_rebase() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-rebase");
    })
}

/// Before starting rebase, stash local modifications away if needed, and apply the stash when done.
/// --autostash
pub fn autostash() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--autostash");
    })
}

/// --no-autostash is useful to override the rebase.autoStash configuration variable.
/// --no-autostash
pub fn no_autostash() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-autostash");
    })
}

/// Fetch all remotes.
/// --all
pub fn all() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--all");
    })
}

/// Append ref names and object names of fetched refs to the existing contents of .git/FETCH_HEAD.
/// Without this option old data in .git/FETCH_HEAD will be overwritten.
/// -a, --append
pub fn append() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--append");
    })
}

/// Limit fetching to the specified number of commits from the tip of each remote branch history.
/// If fetching to a shallow repository created by git clone with --depth=<depth> option (see git-clone(1)), deepen or shorten the history to the specified number of commits.
/// Tags for the deepened commits are not fetched.
/// --depth=<depth>
pub fn depth(depth_arg: &str) -> FnOptionArg {
    let l_depth_arg = format!("--depth={}", depth_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_depth_arg.as_str());
    })
}

/// Similar to --depth, except it specifies the number of commits from the current shallow boundary instead of from the tip of each remote branch history.
/// --deepen=<depth>
pub fn deepen(depth_arg: &str) -> FnOptionArg {
    let l_depth_arg = format!("--deepen={}", depth_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_depth_arg.as_str());
    })
}

/// Deepen or shorten the history of a shallow repository to include all reachable commits after <date>.
/// --shallow-since=<date>
pub fn shallow_since(date_arg: &str) -> FnOptionArg {
    let l_date_arg = format!("--shallow-since={}", date_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_date_arg.as_str());
    })
}

/// Deepen or shorten the history of a shallow repository to exclude commits reachable from a specified remote branch or tag.
/// This option can be specified multiple times.
/// --shallow-exclude=<revision>
pub fn shallow_exclude(revision_arg: &str) -> FnOptionArg {
    let l_revision_arg = format!("--shallow-exclude={}", revision_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_revision_arg.as_str());
    })
}

/// If the source repository is complete, convert a shallow repository to a complete one, removing all the limitations imposed by shallow repositories.
/// If the source repository is shallow, fetch as much as possible so that the current repository has the same history as the source repository.
/// --unshallow
pub fn unshallow() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--unshallow");
    })
}

/// By default when fetching from a shallow repository, git fetch refuses refs that require updating .git/shallow.
/// This option updates .git/shallow and accept such refs.
/// --update-shallow
pub fn update_shallow() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--update-shallow");
    })
}

/// When git fetch is used with <rbranch>:<lbranch> refspec, it refuses to update the local branch <lbranch> unless the remote branch <rbranch> it fetches is a descendant of <lbranch>.
/// This option overrides that check.
/// -f, --force
pub fn force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--force");
    })
}

/// Keep downloaded pack.
/// -k, --keep
pub fn keep() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--keep");
    })
}

/// By default, tags that point at objects that are downloaded from the remote repository are fetched and stored locally.
/// This option disables this automatic tag following.
/// The default behavior for a remote may be specified with the remote.<name>.tagOpt setting.
/// --no-tags
pub fn no_tags() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-tags");
    })
}

/// By default git fetch refuses to update the head which corresponds to the current branch.
/// This flag disables the check.
/// This is purely for the internal use for git pull to communicate with git fetch, and unless you are implementing your own Porcelain you are not supposed to use it.
/// -u, --update-head-ok
pub fn update_head_ok() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--update-head-ok");
    })
}

/// When given, and the repository to fetch from is handled by git fetch-pack, --exec=<upload-pack> is passed to the command to specify non-default path for the command run on the other end.
/// --upload-pack <upload-pack>
pub fn upload_pack(upload_pack_arg: &str) -> FnOptionArg {
    let l_upload_pack_arg = String::from(upload_pack_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--upload-pack");
        cmd.arg(l_upload_pack_arg.as_str());
    })
}

/// Progress status is reported on the standard error stream by default when it is attached to a terminal, unless -q is specified.
/// This flag forces progress status even if the standard error stream is not directed to a terminal.
/// --progress
pub fn progress() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--progress");
    })
}

/// Use IPv4 addresses only, ignoring IPv6 addresses.
/// -4, --ipv4
pub fn ipv4() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ipv4");
    })
}

/// Use IPv6 addresses only, ignoring IPv4 addresses.
/// -6, --ipv6
pub fn ipv6() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ipv6");
    })
}