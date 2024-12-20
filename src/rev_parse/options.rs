// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;

pub const PARSEOPT: &str = "--parseopt";
pub const SQ_QUOTE: &str = "--sq-quote";
pub const KEEP_DASHDASH: &str = "--keep-dashdash";
pub const STOP_AT_NON_OPTION: &str = "--stop-at-non-option";
pub const STUCK_LONG: &str = "--stuck-long";
pub const REVS_ONLY: &str = "--revs-only";
pub const NO_REVS: &str = "--no-revs";
pub const FLAGS: &str = "--flags";
pub const NO_FLAGS: &str = "--no-flags";
pub const DEFAULT: &str = "--default";
pub const PREFIX: &str = "--prefix";
pub const VERIFY: &str = "--verify";
pub const QUIET: &str = "--quiet";
pub const SQ: &str = "--sq";
pub const SHORT: &str = "--short";
pub const NOT: &str = "--not";
pub const ABBREV_REF: &str = "--abbrev-ref";
pub const SYMBOLIC: &str = "--symbolic";
pub const SYMBOLIC_FULL_NAME: &str = "--symbolic-full-name";
pub const ALL: &str = "--all";
pub const BRANCHES: &str = "--branches";
pub const TAGS: &str = "--tags";
pub const REMOTES: &str = "--remotes";
pub const GLOB: &str = "--glob";
pub const EXCLUDE: &str = "--exclude";
pub const DISAMBIGUATE: &str = "--disambiguate";
pub const LOCAL_ENV_VARS: &str = "--local-env-vars";
pub const GIT_DIR: &str = "--git-dir";
pub const ABSOLUTE_GIT_DIR: &str = "--absolute-git-dir";
pub const GIT_COMMON_DIR: &str = "--git-common-dir";
pub const IS_INSIDE_GIT_DIR: &str = "--is-inside-git-dir";
pub const IS_INSIDE_WORK_TREE: &str = "--is-inside-work-tree";
pub const IS_BARE_REPOSITORY: &str = "--is-bare-repository";
pub const RESOLVE_GIT_DIR: &str = "--resolve-git-dir";
pub const GIT_PATH: &str = "--git-path";
pub const SHOW_CDUP: &str = "--show-cdup";
pub const SHOW_PREFIX: &str = "--show-prefix";
pub const SHOW_TOPLEVEL: &str = "--show-toplevel";
pub const SHOW_SUPERPROJECT_WORKING_TREE: &str = "--show-superproject-working-tree";
pub const SHARED_INDEX_PATH: &str = "--shared-index-path";
pub const SINCE: &str = "--since";
pub const AFTER: &str = "-after";
pub const UNTIL: &str = "--until";
pub const BEFORE: &str = "--before";

/// Use git rev-parse in option parsing mode (see PARSEOPT section below).
/// --parseopt
pub fn parseopt() -> FnOptionArg {
    optionarg::simple(PARSEOPT)
}

/// Use git rev-parse in shell quoting mode (see SQ-QUOTE section below).
/// In contrast to the --sq option below, this mode does only quoting.
/// Nothing else is done to command input.
/// --sq-quote
pub fn sq_quote() -> FnOptionArg {
    optionarg::simple(SQ_QUOTE)
}

/// Only meaningful in --parseopt mode.
/// Tells the option parser to echo out the first -- met instead of skipping it.
/// --keep-dashdash
pub fn keep_dashdash() -> FnOptionArg {
    optionarg::simple(KEEP_DASHDASH)
}

/// Only meaningful in --parseopt mode.
/// Lets the option parser stop at the first non-option argument.
/// This can be used to parse sub-commands that take options themselves.
/// --stop-at-non-option
pub fn stop_at_non_option() -> FnOptionArg {
    optionarg::simple(STOP_AT_NON_OPTION)
}

