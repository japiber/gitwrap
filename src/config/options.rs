// Warning!! Code generated automatically: this file must not be edited by hand
use crate::option_arg;
use crate::wrap_command::FnOptionArg;

pub const GLOBAL: &str = "--global";
pub const SYSTEM: &str = "--system";
pub const LOCAL: &str = "--local";
pub const FILE: &str = "--file";
pub const BLOB: &str = "--blob";
pub const LIST: &str = "--list";
pub const BOOL: &str = "--bool";
pub const INT: &str = "--int";
pub const BOOL_OR_INT: &str = "--bool-or-int";
pub const PATH: &str = "--path";
pub const NULL: &str = "--null";
pub const NAME_ONLY: &str = "--name-only";
pub const SHOW_ORIGIN: &str = "--show-origin";
pub const EDIT: &str = "--edit";

/// For writing options: write to global ~/.gitconfig file rather than the repository .git/config, write to $XDG_CONFIG_HOME/git/config file if this file exists and the ~/.gitconfig file doesn’t.
/// --global
pub fn global() -> FnOptionArg {
    option_arg::simple(GLOBAL)
}

/// For writing options: write to system-wide $(prefix)/etc/gitconfig rather than the repository .git/config.
/// --system
pub fn system() -> FnOptionArg {
    option_arg::simple(SYSTEM)
}

/// For writing options: write to the repository .git/config file.
/// This is the default behavior.
/// --local
pub fn local() -> FnOptionArg {
    option_arg::simple(LOCAL)
}

/// Use the given config file instead of the one specified by GIT_CONFIG.
/// -f <config-file>, --file <config-file>
pub fn file(config_file_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(FILE, config_file_arg)
}

/// Similar to --file but use the given blob instead of a file.
/// E.g.
/// you can use master:.gitmodules to read values from the file .gitmodules in the master branch.
/// --blob <blob>
pub fn blob(blob_arg: &str) -> FnOptionArg {
    option_arg::with_parameter(BLOB, blob_arg)
}

/// List all variables set in config file, along with their values.
/// -l, --list
pub fn list() -> FnOptionArg {
    option_arg::simple(LIST)
}

/// git config will ensure that the output is 'true' or 'false'
/// --bool
pub fn bool() -> FnOptionArg {
    option_arg::simple(BOOL)
}

/// git config will ensure that the output is a simple decimal number.
/// An optional value suffix of k, m, or g in the config file will cause the value to be multiplied by 1024, 1048576, or 1073741824 prior to output.
/// --int
pub fn int() -> FnOptionArg {
    option_arg::simple(INT)
}

/// git config will ensure that the output matches the format of either --bool or --int, as described above.
/// --bool-or-int
pub fn bool_or_int() -> FnOptionArg {
    option_arg::simple(BOOL_OR_INT)
}

/// git-config will expand leading ~ to the value of $HOME, and ~user to the home directory for the specified user.
/// This option has no effect when setting the value (but you can use git config bla ~/ from the command line to let your shell do the expansion).
/// --path
pub fn path() -> FnOptionArg {
    option_arg::simple(PATH)
}

/// For all options that output values and/or keys, always end values with the null character (instead of a newline).
/// Use newline instead as a delimiter between key and value.
/// This allows for secure parsing of the output without getting confused e.g.
/// by values that contain line breaks.
/// -z, --null
pub fn null() -> FnOptionArg {
    option_arg::simple(NULL)
}

/// Output only the names of config variables for --list or --get-regexp.
/// --name-only
pub fn name_only() -> FnOptionArg {
    option_arg::simple(NAME_ONLY)
}

/// Augment the output of all queried config options with the origin type (file, standard input, blob, command line)
/// and the actual origin (config file path, ref, or blob id if applicable).
/// --show-origin
pub fn show_origin() -> FnOptionArg {
    option_arg::simple(SHOW_ORIGIN)
}

/// Opens an editor to modify the specified config file; either --system, --global, or repository (default).
/// -e, --edit
pub fn edit() -> FnOptionArg {
    option_arg::simple(EDIT)
}
