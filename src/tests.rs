use std::fs;
use crate::{clone, config, rev_parse};


const REPO_CLONE_PATH: &str = "tmp";

const REPO_CONFIG_EMAIL: &str = "test@email.com";

#[test]
fn test_clone() {
    fs::create_dir(REPO_CLONE_PATH).unwrap();

    {
        let mut cmd = clone::clone("");
        cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
        cmd.option(clone::directory(REPO_CLONE_PATH));
        cmd.option(clone::config("http.sslVerify", "false"));

        assert!(cmd.execute().is_ok());
    }

    {
        let mut cmd = rev_parse::rev_parse(REPO_CLONE_PATH);
        cmd.option(rev_parse::is_inside_work_tree());
        let r = cmd.execute();

        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains("true"));
    }

    fs::remove_dir_all(REPO_CLONE_PATH).unwrap();
}

#[test]
fn test_config() {
    fs::create_dir(REPO_CLONE_PATH).unwrap();

    {
        let mut cmd = clone::clone("");
        cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
        cmd.option(clone::directory(REPO_CLONE_PATH));

        assert!(cmd.execute().is_ok());
    }

    {
        let mut cmd = config::config(REPO_CLONE_PATH);
        cmd.option(config::entry("user.email", REPO_CONFIG_EMAIL));

        assert!(cmd.execute().is_ok());
    }

    {
        let mut cmd = config::config(REPO_CLONE_PATH);
        cmd.option(config::get("user.email", ""));
        let r = cmd.execute();

        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains(REPO_CONFIG_EMAIL));
    }

    fs::remove_dir_all(REPO_CLONE_PATH).unwrap();

}
