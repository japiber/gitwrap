use std::fs;
use crate::{clone, rev_parse};


#[test]
fn test_clone() {
    fs::create_dir("tmp2").unwrap();

    {
        let mut cmd = clone::clone("");
        cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
        cmd.option(clone::directory("tmp2"));
        cmd.option(clone::config("http.sslVerify", "false"));

        assert!(cmd.execute().is_ok());
    }

    {
        let mut cmd = rev_parse::rev_parse("tmp2");
        cmd.option(rev_parse::is_inside_work_tree());
        let r = cmd.execute();

        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains("true"));
    }

    fs::remove_dir_all("tmp2").unwrap();
}
