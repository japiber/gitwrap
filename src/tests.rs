use crate::{clone, config, rev_parse, git};
use rand::Rng;
use std::fs;
use crate::wrap_command::WrapCommand;

const REPO_CONFIG_EMAIL: &str = "test@email.com";

#[test]
fn test_clone() {
    let path = gitwrap_test_path();
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd = cmd_clone(&path);
        assert!(cmd.execute().is_ok());
    }

    {
        let mut cmd = rev_parse::rev_parse(Some(path.as_str()));
        cmd.option(rev_parse::is_inside_work_tree());
        let r = cmd.execute();

        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains("true"));
    }

    fs::remove_dir_all(path.as_str()).unwrap();
}

#[test]
fn test_clone_macro() {
    let path = gitwrap_test_path();
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd = clone!(None,
            clone::repository("https://github.com/japiber/gitwrap.git"),
            clone::directory(path.as_str()));

        assert!(cmd.execute().is_ok());
    }

    {
        let cmd = rev_parse!(Some(path.as_str()),
            rev_parse::is_inside_work_tree());
        let r = cmd.execute();

        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains("true"));
    }

    fs::remove_dir_all(path.as_str()).unwrap();
}


#[test]
fn test_config() {
    let path = gitwrap_test_path();
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd = cmd_clone(&path);
        assert!(cmd.execute().is_ok());
    }

    {
        let cmd = config!(Some(path.as_str()),
            config::entry("user.email", REPO_CONFIG_EMAIL)
        );

        assert!(cmd.execute().is_ok());
    }

    {
        let mut cmd = config::config(Some(path.as_str()));
        cmd.option(config::get("user.email", ""));
        let r = cmd.execute();

        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains(REPO_CONFIG_EMAIL));
    }

    fs::remove_dir_all(path.as_str()).unwrap();
}

fn cmd_clone(path: &str) -> WrapCommand {
    let mut cmd = clone::clone(None);
    cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
    cmd.option(clone::directory(path));
    cmd
}

fn gitwrap_test_path() -> String {
    let mut rng = rand::rng();
    format!("gitwrap_test_{:x}", rng.random_range(0..999999999))
}
