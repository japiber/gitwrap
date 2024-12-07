
# [GitWrap](https://crates.io/crates/gitwrap)

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/japiber/gitwrap?style=flat)](https://rust-reportcard.xuri.me/badge/github.com/japiber/gitwrap?style=flat)[![Build Status](https://github.com/japiber/gitwrap/actions/workflows/rust.yml/badge.svg)](https://github.com/japiber/gitwrap/actions/workflows/rust.yml) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/xuri/rust-reportcard/blob/master/LICENSE)
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
gitwrap = "0.4.0"
```

## Usage

Here are some examples of use (work in progress)

### 1. Cloning a remote repo

```rust
use crate::{clone};


fn initialize(repo_url: &str, repo_path: &str) {
    let mut cmd = clone::clone("");
    cmd.option(clone::repository(repo_url));
    cmd.option(clone::directory(repo_path));
    cmd.option(clone::config("http.sslVerify", "false"));

    assert!(cmd.execute().is_ok());
}
```

### 2. Setting repo configuration

```rust
use crate::{config};

fn set_repo_config(commit_email: &str) {
    let mut cmd = config::config(REPO_CLONE_PATH);
    cmd.option(config::entry("user.email", commit_email));

    assert!(cmd.execute().is_ok());
}
```

### 3. Check if a directory is a valid git repo

```rust
use crate::{rev_parse};

fn is_repo_valid(repo_path: &str) {
    let mut cmd = rev_parse::rev_parse(repo_path);
    cmd.option(rev_parse::is_inside_work_tree());
    let r = cmd.execute();

    assert!(r.is_ok());
    assert!(r.ok().unwrap().contains("true"));
}
```