/// Only meaningful in --parseopt mode.
/// Output the options in their long form if available, and with their arguments stuck.
/// --stuck-long
pub fn stuck_long() -> FnOptionArg {
    optionarg::simple(STUCK_LONG)
}

/// Do not output flags and parameters not meant for git rev-list command.
/// --revs-only
pub fn revs_only() -> FnOptionArg {
    optionarg::simple(REVS_ONLY)
}

/// Do not output flags and parameters meant for git rev-list command.
/// --no-revs
pub fn no_revs() -> FnOptionArg {
    optionarg::simple(NO_REVS)
}

/// Do not output non-flag parameters.
/// --flags
pub fn flags() -> FnOptionArg {
    optionarg::simple(FLAGS)
}

/// Do not output flag parameters.
/// --no-flags
pub fn no_flags() -> FnOptionArg {
    optionarg::simple(NO_FLAGS)
}

/// If there is no parameter given by the user, use <arg> instead.
/// --default <arg>
pub fn default(arg_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(DEFAULT, arg_arg)
}

/// Behave as if git rev-parse was invoked from the <arg> subdirectory of the working tree.
/// Any relative filenames are resolved as if they are prefixed by <arg> and will be printed in that form.
/// This can be used to convert arguments to a command run in a subdirectory so that they can still be used after moving to the top-level of the repository.
/// --prefix <arg>
pub fn prefix(arg_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(PREFIX, arg_arg)
}

/// Verify that exactly one parameter is provided, and that it can be turned into a raw 20-byte SHA-1 that can be used to access the object database.
/// If so, emit it to the standard output; otherwise, error out.
/// --verify
pub fn verify() -> FnOptionArg {
    optionarg::simple(VERIFY)
}

/// Only meaningful in --verify mode.
/// Do not output an error message if the first argument is not a valid object name; instead exit with non-zero status silently.
/// SHA-1s for valid object names are printed to stdout on success.
/// -q, --quiet
pub fn quiet() -> FnOptionArg {
    optionarg::simple(QUIET)
}

/// Usually the output is made one line per flag and parameter.
/// This option makes output a single line, properly quoted for consumption by shell.
/// Useful when you expect your parameter to contain whitespaces and newlines (e.g.
/// when using pickaxe -S with git diff-*).
/// In contrast to the --sq-quote option, the command input is still interpreted as usual.
/// --sq
pub fn sq() -> FnOptionArg {
    optionarg::simple(SQ)
}

/// Same as --verify but shortens the object name to a unique prefix with at least length characters.
/// The minimum length is 4, the default is the effective value of the core.abbrev configuration variable (see git-config(1)).
/// --short[=length]
pub fn short(value: &str) -> FnOptionArg {
    optionarg::equal_optional(SHORT, value)
}

/// When showing object names, prefix them with ^ and strip ^ prefix from the object names that already have one.
/// --not
pub fn not() -> FnOptionArg {
    optionarg::simple(NOT)
}

/// A non-ambiguous short name of the objects name.
/// The option core.warnAmbiguousRefs is used to select the strict abbreviation mode.
/// --abbrev-ref[=(strict|loose)]
pub fn abbrev_ref(value: &str) -> FnOptionArg {
    optionarg::equal_optional(ABBREV_REF, value)
}

/// Usually the object names are output in SHA-1 form (with possible ^ prefix); this option makes them output in a form as close to the original input as possible.
/// --symbolic
pub fn symbolic() -> FnOptionArg {
    optionarg::simple(SYMBOLIC)
}

/// This is similar to --symbolic, but it omits input that are not refs
/// (i.e.
/// branch or tag names; or more explicitly disambiguating 'heads/master' form, when you want to name the 'master' branch when there is an unfortunately named tag 'master'),
/// and show them as full refnames (e.g.
/// 'refs/heads/master').
/// --symbolic-full-name
pub fn symbolic_full_name() -> FnOptionArg {
    optionarg::simple(SYMBOLIC_FULL_NAME)
}

