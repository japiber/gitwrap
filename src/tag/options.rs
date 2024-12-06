// Warning!! Code generated automatically: this file must not be edited by hand

use std::process::Command;

use crate::wrap_command::FnOptionArg;

/// Make an unsigned, annotated tag object.
/// -a, --annotate
pub fn annotate() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--annotate");
    })
}

/// Make a GPG-signed tag, using the default e-mail address’s key.
/// -s, --sign
pub fn sign() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--sign");
    })
}

/// Make a GPG-signed tag, using the given key.
/// -u <keyid>, --local-user=<keyid>
pub fn local_user(keyid_arg: &str) -> FnOptionArg {
    let l_keyid_arg = format!("--local-user={}", keyid_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_keyid_arg.as_str());
    })
}

/// Replace an existing tag with the given name (instead of failing).
/// -f, --force
pub fn force() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--force");
    })
}

/// Verify the GPG signature of the given tag names.
/// -v, --verify
pub fn verify() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--verify");
    })
}

/// <num> specifies how many lines from the annotation, if any, are printed when using -l. Implies --list.
/// 
/// The default is not to print any annotation lines. If no number is given to -n, only the first line is printed. If the tag is not annotated, the commit message is displayed instead.
/// -n<num>
pub fn n(num_arg: &str) -> FnOptionArg {
    let l_num_arg = String::from(num_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("-n");
        cmd.arg(l_num_arg.as_str());
    })
}

/// List tags. With optional <pattern>..., e.g.  git tag --list 'v-*', list only the tags that match the pattern(s).
/// 
/// Running "git tag" without arguments also lists all tags. The pattern is a shell wildcard (i.e., matched using fnmatch(3)). Multiple patterns may be given; if any of them matches, the tag is shown.
/// 
/// This option is implicitly supplied if any other list-like option such as --contains is provided. See the documentation for each of those options for details.
/// -l, --list
pub fn list() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--list");
    })
}

/// Sort based on the key given. Prefix - to sort in descending order of the value. You may use the --sort=<key> option multiple times, in which case the last key becomes the primary key. Also
/// supports "version:refname" or "v:refname" (tag names are treated as versions). The "version:refname" sort order can also be affected by the "versionsort.suffix" configuration variable. The keys 
/// supported are the same as those in git for-each-ref. Sort order defaults to the value configured for the tag.sort variable if it exists, or lexicographic order otherwise. See git-config(1).
/// --sort=<key>
pub fn sort(key_arg: &str) -> FnOptionArg {
    let l_key_arg = format!("--sort={}", key_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_key_arg.as_str());
    })
}

/// Respect any colors specified in the --format option. The <when> field must be one of always, never, or auto (if <when> is absent, behave as if always was given).
/// --color[=<when>]
pub fn color(when_arg: &str) -> FnOptionArg {
    let l_when_arg = format!("--color={}", when_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_when_arg.as_str());
    })
}

/// Sorting and filtering tags are case insensitive.
/// -i, --ignore-case
pub fn ignore_case() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--ignore-case");
    })
}

/// Display tag listing in columns. See configuration variable column.tag for option syntax.--column and --no-column without options are equivalent to always and never respectively.
/// 
///  This option is only applicable when listing tags without annotation lines.
/// --column[=<options>]
pub fn column(options_arg: &str) -> FnOptionArg {
    let l_options_arg = format!("--column={}", options_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_options_arg.as_str());
    })
}

/// Display tag listing in columns. See configuration variable column.tag for option syntax.--column and --no-column without options are equivalent to always and never respectively.
/// 
///  This option is only applicable when listing tags without annotation lines.
/// --no-column
pub fn no_column() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--no-column");
    })
}

/// Only list tags which contain the specified commit (HEAD if not specified). Implies --list.
/// --contains [<commit>]
pub fn contains(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = format!("--contains={}", commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_commit_arg.as_str());
    })
}

/// Only list tags which don’t contain the specified commit (HEAD if not specified). Implies --list.
/// --no-contains [<commit>]
pub fn no_contains(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = format!("--no-contains={}", commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_commit_arg.as_str());
    })
}

/// Only list tags whose commits are reachable from the specified commit (HEAD if not specified), incompatible with --no-merged.
/// --merged [<commit>]
pub fn merged(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = format!("--merged={}", commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_commit_arg.as_str());
    })
}

/// Only list tags whose commits are not reachable from the specified commit (HEAD if not specified), incompatible with --merged.
/// --no-merged [<commit>]
pub fn no_merged(commit_arg: &str) -> FnOptionArg {
    let l_commit_arg = format!("--no-merged={}", commit_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_commit_arg.as_str());
    })
}

/// Only list tags of the given object (HEAD if not specified). Implies --list.
/// --points-at <object>
pub fn points_at(object_arg: &str) -> FnOptionArg {
    let l_object_arg = String::from(object_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--points-at");
        cmd.arg(l_object_arg.as_str());
    })
}

/// Use the given tag message (instead of prompting).
/// If multiple -m options are given, their values are concatenated as separate paragraphs. Implies -a if none of -a, -s, or -u <keyid> is given.
/// -m <msg>, --message=<msg>
pub fn message(msg_arg: &str) -> FnOptionArg {
    let l_msg_arg = format!("--message={}", msg_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_msg_arg.as_str());
    })
}

/// Take the tag message from the given file. Use - to read the message from the standard input. Implies -a if none of -a, -s, or -u <keyid> is given.
/// -F <file>, --file=<file>
pub fn file(file_arg: &str) -> FnOptionArg {
    let l_file_arg = format!("--file={}", file_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_file_arg.as_str());
    })
}

/// The message taken from file with -F and command line with -m are usually used as the tag message unmodified.
/// This option lets you further edit the message taken from these sources.
/// -e, --edit
pub fn edit() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--edit");
    })
}

/// This option sets how the tag message is cleaned up. The <mode> can be one of verbatim, whitespace and strip. The strip mode is default. The verbatim mode does not change message at all, whitespace
///  removes just leading/trailing whitespace lines and strip removes both whitespace and commentary.
/// --cleanup=<mode>
pub fn cleanup(mode_arg: &str) -> FnOptionArg {
    let l_mode_arg = format!("--cleanup={}", mode_arg);
    Box::new(move |cmd: &mut Command| {
        cmd.arg(l_mode_arg.as_str());
    })
}

/// Create a reflog for the tag. To globally enable reflogs for tags, see core.logAllRefUpdates in git-config(1). The negated form --no-create-reflog only overrides an earlier --create-reflog, but
///  currently does not negate the setting of core.logAllRefUpdates.
/// --create-reflog
pub fn create_reflog() -> FnOptionArg {
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--create-reflog");
    })
}