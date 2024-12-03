
# [GitWrap](https://crates.io/crates/gitwrap)

[![Build Status](https://github.com/japiber/gitwrap/actions/workflows/rust.yml/badge.svg)](https://github.com/japiber/gitwrap/actions/workflows/rust.yml) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/xuri/rust-reportcard/blob/master/LICENSE)
[![](https://img.shields.io/crates/v/gitwrap.svg)](https://crates.io/crates/gitwrap)  

GitWrap is a simple wrapper around `git` command.

The purpose of this library is to provide a controlled and reliable method of accessing the git commands in the simplest possible way.

## Credits

This project is inspired and based on [Go Git Cmd Wrapper](https://github.com/ldez/go-git-cmd-wrapper)

## License

The code is licensed under the permissive Apache v2.0 licence. This means you can do what you like with the software, as long as you include the required notices. [Read this](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0)) for a summary.

## Install

```
cargo install gitwrap
```

Running the above command will globally install the gitwrap binary.
Install as library

Run the following Cargo command in your project directory:
```
cargo add gitwrap
```

Or add the following line to your Cargo.toml:
```
gitwrap = "0.3.6"
```

## Usage

Here are some examples of use (work in progress)

### 1. cloning a remote repo

```rust
use gitwrap::{clone, CommandOption};
use gitwrap::options::clone;


fn initialize(repo_url: &str, repo_path: &str, insecure: bool) -> Result<(), RepoErr> {
    let mut opt: Vec<CommandOption> = Vec::new();
    opt.push(clone::repository(repo_url));
    opt.push(clone::directory(repo_path));
    if let Some(auth_header) = build_auth_header() {
        opt.push(clone::config("http.extraHeader", &auth_header))
    }
    if insecure {
        opt.push(clone::config("http.sslVerify", "false"))
    }

    match clone(opt) {
        Ok(_) => Ok(()),
        Err(e) => Err(RepoErr::InitializeError(Box::new(e))),
    }
}
```

### 2. setting repo configuration

```rust
use gitwrap::{config, CommandOption, ExecResult};
use gitwrap::options::config;

fn set_repo_config(commit_user: &str, commit_email: &str) -> Result<(), RepoStoreErr> {
    match (config(config::entry("user.email", commit.commit_email)),
           config(config::entry("user.name", commit.commit_user))) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), _) => Err(RepoStoreErr::InitializeError(Box::new(e))),
        (_, Err(e)) => Err(RepoStoreErr::InitializeError(Box::new(e))),
    }
}
```
