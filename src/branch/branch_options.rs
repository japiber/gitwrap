use crate::command_executor::{CommandExecutor, CommandOption};

/// use <n> digits to display SHA-1s
/// --abbrev[=<n>]
pub fn abbrev_option(n_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--abbrev={}", n_arg)))
}

/// All list both remote-tracking and local branches
/// -a, --all
pub fn all_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--all"))
}

/// specify a valid branch name
/// <branch_name>
pub fn branch_name_option(value :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
         g.add_option(value);
    })
}


/// use colored output
/// --color[=<when>]
pub fn color_option(when_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--color={}", when_arg)))
}

/// list branches in columns
/// --column[=<style>]
pub fn column_option(style_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--column={}", style_arg)))
}

/// print only branches that contain the commit
/// --contains <commit>
pub fn contains_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--contains");
        g.add_option(commit_arg);
    })
}


/// print only branches that don't contain the commit
/// --no-contains <commit>
pub fn no_contains_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--no-contains");
        g.add_option(commit_arg);
    })
}


/// create the branch's reflog
/// -l, --create-reflog
pub fn create_reflog_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--create-reflog"))
}

/// delete fully merged branch
/// -d, --delete
pub fn delete_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--delete"))
}

/// delete branch (even if not merged)
/// -D
pub fn d_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("-D"))
}

/// edit the description for the branch
/// --edit-description
pub fn edit_description_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--edit-description"))
}

/// force creation, move/rename, deletion
/// -f, --force
pub fn force_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--force"))
}

/// format to use for the output
/// --format <format>
pub fn format_option(format_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--format");
        g.add_option(format_arg);
    })
}


/// sorting and filtering are case insensitive
/// -i, --ignore-case
pub fn ignore_case_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ignore-case"))
}

/// list branch names
/// --list
pub fn list_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--list"))
}

/// print only branches of the object
/// --points-at <object>
pub fn points_at_option(object_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--points-at");
        g.add_option(object_arg);
    })
}


/// print only branches that are merged
/// --merged <commit>
pub fn merged_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--merged");
        g.add_option(commit_arg);
    })
}


/// print only branches that are not merged
/// --no-merged <commit>
pub fn no_merged_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--no-merged");
        g.add_option(commit_arg);
    })
}


/// move/rename a branch and its reflog
/// -m, --move
pub fn move_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--move"))
}

/// move/rename a branch, even if target exists
/// -M
pub fn m_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("-M"))
}

/// suppress informational messages
/// -q, --quiet
pub fn quiet_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--quiet"))
}

/// act on remote-tracking branches
/// -r, --remotes
pub fn remotes_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--remotes"))
}

/// print the name of the current branch. In detached HEAD state, nothing is printed
/// --show-current
pub fn show_current_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--show-current"))
}

/// SetUpstream change upstream info
/// --unset-upstream


/// change the upstream info to upstream
/// -u, --set-upstream-to <upstream>
pub fn set_upstream_to_option(upstream_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--set-upstream-to");
        g.add_option(upstream_arg);
    })
}


/// field name to sort on
/// --sort <key>
pub fn sort_option(key_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--sort");
        g.add_option(key_arg);
    })
}


/// set up tracking mode (see git-pull(1)
/// -t, --track
pub fn track_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--track"))
}

/// show hash and subject, give twice for upstream branch
/// -v, --verbose
pub fn verbose_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--verbose"))
}

/// 