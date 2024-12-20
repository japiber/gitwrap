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
            let mut command = git(pull::GIT_COMMAND, $path);
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
            let mut command = git(fetch::GIT_COMMAND, $path);
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
            let mut command = git(init::GIT_COMMAND, $path);
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
            let mut command = git(rebase::GIT_COMMAND, $path);
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
            let mut command = git(push::GIT_COMMAND, $path);
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
            let mut command = git(clone::GIT_COMMAND, $path);
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
            let mut command = git(checkout::GIT_COMMAND, $path);
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
            let mut command = git(config::GIT_COMMAND, $path);
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
            let mut command = git(reset::GIT_COMMAND, $path);
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
            let mut command = git(commit::GIT_COMMAND, $path);
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
            let mut command = git(add::GIT_COMMAND, $path);
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
            let mut command = git(merge::GIT_COMMAND, $path);
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
            let mut command = git(rev_parse::GIT_COMMAND, $path);
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
            let mut command = git(tag::GIT_COMMAND, $path);
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
            let mut command = git(status::GIT_COMMAND, $path);
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
            let mut command = git(notes::GIT_COMMAND, $path);
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
            let mut command = git(ls_files::GIT_COMMAND, $path);
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
            let mut command = git(branch::GIT_COMMAND, $path);
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
            let mut command = git(clean::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command
        }
     }
}
