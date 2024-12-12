use crate::wrap_command::WrapCommand;

pub fn git(cmd: &str, current_dir: Option<&str>) -> WrapCommand {
    let mut command = WrapCommand::new("git", current_dir);
    let l_cmd = String::from(cmd);
    command.option(Box::new(move |c: &mut  std::process::Command| { c.arg(l_cmd.as_str()); }));
    command
}

#[macro_export]
macro_rules! pull {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("pull", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! fetch {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("fetch", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! init {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("init", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! rebase {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("rebase", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! push {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("push", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! clone {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("clone", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! checkout {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("checkout", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! config {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("config", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! reset {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("reset", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! commit {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("commit", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! add {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("add", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! merge {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("merge", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! rev_parse {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("rev-parse", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! tag {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("tag", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! status {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("status", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! notes {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("notes", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! ls_files {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("ls-files", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! branch {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("branch", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}

#[macro_export]
macro_rules! clean {
    ($path:expr,
     $($options:expr), *) => {
        {
            let mut command = crate::git_command::git("clean", $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}
