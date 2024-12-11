use crate::wrap_command::WrapCommand;

pub fn git(current_dir: &str, cmd: &str) -> WrapCommand {
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
            let mut command = crate::git_command::git($path, "pull");
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
            let mut command = crate::git_command::git($path, "fetch");
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
            let mut command = crate::git_command::git($path, "init");
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
            let mut command = crate::git_command::git($path, "rebase");
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
            let mut command = crate::git_command::git($path, "push");
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
            let mut command = crate::git_command::git($path, "clone");
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
            let mut command = crate::git_command::git($path, "checkout");
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
            let mut command = crate::git_command::git($path, "config");
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
            let mut command = crate::git_command::git($path, "reset");
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
            let mut command = crate::git_command::git($path, "commit");
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
            let mut command = crate::git_command::git($path, "add");
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
            let mut command = crate::git_command::git($path, "merge");
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
            let mut command = crate::git_command::git($path, "rev-parse");
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
            let mut command = crate::git_command::git($path, "tag");
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
            let mut command = crate::git_command::git($path, "status");
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
            let mut command = crate::git_command::git($path, "notes");
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
            let mut command = crate::git_command::git($path, "ls-files");
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
            let mut command = crate::git_command::git($path, "branch");
            $(
                command.option($options);
            )*
            command
        }
     }
}
