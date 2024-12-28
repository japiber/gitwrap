use crate::option_arg;
use crate::wrap_command::FnOptionArg;

/// Pass a configuration parameter to the command.
/// The value given will override values from configuration files.
/// The <name> is expected in the same format as listed by git config (subkeys separated by dots).
/// -c <name>=<value>
pub fn config(name :&str, value: &str) -> FnOptionArg {
    let kv = format!("{}={}", name, value);
    option_arg::with_parameter("-c", kv.as_str())
}

/// Like -c <name>=<value>, give configuration variable <name> a value, where <envvar> is the name of an environment variable from which to retrieve the value.
/// Unlike -c there is no shortcut for directly setting the value to an empty string, instead the environment variable itself must be set to the empty string.
/// It is an error if the <envvar> does not exist in the environment. <envvar> may not contain an equals sign to avoid ambiguity with <name> containing one.
/// --config-env <name>=<envvar>
pub fn config_env(name :&str, envvar: &str) -> FnOptionArg {
    let kv = format!("{}={}", name, envvar);
    option_arg::with_parameter("--config-env", kv.as_str())
}