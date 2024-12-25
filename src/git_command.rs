use std::sync::Arc;
use crate::wrap_command::{FnOptionArg, WrapCommand};

pub fn git(cmd: &str) -> WrapCommand {
    let l_cmd = String::from(cmd);
    WrapCommand::new("git")
        .add_option(FnOptionArg(Arc::new(move || vec![String::from(l_cmd.as_str())])))
}

#[macro_export]
macro_rules! pull {
    () => (
        {
            $crate::git(pull::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(pull::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(pull::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(pull::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! fetch {
    () => (
        {
            $crate::git(fetch::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(fetch::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(fetch::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(fetch::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! init {
    () => (
        {
            $crate::git(init::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(init::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(init::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(init::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! rebase {
    () => (
        {
            $crate::git(rebase::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(rebase::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(rebase::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(rebase::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! push {
    () => (
        {
            $crate::git(push::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(push::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(push::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(push::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! clone {
    () => (
        {
            $crate::git(clone::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(clone::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(clone::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(clone::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! checkout {
    () => (
        {
            $crate::git(checkout::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(checkout::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(checkout::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(checkout::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! config {
    () => (
        {
            $crate::git(config::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(config::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(config::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(config::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! reset {
    () => (
        {
            $crate::git(reset::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(reset::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(reset::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(reset::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! commit {
    () => (
        {
            $crate::git(commit::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(commit::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(commit::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(commit::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! add {
    () => (
        {
            $crate::git(add::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(add::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(add::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(add::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! merge {
    () => (
        {
            $crate::git(merge::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(merge::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(merge::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(merge::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! rev_parse {
    () => (
        {
            $crate::git(rev_parse::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(rev_parse::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(rev_parse::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(rev_parse::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! tag {
    () => (
        {
            $crate::git(tag::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(tag::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(tag::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(tag::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! status {
    () => (
        {
            $crate::git(status::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(status::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(status::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(status::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! notes {
    () => (
        {
            $crate::git(notes::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(notes::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(notes::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(notes::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! ls_files {
    () => (
        {
            $crate::git(ls_files::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(ls_files::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(ls_files::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(ls_files::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! branch {
    () => (
        {
            $crate::git(branch::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(branch::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(branch::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(branch::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}

#[macro_export]
macro_rules! clean {
    () => (
        {
            $crate::git(clean::GIT_COMMAND).run()
        }
    );
    (path: $path:expr) => (
        {
            $crate::git(clean::GIT_COMMAND).current_dir($path).run()
        }
    );
    (path: $path:expr, options: $($option:expr), *) => (
        {
            let command = $crate::git(clean::GIT_COMMAND)
            $(.add_option($option))*;
            command.current_dir($path).run()
        }
     );
    (options: $($option:expr), *) => (
        {
            let command = $crate::git(clean::GIT_COMMAND)
            $(.add_option($option))*;
            command.run()
        }
     );
}
