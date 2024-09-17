use crate::command_executor::{CommandExecutor, CommandOption};

/// For writing options: write to global ~/.gitconfig file rather than the repository .git/config, write to $XDG_CONFIG_HOME/git/config file if this file exists and the ~/.gitconfig file doesn’t.
/// --global
pub fn global_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--global"))
}

/// For writing options: write to system-wide $(prefix)/etc/gitconfig rather than the repository .git/config.
/// --system
pub fn system_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--system"))
}

/// For writing options: write to the repository .git/config file. This is the default behavior.
/// --local
pub fn local_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--local"))
}

/// Use the given config file instead of the one specified by GIT_CONFIG.
/// -f <config-file>, --file <config-file>
pub fn file_option(config_file_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--file");
        g.add_option_string(format!("{}", config_file_arg ));
    })
}


/// Similar to --file but use the given blob instead of a file. E.g. you can use master:.gitmodules to read values from the file .gitmodules in the master branch. See 'SPECIFYING REVISIONS' section in gitrevisions(7) for a more complete list of ways to spell blob names.
/// --blob <blob>
pub fn blob_option(blob_arg :&str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--blob");
        g.add_option_string(format!("{}", blob_arg ));
    })
}


/// List all variables set in config file, along with their values.
/// -l, --list
pub fn list_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--list"))
}

/// git config will ensure that the output is 'true' or 'false'
/// --bool
pub fn bool_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--bool"))
}

/// git config will ensure that the output is a simple decimal number. An optional value suffix of k, m, or g in the config file will cause the value to be multiplied by 1024, 1048576, or 1073741824 prior to output.
/// --int
pub fn int_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--int"))
}

/// git config will ensure that the output matches the format of either --bool or --int, as described above.
/// --bool-or-int
pub fn bool_or_int_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--bool-or-int"))
}

/// git-config will expand leading ~ to the value of $HOME, and ~user to the home directory for the specified user. This option has no effect when setting the value (but you can use git config bla ~/ from the command line to let your shell do the expansion).
/// --path
pub fn path_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--path"))
}

/// For all options that output values and/or keys, always end values with the null character (instead of a newline). Use newline instead as a delimiter between key and value. This allows for secure parsing of the output without getting confused e.g. by values that contain line breaks.
/// -z, --null
pub fn null_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--null"))
}

/// Output only the names of config variables for --list or --get-regexp.
/// --name-only
pub fn name_only_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--name-only"))
}

/// Augment the output of all queried config options with the origin type (file, standard input, blob, command line) and the actual origin (config file path, ref, or blob id if applicable).
/// --show-origin
pub fn show_origin_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--show-origin"))
}

/// Opens an editor to modify the specified config file; either --system, --global, or repository (default).
/// -e, --edit
pub fn edit_option() -> CommandOption<'static> {
    Box::new(|g: &mut CommandExecutor| g.add_option("--edit"))
}