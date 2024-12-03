# GitRack: Git command Wrapper

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/japiber/gitrack?style=flat)](https://rust-reportcard.xuri.me/badge/github.com/japiber/gitrack?style=flat


GitRack is a simple wrapper around `git` command.

```rust
use gitrack::{clone, CommandOption};
use gitrack::options::clone;


fn initialize(&self) -> Result<(), RepoErr> {
    let mut opt: Vec<CommandOption> = Vec::new();
    opt.push(clone::repository(self.repo_url));
    opt.push(clone::directory(self.repo_path));
    if let Some(auth_header) = self.build_auth_header() {
        opt.push(clone::config("http.extraHeader", &auth_header))
    }
    if self.auth.insecure {
        opt.push(clone::config("http.sslVerify", "false"))
    }

    match clone(opt) {
        Ok(_) => Ok(()),
        Err(e) => Err(RepoErr::InitializeError(Box::new(e))),
    }
}

```
