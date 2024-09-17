use crate::command_executor::{CommandExecutor, CommandOption};

/// Make an unsigned, annotated tag object.
/// -a, --annotate
pub fn annotate_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--annotate"))
}

/// Make a GPG-signed tag, using the default e-mail address’s key.
/// -s, --sign
pub fn sign_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--sign"))
}

/// Make a GPG-signed tag, using the given key.
/// -u <keyid>, --local-user=<keyid>
pub fn local_user_option(keyid_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--local-user={}", keyid_arg)))
}

/// Replace an existing tag with the given name (instead of failing).
/// -f, --force
pub fn force_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--force"))
}

/// Verify the GPG signature of the given tag names.
/// -v, --verify
pub fn verify_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--verify"))
}

/// <num> specifies how many lines from the annotation, if any, are printed when using -l. Implies --list.
/// 
/// The default is not to print any annotation lines. If no number is given to -n, only the first line is printed. If the tag is not annotated, the commit message is displayed instead.
/// -n<num>
pub fn n_option(num_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("-n");
        g.add_option_string(format!("{}", num_arg ));
    })
}


/// List tags. With optional <pattern>..., e.g.  git tag --list 'v-*', list only the tags that match the pattern(s).
/// 
/// Running "git tag" without arguments also lists all tags. The pattern is a shell wildcard (i.e., matched using fnmatch(3)). Multiple patterns may be given; if any of them matches, the tag is shown.
/// 
/// This option is implicitly supplied if any other list-like option such as --contains is provided. See the documentation for each of those options for details.
/// -l, --list
pub fn list_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--list"))
}

/// Sort based on the key given. Prefix - to sort in descending order of the value. You may use the --sort=<key> option multiple times, in which case the last key becomes the primary key. Also
/// supports "version:refname" or "v:refname" (tag names are treated as versions). The "version:refname" sort order can also be affected by the "versionsort.suffix" configuration variable. The keys 
/// supported are the same as those in git for-each-ref. Sort order defaults to the value configured for the tag.sort variable if it exists, or lexicographic order otherwise. See git-config(1).
/// --sort=<key>
pub fn sort_option(key_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--sort={}", key_arg)))
}

/// Respect any colors specified in the --format option. The <when> field must be one of always, never, or auto (if <when> is absent, behave as if always was given).
/// --color[=<when>]
pub fn color_option(when_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--color={}", when_arg)))
}

/// Sorting and filtering tags are case insensitive.
/// -i, --ignore-case
pub fn ignore_case_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--ignore-case"))
}

/// Display tag listing in columns. See configuration variable column.tag for option syntax.--column and --no-column without options are equivalent to always and never respectively.
/// 
///  This option is only applicable when listing tags without annotation lines.
/// --column[=<options>]
pub fn column_option(options_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--column={}", options_arg)))
}

/// Display tag listing in columns. See configuration variable column.tag for option syntax.--column and --no-column without options are equivalent to always and never respectively.
/// 
///  This option is only applicable when listing tags without annotation lines.
/// --no-column
pub fn no_column_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--no-column"))
}

/// Only list tags which contain the specified commit (HEAD if not specified). Implies --list.
/// --contains [<commit>]
pub fn contains_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--contains={}", commit_arg)))
}

/// Only list tags which don’t contain the specified commit (HEAD if not specified). Implies --list.
/// --no-contains [<commit>]
pub fn no_contains_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--no-contains={}", commit_arg)))
}

/// Only list tags whose commits are reachable from the specified commit (HEAD if not specified), incompatible with --no-merged.
/// --merged [<commit>]
pub fn merged_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--merged={}", commit_arg)))
}

/// Only list tags whose commits are not reachable from the specified commit (HEAD if not specified), incompatible with --merged.
/// --no-merged [<commit>]
pub fn no_merged_option(commit_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--no-merged={}", commit_arg)))
}

/// Only list tags of the given object (HEAD if not specified). Implies --list.
/// --points-at <object>
pub fn points_at_option(object_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--points-at");
        g.add_option_string(format!("{}", object_arg ));
    })
}


/// Use the given tag message (instead of prompting).
/// If multiple -m options are given, their values are concatenated as separate paragraphs. Implies -a if none of -a, -s, or -u <keyid> is given.
/// -m <msg>, --message=<msg>
pub fn message_option(msg_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--message={}", msg_arg)))
}

/// Take the tag message from the given file. Use - to read the message from the standard input. Implies -a if none of -a, -s, or -u <keyid> is given.
/// -F <file>, --file=<file>
pub fn file_option(file_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--file={}", file_arg)))
}

/// The message taken from file with -F and command line with -m are usually used as the tag message unmodified.
/// This option lets you further edit the message taken from these sources.
/// -e, --edit
pub fn edit_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--edit"))
}

/// This option sets how the tag message is cleaned up. The <mode> can be one of verbatim, whitespace and strip. The strip mode is default. The verbatim mode does not change message at all, whitespace
///  removes just leading/trailing whitespace lines and strip removes both whitespace and commentary.
/// --cleanup=<mode>
pub fn cleanup_option(mode_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option_string(format!("--cleanup={}", mode_arg)))
}

/// Create a reflog for the tag. To globally enable reflogs for tags, see core.logAllRefUpdates in git-config(1). The negated form --no-create-reflog only overrides an earlier --create-reflog, but
///  currently does not negate the setting of core.logAllRefUpdates.
/// --create-reflog
pub fn create_reflog_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--create-reflog"))
}