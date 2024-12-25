use crate::option_arg;
use crate::wrap_command::FnOptionArg;

/// Set a configuration variable in the newly-created repository; this takes effect immediately after the repository is initialized, but before the remote history is fetched or any files checked out. The key is in the same format as expected by git-config(1) (e.g., core.eol=true). If multiple values are given for the same key, each value will be written to the config file. This makes it safe, for example, to add additional fetch refspecs to the origin remote.
/// --config <key>=<value>, -c <key>=<value>
pub fn config(key :&str, value: &str) -> FnOptionArg {
    let kv = format!("{}={}", key, value);
    option_arg::with_parameter("--config", kv.as_str())
}
