// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// Show cached files in the output (default)
/// -c, --cached
pub fn cached() -> FnOptionArg {
    optionarg::simple("--cached")
}

/// Show deleted files in the output
/// -d, --deleted
pub fn deleted() -> FnOptionArg {
    optionarg::simple("--deleted")
}

/// Show modified files in the output
/// -m, --modified
pub fn modified() -> FnOptionArg {
    optionarg::simple("--modified")
}

/// Show other (i.e. untracked) files in the output
/// -o, --others
pub fn others() -> FnOptionArg {
    optionarg::simple("--others")
}

/// Show only ignored files in the output. When showing files in the index, print only those matched by an exclude pattern. When showing "other" files, show only those matched by an exclude pattern. Standard ignore rules are not automatically activated, therefore at least one of the --exclude* options is required.
/// -i, --ignored
pub fn ignored() -> FnOptionArg {
    optionarg::simple("--ignored")
}

/// Show staged contents' mode bits, object name and stage number in the output.
/// -s, --stage
pub fn stage() -> FnOptionArg {
    optionarg::simple("--stage")
}

/// If a whole directory is classified as "other", show just its name (with a trailing slash) and not its whole contents.
/// --directory
pub fn directory() -> FnOptionArg {
    optionarg::simple("--directory")
}

/// Do not list empty directories. Has no effect without --directory.
/// --no-empty-directory
pub fn no_empty_directory() -> FnOptionArg {
    optionarg::simple("--no-empty-directory")
}

/// Show unmerged files in the output (forces --stage)
/// -u, --unmerged
pub fn unmerged() -> FnOptionArg {
    optionarg::simple("--unmerged")
}

/// Show files on the filesystem that need to be removed due to file/directory conflicts for checkout-index to succeed.
/// -k, --killed
pub fn killed() -> FnOptionArg {
    optionarg::simple("--killed")
}

/// \0 line termination on output and do not quote filenames. See OUTPUT below for more information.
/// -z
pub fn z() -> FnOptionArg {
    optionarg::simple("-z")
}

/// When only filenames are shown, suppress duplicates that may come from having multiple stages during a merge, or giving --deleted and --modified option at the same time. When any of the -t, --unmerged, or --stage option is in use, this option has no effect.
/// --deduplicate
pub fn deduplicate() -> FnOptionArg {
    optionarg::simple("--deduplicate")
}

/// Skip untracked files matching pattern. Note that pattern is a shell wildcard pattern. See EXCLUDE PATTERNS below for more information.
/// -x <pattern>, --exclude=<pattern>
pub fn exclude(pattern_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--exclude", pattern_arg)
}

/// Read exclude patterns from <file>; 1 per line.
/// -X <file>, --exclude-from=<file>
pub fn exclude_from(file_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--exclude-from", file_arg)
}

/// Read additional exclude patterns that apply only to the directory and its subdirectories in <file>.
/// --exclude-per-directory=<file>
pub fn exclude_per_directory(file_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--exclude-per-directory", file_arg)
}

/// Add the standard Git exclusions: .git/info/exclude, .gitignore in each directory, and the user’s global exclusion file.
/// --exclude-standard
pub fn exclude_standard() -> FnOptionArg {
    optionarg::simple("--exclude-standard")
}

/// If any <file> does not appear in the index, treat this as an error (return 1).
/// --error-unmatch
pub fn error_unmatch() -> FnOptionArg {
    optionarg::simple("--error-unmatch")
}

/// When using --error-unmatch to expand the user supplied <file> (i.e. path pattern) arguments to paths, pretend that paths which were removed in the index since the named <tree-ish> are still present. Using this option with -s or -u options does not make any sense.
/// --with-tree=<tree-ish>
pub fn with_tree(tree_ish_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--with-tree", tree_ish_arg)
}

/// This feature is semi-deprecated. For scripting purpose, git-status(1) --porcelain and git-diff-files(1) --name-status are almost always superior alternatives, and users should look at git-status(1) --short or git-diff(1) --name-status for more user-friendly alternatives.
/// -t
pub fn t() -> FnOptionArg {
    optionarg::simple("-t")
}

/// Similar to -t, but use lowercase letters for files that are marked as assume unchanged (see git-update-index(1)).
/// -v
pub fn v() -> FnOptionArg {
    optionarg::simple("-v")
}

/// Similar to -t, but use lowercase letters for files that are marked as fsmonitor valid (see git-update-index(1)).
/// -f
pub fn f() -> FnOptionArg {
    optionarg::simple("-f")
}

/// When run from a subdirectory, the command usually outputs paths relative to the current directory. This option forces paths to be output relative to the project top directory.
/// --full-name
pub fn full_name() -> FnOptionArg {
    optionarg::simple("--full-name")
}

/// Recursively calls ls-files on each active submodule in the repository. Currently there is only support for the --cached and --stage modes.
/// --recurse-submodules
pub fn recurse_submodules() -> FnOptionArg {
    optionarg::simple("--recurse-submodules")
}

/// Instead of showing the full 40-byte hexadecimal object lines, show the shortest prefix that is at least <n> hexdigits long that uniquely refers the object. Non default number of digits can be specified with --abbrev=<n>.
/// --abbrev[=<n>]
pub fn abbrev(n_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--abbrev", n_arg)
}

/// After each line that describes a file, add more data about its cache entry. This is intended to show as much information as possible for manual inspection; the exact format may change at any time.
/// --debug
pub fn debug() -> FnOptionArg {
    optionarg::simple("--debug")
}

/// Show <eolinfo> and <eolattr> of files. <eolinfo> is the file content identification used by Git when the "text" attribute is "auto" (or not set and core.autocrlf is not false). <eolinfo> is either "-text", "none", "lf", "crlf", "mixed" or "".\n"" means the file is not a regular file, it is not in the index or not accessible in the working tree.
/// <eolattr> is the attribute that is used when checking out or committing, it is either "", "-text", "text", "text=auto", "text eol=lf", "text eol=crlf". Since Git 2.10 "text=auto eol=lf" and "text=auto eol=crlf" are supported.
/// Both the <eolinfo> in the index ("i/<eolinfo>") and in the working tree ("w/<eolinfo>") are shown for regular files, followed by the ("attr/<eolattr>").
/// --eol
pub fn eol() -> FnOptionArg {
    optionarg::simple("--eol")
}

/// If the index is sparse, show the sparse directories without expanding to the contained files. Sparse directories will be shown with a trailing slash, such as "x/" for a sparse directory •"x•".
/// --sparse
pub fn sparse() -> FnOptionArg {
    optionarg::simple("--sparse")
}

/// A string that interpolates %(fieldname) from the result being shown. It also interpolates %% to %, and %xx where xx are hex digits interpolates to character with hex code xx; for example %00 interpolates to \0 (NUL), %09 to \t (TAB) and %0a to \n (LF). --format cannot be combined with -s, -o, -k, -t, --resolve-undo and --eol.
/// --format=<format>
pub fn format(format_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional("--format", format_arg)
}

/// Do not interpret any more arguments as options.
/// --
pub fn hyphen_hyphen() -> FnOptionArg {
    optionarg::simple("--")
}

/// Files to show. If no files are given all files which match the other specified criteria are shown.
/// <file>
pub fn file(file: &str) -> FnOptionArg {
    optionarg::value_parameter(file)
}
