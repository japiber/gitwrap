use crate::{clone, config, rev_parse};
use rand::Rng;
use std::fs;


const REPO_CONFIG_EMAIL: &str = "test@email.com";

#[test]
fn test_clone() {
    let mut rng = rand::rng();
    let path = format!("repo_{:10x}!", rng.random_range(0..10000000));
    fs::create_dir(path.as_str()).unwrap();

    {
        let mut cmd = clone::clone(None);
        cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
        cmd.option(clone::directory(path.as_str()));
        cmd.option(clone::config("http.sslVerify", "false"));

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
    let mut rng = rand::rng();
    let path = format!("repo_{:10x}!", rng.random_range(0..10000000));
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd = clone!(None,
            clone::repository("https://github.com/japiber/gitwrap.git"),
            clone::directory(path.as_str()),
            clone::config("http.sslVerify", "false"));

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
    let mut rng = rand::rng();
    let path = format!("repo_{:10x}!", rng.random_range(0..10000000));
    fs::create_dir(path.as_str()).unwrap();

    {
        let mut cmd = clone::clone(None);
        cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
        cmd.option(clone::directory(path.as_str()));

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
