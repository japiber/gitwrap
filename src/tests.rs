use crate::{clone, config, rev_parse, reset, checkout, clean, batch};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::wrap_command::WrapCommand;

const REPO_CONFIG_EMAIL: &str = "test@email.com";
const REPO_URL: &str = "https://github.com/japiber/gitwrap.git";


#[test]
fn test_clone() {
    let path = gitwrap_test_path();
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd_set = format!("git clone {} {}", REPO_URL, path);
        let cmd = cmd_clone(&path);
        assert!(cmd.dry_run().unwrap().eq(&cmd_set));
        assert!(cmd.run().is_ok());
    }

    {
        let cmd_set = String::from("git rev-parse --is-inside-work-tree");
        let cmd = rev_parse::rev_parse().add_option(rev_parse::is_inside_work_tree());
        assert!(cmd.dry_run().unwrap().eq(&cmd_set));

        let r = cmd.current_dir(path.as_str()).run();
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
        assert!(
            clone!(
                options:
                    clone::repository("https://github.com/japiber/gitwrap.git"),
                    clone::directory(path.as_str()
                )
            ).is_ok());
    }

    {
       assert!(
           rev_parse!(
               path: path.as_str(),
               options: rev_parse::is_inside_work_tree()
           ).ok().unwrap().contains("true"));
    }

    fs::remove_dir_all(path.as_str()).unwrap();
}


#[test]
fn test_config() {
    let path = gitwrap_test_path();
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd = cmd_clone(&path);
        assert!(cmd.run().is_ok());
    }

    {
        let config_key = "user.email";
        let cmd_set = format!("git {} {} {}", config::GIT_COMMAND, config_key, REPO_CONFIG_EMAIL);
        let cmd = config::config().add_option(config::entry(config_key, REPO_CONFIG_EMAIL));

        assert!(cmd.dry_run().unwrap().eq(&cmd_set));

        assert!(cmd.current_dir(path.as_str()).run().is_ok());
    }

    {
        let cmd_set = String::from("git config --get user.email");
        let cmd = config::config().add_option(config::get("user.email", ""));
        assert!(cmd.dry_run().unwrap().eq(&cmd_set));

        let r = cmd.current_dir(path.as_str()).run();
        assert!(r.is_ok());
        assert!(r.ok().unwrap().contains(REPO_CONFIG_EMAIL));
    }

    fs::remove_dir_all(path.as_str()).unwrap();
}

#[test]
fn test_batch() {
    let path = gitwrap_test_path();
    fs::create_dir(path.as_str()).unwrap();

    {
        let cmd = cmd_clone(&path);
        assert!(cmd.run().is_ok());
    }

    {
        assert!(fs::remove_file(format!("{}/{}", path, "README.md")).is_ok());
        assert!(fs::exists(format!("{}/{}", path, "README.md")).is_ok_and(|x| !x));

        assert!(batch!(
            path:
                path.as_str(),
            commands:
                reset::reset(),
                checkout::checkout().add_option(checkout::pathspec(".")),
                reset::reset().add_option(reset::hard()),
                clean::clean().add_option(clean::force()).add_option(clean::recurse_directories()).add_option(clean::no_gitignore())
        ).is_ok());

        assert!(fs::exists(format!("{}/{}", path, "README.md")).is_ok_and(|x| x));
    }

    fs::remove_dir_all(path.as_str()).unwrap();
}



fn cmd_clone(path: &str) -> WrapCommand {
    clone::clone()
        .add_option(clone::repository("https://github.com/japiber/gitwrap.git"))
        .add_option(clone::directory(path))
}

fn gitwrap_test_path() -> String {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
    format!("gitwrap_test_{:x}", nanos)
}
