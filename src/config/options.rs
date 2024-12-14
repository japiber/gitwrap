// Warning!! Code generated automatically: this file must not be edited by hand
use crate::optionarg;
use crate::wrap_command::FnOptionArg;
/// For writing options: write to global ~/.gitconfig file rather than the repository .git/config, write to $XDG_CONFIG_HOME/git/config file if this file exists and the ~/.gitconfig file doesn’t.
/// --global
pub fn global() -> FnOptionArg {
    optionarg::simple("--global")
}

/// For writing options: write to system-wide $(prefix)/etc/gitconfig rather than the repository .git/config.
/// --system
pub fn system() -> FnOptionArg {
    optionarg::simple("--system")
}

/// For writing options: write to the repository .git/config file. This is the default behavior.
/// --local
pub fn local() -> FnOptionArg {
    optionarg::simple("--local")
}

/// Use the given config file instead of the one specified by GIT_CONFIG.
/// -f <config-file>, --file <config-file>
pub fn file(config_file_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--file", config_file_arg)
}

/// Similar to --file but use the given blob instead of a file. E.g. you can use master:.gitmodules to read values from the file .gitmodules in the master branch. See 'SPECIFYING REVISIONS' section in gitrevisions(7) for a more complete list of ways to spell blob names.
/// --blob <blob>
pub fn blob(blob_arg: &str) -> FnOptionArg {
    optionarg::with_parameter("--blob", blob_arg)
}

/// List all variables set in config file, along with their values.
/// -l, --list
pub fn list() -> FnOptionArg {
    optionarg::simple("--list")
}

/// git config will ensure that the output is 'true' or 'false'
/// --bool
pub fn bool() -> FnOptionArg {
    optionarg::simple("--bool")
}

/// git config will ensure that the output is a simple decimal number. An optional value suffix of k, m, or g in the config file will cause the value to be multiplied by 1024, 1048576, or 1073741824 prior to output.
/// --int
pub fn int() -> FnOptionArg {
    optionarg::simple("--int")
}

/// git config will ensure that the output matches the format of either --bool or --int, as described above.
/// --bool-or-int
pub fn bool_or_int() -> FnOptionArg {
    optionarg::simple("--bool-or-int")
}

/// git-config will expand leading ~ to the value of $HOME, and ~user to the home directory for the specified user. This option has no effect when setting the value (but you can use git config bla ~/ from the command line to let your shell do the expansion).
/// --path
pub fn path() -> FnOptionArg {
    optionarg::simple("--path")
}

/// For all options that output values and/or keys, always end values with the null character (instead of a newline). Use newline instead as a delimiter between key and value. This allows for secure parsing of the output without getting confused e.g. by values that contain line breaks.
/// -z, --null
pub fn null() -> FnOptionArg {
    optionarg::simple("--null")
}

/// Output only the names of config variables for --list or --get-regexp.
/// --name-only
pub fn name_only() -> FnOptionArg {
    optionarg::simple("--name-only")
}

/// Augment the output of all queried config options with the origin type (file, standard input, blob, command line) and the actual origin (config file path, ref, or blob id if applicable).
/// --show-origin
pub fn show_origin() -> FnOptionArg {
    optionarg::simple("--show-origin")
}

/// Opens an editor to modify the specified config file; either --system, --global, or repository (default).
/// -e, --edit
pub fn edit() -> FnOptionArg {
    optionarg::simple("--edit")
}
