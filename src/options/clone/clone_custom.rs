use crate::{CommandExecutor, CommandOption};

/// Set a configuration variable in the newly-created repository; this takes effect immediately after the repository is initialized, but before the remote history is fetched or any files checked out. The key is in the same format as expected by git-config(1) (e.g., core.eol=true). If multiple values are given for the same key, each value will be written to the config file. This makes it safe, for example, to add additional fetch refspecs to the origin remote.
/// --config <key>=<value>, -c <key>=<value>
pub fn config(key :&str, value: &str) -> CommandOption<'static> {
    let kv = format!("{}={}", key, value);
    Box::new(move |g: &mut CommandExecutor| {
        g.add_option("--config");
        g.add_option(kv.as_str());
    })
}

// The (possibly remote) repository to clone from. See the URLS section below for more information on specifying repositories.
// <repository>
pub fn repository(url: &str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option(url))
}

// The name of a new directory to clone into. The "humanish" part of the source repository is used if no directory is explicitly given (repo for /path/to/repo.git and foo for host.xz:foo/.git). Cloning into an existing directory is only allowed if the directory is empty.
// <directory>
pub fn directory(path: &str) -> CommandOption {
    Box::new(move |g: &mut CommandExecutor| g.add_option(path))
}