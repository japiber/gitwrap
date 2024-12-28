
# [GitWrap](https://crates.io/crates/gitwrap)

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/japiber/gitwrap?style=flat)](https://rust-reportcard.xuri.me/badge/github.com/japiber/gitwrap?style=flat)[![Build Status](https://github.com/japiber/gitwrap/actions/workflows/rust.yml/badge.svg)](https://github.com/japiber/gitwrap/actions/workflows/rust.yml) [![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/xuri/rust-reportcard/blob/master/LICENSE)
[![](https://img.shields.io/crates/v/gitwrap.svg)](https://crates.io/crates/gitwrap)  


GitWrap is a simple wrapper around `git` command.

The purpose of this library is to provide a controlled and reliable method of accessing the git commands in the simplest possible way.

This project is in progress, not all git commands / options are implemented yet.

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
gitwrap = "0.11.0"
```

## Usage

Here are some examples of use (work in progress)

### 1. Cloning a remote repo

```rust
use gitwrap::clone;


fn initialize(repo_url: &str, repo_path: &str) {
    let cmd = clone::clone()
        .add_option(clone::repository(repo_url))
        .add_option(clone::directory(repo_path));

    assert!(cmd.run().is_ok());
}
```

### Clone a repo using macros. Macros allow to specify all command options and execute it in a single step

```rust
fn initialize(repo_url: &str, repo_path: &str) {
    assert!(
        clone!(
            options:
                clone::repository("https://github.com/japiber/gitwrap.git"),
                clone::directory(path.as_str())
        ).is_ok()
    );
}
```

### Fine-grained control over the options command

```rust
fn set_repo_config(commit_email: &str, repo_path: &str) {
    let mut cmd = config::config();
    if (!commit_email.is_empty()) {
        cmd = cmd.add_option(config::entry("user.email", commit_email));
    }

    assert!(cmd.current_dir(repo_path).run().is_ok());
}
```

### Execute a series of git commands at once with a BatchCommand or using the batch! macro

```rust
fn clean_repo(path: &str) {
    let s_path = Some(path);
    assert!(
        batch!(
            path: 
                s_path,
            commands:
                reset::reset(),
                checkout::checkout().add_option(checkout::pathspec(".")),
                clean::clean().add_options(vec![
                    clean::force(), 
                    clean::recurse_directories(), 
                    clean::no_gitignore()
                ])
        ).is_ok()
    );
}
```
