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
            $crate::git(pull::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(pull::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(pull::GIT_COMMAND, $path);
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
            $crate::git(fetch::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(fetch::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(fetch::GIT_COMMAND, $path);
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
            $crate::git(init::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(init::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(init::GIT_COMMAND, $path);
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
            $crate::git(rebase::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(rebase::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(rebase::GIT_COMMAND, $path);
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
            $crate::git(push::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(push::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(push::GIT_COMMAND, $path);
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
            $crate::git(clone::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(clone::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(clone::GIT_COMMAND, $path);
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
            $crate::git(checkout::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(checkout::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(checkout::GIT_COMMAND, $path);
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
            $crate::git(config::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(config::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(config::GIT_COMMAND, $path);
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
            $crate::git(reset::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(reset::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(reset::GIT_COMMAND, $path);
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
            $crate::git(commit::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(commit::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(commit::GIT_COMMAND, $path);
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
            $crate::git(add::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(add::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(add::GIT_COMMAND, $path);
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
            $crate::git(merge::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(merge::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(merge::GIT_COMMAND, $path);
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
            $crate::git(rev_parse::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(rev_parse::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(rev_parse::GIT_COMMAND, $path);
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
            $crate::git(tag::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(tag::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(tag::GIT_COMMAND, $path);
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
            $crate::git(status::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(status::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(status::GIT_COMMAND, $path);
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
            $crate::git(notes::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(notes::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(notes::GIT_COMMAND, $path);
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
            $crate::git(ls_files::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(ls_files::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(ls_files::GIT_COMMAND, $path);
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
            $crate::git(branch::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(branch::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(branch::GIT_COMMAND, $path);
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
            $crate::git(clean::GIT_COMMAND, None).execute()
        }
    );
    ($path:expr) => (
        {
            $crate::git(clean::GIT_COMMAND, Some($path)).execute()
        }
    );
    ($path:expr, $($options:expr), *) => (
        {
            let mut command = $crate::git(clean::GIT_COMMAND, $path);
            $(
                command.option($options);
            )*
            command.execute()
        }
     );
}
