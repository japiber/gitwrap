use crate::wrap_command::WrapCommand;

pub fn git(cmd: &str, current_dir: Option<&str>) -> WrapCommand {
    let mut command = WrapCommand::new("git", current_dir);
    let l_cmd = String::from(cmd);
    command.option(Box::new(move |c: &mut  std::process::Command| { c.arg(l_cmd.as_str()); }));
    command
}

#[macro_export]
macro_rules! pull {
    () => (
        {
            git(pull::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(pull::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(pull::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! fetch {
    () => (
        {
            git(fetch::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(fetch::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(fetch::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! init {
    () => (
        {
            git(init::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(init::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(init::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! rebase {
    () => (
        {
            git(rebase::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(rebase::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(rebase::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! push {
    () => (
        {
            git(push::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(push::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(push::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! clone {
    () => (
        {
            git(clone::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(clone::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(clone::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! checkout {
    () => (
        {
            git(checkout::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(checkout::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(checkout::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! config {
    () => (
        {
            git(config::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(config::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(config::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! reset {
    () => (
        {
            git(reset::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(reset::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(reset::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! commit {
    () => (
        {
            git(commit::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(commit::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(commit::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! add {
    () => (
        {
            git(add::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(add::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(add::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! merge {
    () => (
        {
            git(merge::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(merge::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(merge::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! rev_parse {
    () => (
        {
            git(rev_parse::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(rev_parse::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(rev_parse::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! tag {
    () => (
        {
            git(tag::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(tag::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(tag::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! status {
    () => (
        {
            git(status::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(status::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(status::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! notes {
    () => (
        {
            git(notes::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(notes::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(notes::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! ls_files {
    () => (
        {
            git(ls_files::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(ls_files::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(ls_files::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! branch {
    () => (
        {
            git(branch::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(branch::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(branch::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}

#[macro_export]
macro_rules! clean {
    () => (
        {
            git(clean::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            git(clean::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = git(clean::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}
