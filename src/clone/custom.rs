use std::process::Command;
use crate::wrap_command::FnOptionArg;

/// Set a configuration variable in the newly-created repository; this takes effect immediately after the repository is initialized, but before the remote history is fetched or any files checked out. The key is in the same format as expected by git-config(1) (e.g., core.eol=true). If multiple values are given for the same key, each value will be written to the config file. This makes it safe, for example, to add additional fetch refspecs to the origin remote.
/// --config <key>=<value>, -c <key>=<value>
pub fn config(key :&str, value: &str) -> FnOptionArg {
    let kv = format!("{}={}", key, value);
    Box::new(move |cmd: &mut Command| {
        cmd.arg("--config");
        cmd.arg(kv.as_str());
    })
}

/// The (possibly remote) repository to clone from. See the URLS section below for more information on specifying repositories.
/// <repository>
pub fn repository(url: &str) -> FnOptionArg {
    let l_url = String::from(url);
    Box::new(move |cmd: &mut Command| { cmd.arg(l_url.as_str()); } )
}

/// The name of a new directory to clone into. The "humanish" part of the source repository is used if no directory is explicitly given (repo for /path/to/repo.git and foo for host.xz:foo/.git). Cloning into an existing directory is only allowed if the directory is empty.
/// <directory>
pub fn directory(path: &str) -> FnOptionArg {
    let l_path = String::from(path);
    Box::new(move |cmd: &mut Command| { cmd.arg(l_path.as_str()); } )
}