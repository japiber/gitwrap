use crate::command_executor::{command, CommandExecutor, CommandOption, ExecResult};


fn git_command<'a, I>(cmd: &str, options: I) -> CommandExecutor
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    command("git", cmd, options)
}


pub fn pull<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("pull", options).exec()
}


pub fn fetch<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("fetch", options).exec()
}


pub fn init<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("init", options).exec()
}


pub fn rebase<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("rebase", options).exec()
}


pub fn push<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("push", options).exec()
}


pub fn clone<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("clone", options).exec()
}


pub fn checkout<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("checkout", options).exec()
}


pub fn config<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("config", options).exec()
}


pub fn reset<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("reset", options).exec()
}


pub fn commit<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("commit", options).exec()
}


pub fn add<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("add", options).exec()
}


pub fn merge<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("merge", options).exec()
}


pub fn revparse<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("revparse", options).exec()
}


pub fn tag<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("tag", options).exec()
}


pub fn status<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("status", options).exec()
}


pub fn notes<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("notes", options).exec()
}


pub fn ls_files<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("ls-files", options).exec()
}


pub fn branch<'a, I>(options: I) -> ExecResult
where
    I: IntoIterator<Item=CommandOption<'a>>
{
    git_command("branch", options).exec()
}