/// Show all refs found in refs/.
/// --all
pub fn all() -> FnOptionArg {
    optionarg::simple(ALL)
}

/// Show all branches, tags, or remote-tracking branches, respectively (i.e., refs found in refs/heads, refs/tags, or refs/remotes, respectively).
/// If a pattern is given, only refs matching the given shell glob are shown.
/// If the pattern does not contain a globbing character (?, *, or [), it is turned into a prefix match by appending /*.
/// --branches[=pattern], --tags[=pattern], --remotes[=pattern]
pub fn branches(value: &str) -> FnOptionArg {
    optionarg::equal_optional(BRANCHES, value)
}

/// Show all branches, tags, or remote-tracking branches, respectively (i.e., refs found in refs/heads, refs/tags, or refs/remotes, respectively).
/// If a pattern is given, only refs matching the given shell glob are shown.
/// If the pattern does not contain a globbing character (?, *, or [), it is turned into a prefix match by appending /*.
/// --branches[=pattern], --tags[=pattern], --remotes[=pattern]
pub fn tags(value: &str) -> FnOptionArg {
    optionarg::equal_optional(TAGS, value)
}

/// Show all branches, tags, or remote-tracking branches, respectively (i.e., refs found in refs/heads, refs/tags, or refs/remotes, respectively).
/// If a pattern is given, only refs matching the given shell glob are shown.
/// If the pattern does not contain a globbing character (?, *, or [), it is turned into a prefix match by appending /*.
/// --branches[=pattern], --tags[=pattern], --remotes[=pattern]
pub fn remotes(value: &str) -> FnOptionArg {
    optionarg::equal_optional(REMOTES, value)
}

/// Show all refs matching the shell glob pattern pattern.
/// If the pattern does not start with refs/, this is automatically prepended.
/// If the pattern does not contain a globbing character (?, *, or [), it is turned into a prefix match by appending /*.
/// --glob=pattern
pub fn glob(pattern_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(GLOB, pattern_arg)
}

/// Do not include refs matching <glob-pattern> that the next --all, --branches, --tags, --remotes, or --glob would otherwise consider.
/// Repetitions of this option accumulate exclusion patterns up to the next --all, --branches, --tags, --remotes, or --glob option (other options or arguments do not clear accumulated patterns).
/// The patterns given should not begin with refs/heads, refs/tags, or refs/remotes when applied to --branches, --tags, or --remotes, respectively, and they must begin with refs/ when applied to --glob or --all.
/// If a trailing /* is intended, it must be given explicitly.
/// --exclude=<glob-pattern>
pub fn exclude(glob_pattern_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(EXCLUDE, glob_pattern_arg)
}

/// Show every object whose name begins with the given prefix.
/// The <prefix> must be at least 4 hexadecimal digits long to avoid listing each and every object in the repository by mistake.
/// --disambiguate=<prefix>
pub fn disambiguate(prefix_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(DISAMBIGUATE, prefix_arg)
}

/// List the GIT_* environment variables that are local to the repository (e.g.
/// GIT_DIR or GIT_WORK_TREE, but not GIT_EDITOR).
/// Only the names of the variables are listed, not their value, even if they are set.
/// --local-env-vars
pub fn local_env_vars() -> FnOptionArg {
    optionarg::simple(LOCAL_ENV_VARS)
}

/// Show $GIT_DIR if defined.
/// Otherwise show the path to the .git directory.
/// The path shown, when relative, is relative to the current working directory.
/// If $GIT_DIR is not defined and the current directory is not detected to lie in a Git repository or work tree print a message to stderr and exit with nonzero status.
/// --git-dir
pub fn git_dir() -> FnOptionArg {
    optionarg::simple(GIT_DIR)
}

/// Like --git-dir, but its output is always the canonicalized absolute path.
/// --absolute-git-dir
pub fn absolute_git_dir() -> FnOptionArg {
    optionarg::simple(ABSOLUTE_GIT_DIR)
}

