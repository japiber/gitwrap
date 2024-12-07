// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// use <n> digits to display SHA-1s
/// --abbrev[=<n>]
pub fn abbrev(n_arg: &str) -> FnOptionArg {
    let l_n_arg = format!("--abbrev={}", n_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_n_arg.as_str());
    })
}

/// All list both remote-tracking and local branches
/// -a, --all
pub fn all() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--all");
    })
}

/// specify a valid branch name
/// <branch_name>
pub fn branch_name(value: &str) -> FnOptionArg {
    let l_value = String::from(value);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_value.as_str());
    })
}

/// use colored output
/// --color[=<when>]
pub fn color(when_arg: &str) -> FnOptionArg {
    let l_when_arg = format!("--color={}", when_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_when_arg.as_str());
    })
}

/// list branches in columns
/// --column[=<style>]
pub fn column(style_arg: &str) -> FnOptionArg {
    let l_style_arg = format!("--column={}", style_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_style_arg.as_str());
    })
}

/// print only branches that contain the commit
/// --contains <commit>
pub fn contains(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = String::from(commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--contains");
        cmd.arg(l_commit_arg.as_str());
    })
}

/// print only branches that don't contain the commit
/// --no-contains <commit>
pub fn no_contains(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = String::from(commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-contains");
        cmd.arg(l_commit_arg.as_str());
    })
}

/// create the branch's reflog
/// -l, --create-reflog
pub fn create_reflog() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--create-reflog");
    })
}

/// delete fully merged branch
/// -d, --delete
pub fn delete() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--delete");
    })
}

/// delete branch (even if not merged)
/// -D
pub fn delete_force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-D");
    })
}

/// edit the description for the branch
/// --edit-description
pub fn edit_description() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--edit-description");
    })
}

/// force creation, move/rename, deletion
/// -f, --force
pub fn force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--force");
    })
}

/// format to use for the output
/// --format <format>
pub fn format(format_arg: &str) -> FnOptionArg {
    let l_format_arg = String::from(format_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--format");
        cmd.arg(l_format_arg.as_str());
    })
}

/// sorting and filtering are case insensitive
/// -i, --ignore-case
pub fn ignore_case() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ignore-case");
    })
}

/// list branch names
/// --list
pub fn list() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--list");
    })
}

/// print only branches of the object
/// --points-at <object>
pub fn points_at(object_arg: &str) -> FnOptionArg {
    let l_object_arg = String::from(object_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--points-at");
        cmd.arg(l_object_arg.as_str());
    })
}

/// print only branches that are merged
/// --merged <commit>
pub fn merged(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = String::from(commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--merged");
        cmd.arg(l_commit_arg.as_str());
    })
}

/// print only branches that are not merged
/// --no-merged <commit>
pub fn no_merged(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = String::from(commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-merged");
        cmd.arg(l_commit_arg.as_str());
    })
}

/// move/rename a branch and its reflog
/// -m, --move
pub fn move_branch() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--move");
    })
}

/// move/rename a branch, even if target exists
/// -M
pub fn move_force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-M");
    })
}

/// suppress informational messages
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--quiet");
    })
}

/// act on remote-tracking branches
/// -r, --remotes
pub fn remotes() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--remotes");
    })
}

/// print the name of the current branch. In detached HEAD state, nothing is printed
/// --show-current
pub fn show_current() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--show-current");
    })
}

/// SetUpstream change upstream info
/// --unset-upstream
pub fn unset_upstream(branchname_arg: &str) -> FnOptionArg {
    let l_branchname_arg = if branchname_arg.is_empty() {
        String::from("--unset-upstream")
    } else {
        format!("--unset-upstream {}", branchname_arg)
    };
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_branchname_arg.as_str());
    })
}

/// change the upstream info to upstream
/// -u, --set-upstream-to <upstream>
pub fn set_upstream_to(upstream_arg: &str) -> FnOptionArg {
    let l_upstream_arg = String::from(upstream_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--set-upstream-to");
        cmd.arg(l_upstream_arg.as_str());
    })
}

/// field name to sort on
/// --sort <key>
pub fn sort(key_arg: &str) -> FnOptionArg {
    let l_key_arg = String::from(key_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--sort");
        cmd.arg(l_key_arg.as_str());
    })
}

/// set up tracking mode (see git-pull(1)
/// -t, --track
pub fn track() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--track");
    })
}

/// show hash and subject, give twice for upstream branch
/// -v, --verbose
pub fn verbose() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verbose");
    })
}