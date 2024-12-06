use std::fs;
use crate::clone;


#[test]
fn test_clone() {
    fs::create_dir("tmp2").unwrap();

    let mut cmd = clone::clone("");
    cmd.option(clone::repository("https://github.com/japiber/gitwrap.git"));
    cmd.option(clone::directory("tmp2"));
    cmd.option(clone::config("http.sslVerify", "false"));

    assert!(cmd.execute().is_ok());

    fs::remove_dir_all("tmp2").unwrap();
}
