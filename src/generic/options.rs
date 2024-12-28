// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const VERSION: &str = "--version";
pub const HELP: &str = "--help";
pub const WORKING_PATH: &str = "-C";
pub const EXEC_PATH: &str = "--exec-path";
pub const HTML_PATH: &str = "--html-path";
pub const MAN_PATH: &str = "--man-path";
pub const INFO_PATH: &str = "--info-path";
pub const PAGINATE: &str = "--paginate";
pub const NO_PAGER: &str = "--no-pager";
pub const GIT_DIR: &str = "--git-dir";
pub const WORK_TREE: &str = "--work-tree";
pub const NAMESPACE: &str = "--namespace";
pub const BARE: &str = "--bare";
pub const NO_REPLACE_OBJECTS: &str = "--no-replace-objects";
pub const NO_LAZY_FETCH: &str = "--no-lazy-fetch";
pub const NO_OPTIONAL_LOCKS: &str = "--no-optional-locks";
pub const NO_ADVICE: &str = "--no-advice";
pub const GLOB_PATHSPECS: &str = "--glob-pathspecs";
pub const NOGLOB_PATHSPECS: &str = "--noglob-pathspecs";
pub const ICASE_PATHSPECS: &str = "--icase-pathspecs";
pub const ATTR_SOURCE: &str = "--attr-source";

/// Prints the Git suite version that the git program came from.
/// --version, -v
pub fn version() -> FnOptionArg {
    option_arg::simple(VERSION)
}

/// Prints the synopsis and a list of the most commonly used commands.
/// --help, -h
pub fn help() -> FnOptionArg {
    option_arg::simple(HELP)
}

/// Run as if git was started in <path> instead of the current working directory.
/// -C <path>
pub fn working_path(path_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(WORKING_PATH, path_arg)
}

/// Path to wherever your core Git programs are installed.
/// This can also be controlled by setting the GIT_EXEC_PATH environment variable.
/// If no path is given, git will print the current setting and then exit.
/// --exec-path[=<path>]
pub fn exec_path(path_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(EXEC_PATH, path_arg)
}

/// Print the path, without trailing slash, where Git’s HTML documentation is installed and exit
/// --html-path
pub fn html_path() -> FnOptionArg {
    option_arg::simple(HTML_PATH)
}

/// Print the manpath (see man(1)) for the man pages for this version of Git and exit.
/// --man-path
pub fn man_path() -> FnOptionArg {
    option_arg::simple(MAN_PATH)
}

/// Print the path where the Info files documenting this version of Git are installed and exit
/// --info-path
pub fn info_path() -> FnOptionArg {
    option_arg::simple(INFO_PATH)
}

/// Pipe all output into less (or if set, $PAGER) if standard output is a terminal. This overrides the pager.<cmd> configuration options
/// --paginate, -p
pub fn paginate() -> FnOptionArg {
    option_arg::simple(PAGINATE)
}

/// Do not pipe Git output into a pager.
/// --no-pager, -P
pub fn no_pager() -> FnOptionArg {
    option_arg::simple(NO_PAGER)
}

/// Set the path to the repository (".git" directory).
/// This can also be controlled by setting the GIT_DIR environment variable.
/// It can be an absolute path or relative path to current working directory.
/// --git-dir=<path>
pub fn git_dir(path_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(GIT_DIR, path_arg)
}

/// Set the path to the working tree.
/// It can be an absolute path or a path relative to the current working directory.
/// This can also be controlled by setting the GIT_WORK_TREE environment variable and the core.worktree configuration variable
/// --work-tree=<path>
pub fn work_tree(path_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(WORK_TREE, path_arg)
}

/// Set the Git namespace.
/// See gitnamespaces[7] for more details.
/// Equivalent to setting the GIT_NAMESPACE environment variable.
/// --namespace=<path>
pub fn namespace(path_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(NAMESPACE, path_arg)
}

/// Treat the repository as a bare repository.
/// If GIT_DIR environment is not set, it is set to the current working directory.
/// --bare
pub fn bare() -> FnOptionArg {
    option_arg::simple(BARE)
}

/// Do not use replacement refs to replace Git objects.
/// This is equivalent to exporting the GIT_NO_REPLACE_OBJECTS environment variable with any value.
/// --no-replace-objects
pub fn no_replace_objects() -> FnOptionArg {
    option_arg::simple(NO_REPLACE_OBJECTS)
}

/// Do not fetch missing objects from the promisor remote on demand.
/// Useful together with git cat-file -e <object> to see if the object is locally available.
/// This is equivalent to setting the GIT_NO_LAZY_FETCH environment variable to 1
/// --no-lazy-fetch
pub fn no_lazy_fetch() -> FnOptionArg {
    option_arg::simple(NO_LAZY_FETCH)
}

/// Do not perform optional operations that require locks.
/// This is equivalent to setting the GIT_OPTIONAL_LOCKS to 0.
/// --no-optional-locks
pub fn no_optional_locks() -> FnOptionArg {
    option_arg::simple(NO_OPTIONAL_LOCKS)
}

/// Disable all advice hints from being printed
/// --no-advice
pub fn no_advice() -> FnOptionArg {
    option_arg::simple(NO_ADVICE)
}

/// Add "glob" magic to all pathspec.
/// This is equivalent to setting the GIT_GLOB_PATHSPECS environment variable to 1.
/// --glob-pathspecs
pub fn glob_pathspecs() -> FnOptionArg {
    option_arg::simple(GLOB_PATHSPECS)
}

/// Add "literal" magic to all pathspec.
/// This is equivalent to setting the GIT_NOGLOB_PATHSPECS environment variable to 1.
/// --noglob-pathspecs
pub fn noglob_pathspecs() -> FnOptionArg {
    option_arg::simple(NOGLOB_PATHSPECS)
}

/// Add "icase" magic to all pathspec.
/// This is equivalent to setting the GIT_ICASE_PATHSPECS environment variable to 1.
/// --icase-pathspecs
pub fn icase_pathspecs() -> FnOptionArg {
    option_arg::simple(ICASE_PATHSPECS)
}

/// Read gitattributes from <tree-ish> instead of the worktree. See gitattributes[5]. This is equivalent to setting the GIT_ATTR_SOURCE environment variable.
/// --attr-source=<tree-ish>
pub fn attr_source(tree_ish_arg: &str) -> FnOptionArg {
    option_arg::equal_no_optional(ATTR_SOURCE, tree_ish_arg)
}
