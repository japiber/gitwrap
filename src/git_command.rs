use crate::wrap_command::WrapCommand;

pub fn git(current_dir: &str, cmd: &str) -> WrapCommand {
    let mut command = WrapCommand::new("git", current_dir);
    let l_cmd = String::from(cmd);
    command.option(Box::new(move |c: &mut  std::process::Command| { c.arg(l_cmd.as_str()); }));
    command
}

#[macro_export]
macro_rules! commit {
    ($path:expr,
     $($options:expr), *) => {{
        let mut command = crate::git_command::git($path, "commit");
        $(
            command.option($options);
        )*
        command
    }}
}

pub use commit;

#[macro_export]
macro_rules! clone {
    ($path:expr,
     $($options:expr), *) => {{
        let mut command = crate::git_command::git($path, "clone");
        $(
            command.option($options);
        )*
        command
    }}
}

pub use clone;