/// Show $GIT_COMMON_DIR if defined, else $GIT_DIR.
/// --git-common-dir
pub fn git_common_dir() -> FnOptionArg {
    optionarg::simple(GIT_COMMON_DIR)
}

/// When the current working directory is below the repository directory print 'true', otherwise 'false'.
/// --is-inside-git-dir
pub fn is_inside_git_dir() -> FnOptionArg {
    optionarg::simple(IS_INSIDE_GIT_DIR)
}

/// When the current working directory is inside the work tree of the repository print 'true', otherwise 'false'.
/// --is-inside-work-tree
pub fn is_inside_work_tree() -> FnOptionArg {
    optionarg::simple(IS_INSIDE_WORK_TREE)
}

/// When the repository is bare print 'true', otherwise 'false'.
/// --is-bare-repository
pub fn is_bare_repository() -> FnOptionArg {
    optionarg::simple(IS_BARE_REPOSITORY)
}

/// Check if <path> is a valid repository or a gitfile that points at a valid repository, and print the location of the repository.
/// If <path> is a gitfile then the resolved path to the real repository is printed.
/// --resolve-git-dir <path>
pub fn resolve_git_dir(path_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(RESOLVE_GIT_DIR, path_arg)
}

/// Resolve '$GIT_DIR/<path>' and takes other path relocation variables such as $GIT_OBJECT_DIRECTORY, $GIT_INDEX_FILE...
/// into account.
/// For example, if $GIT_OBJECT_DIRECTORY is set to /foo/bar then 'git rev-parse --git-path objects/abc' returns /foo/bar/abc.
/// --git-path <path>
pub fn git_path(path_arg: &str) -> FnOptionArg {
    optionarg::with_parameter(GIT_PATH, path_arg)
}

/// When the command is invoked from a subdirectory,
/// show the path of the top-level directory relative to the current directory (typically a sequence of '../', or an empty string).
/// --show-cdup
pub fn show_cdup() -> FnOptionArg {
    optionarg::simple(SHOW_CDUP)
}

/// When the command is invoked from a subdirectory,
/// show the path of the current directory relative to the top-level directory.
/// --show-prefix
pub fn show_prefix() -> FnOptionArg {
    optionarg::simple(SHOW_PREFIX)
}

/// Show the absolute path of the top-level directory.
/// --show-toplevel
pub fn show_toplevel() -> FnOptionArg {
    optionarg::simple(SHOW_TOPLEVEL)
}

/// Show the absolute path of the root of the superproject’s working tree (if exists) that uses the current repository as its submodule.
/// Outputs nothing if the current repository is not used as a submodule by any project.
/// --show-superproject-working-tree
pub fn show_superproject_working_tree() -> FnOptionArg {
    optionarg::simple(SHOW_SUPERPROJECT_WORKING_TREE)
}

/// Show the path to the shared index file in split index mode, or empty if not in split-index mode.
/// --shared-index-path
pub fn shared_index_path() -> FnOptionArg {
    optionarg::simple(SHARED_INDEX_PATH)
}

/// Parse the date string, and output the corresponding --max-age= parameter for git rev-list.
/// --since=datestring, --after=datestring
pub fn since(datestring_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(SINCE, datestring_arg)
}

/// Parse the date string, and output the corresponding --max-age= parameter for git rev-list.
/// --since=datestring, --after=datestring
pub fn after(datestring_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(AFTER, datestring_arg)
}

/// Parse the date string, and output the corresponding --min-age= parameter for git rev-list.
/// --until=datestring, --before=datestring
pub fn until(datestring_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(UNTIL, datestring_arg)
}

/// Parse the date string, and output the corresponding --min-age= parameter for git rev-list.
/// --until=datestring, --before=datestring
pub fn before(datestring_arg: &str) -> FnOptionArg {
    optionarg::equal_no_optional(BEFORE, datestring_arg)
}
