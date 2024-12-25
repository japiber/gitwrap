use crate::wrap_command::WrapCommand;
use crate::WrapError;

pub struct BatchCommand {
    commands: Vec<WrapCommand>,
    current_dir: Option<String>
}

impl Default for BatchCommand {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchCommand {
    /// Creates a new empty BatchCommand
    pub fn new() -> Self {
        Self {
            commands: vec![],
            current_dir: None
        }
    }

    /// Set this BatchCommand current_dir to [dir]
    /// When [current_dir] is set, all the commands working directory are set to it at run.
    pub fn current_dir(&self, dir: &str) -> Self {
        Self {
            commands: self.commands.clone(),
            current_dir: Some(dir.to_string())
        }
    }

    /// Includes a new WrapCommand to execute
    pub fn add_command(self, command: WrapCommand) -> Self {
        let mut v1 = self.commands.clone();
        v1.push(command);
        Self {
            commands: v1,
            current_dir: self.current_dir.clone()
        }
    }

    /// Includes several new WrapCommands at only once
    pub fn add_commands<I>(&self, args: I) -> Self
    where
        I: IntoIterator<Item = WrapCommand>
    {
        let mut v1 = self.commands.clone();
        let mut v2 = args.into_iter().collect::<Vec<WrapCommand>>();
        v1.append(&mut v2);
        Self {
            commands: v1,
            current_dir: self.current_dir.clone()
        }
    }

    /// Executes all the previously included WrapCommands in the same insertions order
    /// returns the commands output in a vector or a WrapError of the first failing command
    pub fn run(&self) -> Result<Vec<String>, WrapError> {
        let mut result: Vec<String> = vec![];
        for cmd in &self.commands {
            match self.prepare_command(cmd).run() {
                Ok(r) => result.push(r),
                Err(e) => return Err(e)
            }
        }
        Ok(result)
    }

    /// Dry-runs all the previously included WrapCommands in the same insertions order
    /// returns the full commands specifications provided in a vector or a WrapError of the first failing command
    pub fn dry_run(&self) -> Result<Vec<String>, WrapError> {
        let mut result: Vec<String> = vec![];
        for cmd in &self.commands {
            match self.prepare_command(cmd).dry_run() {
                Ok(r) => result.push(r),
                Err(e) => return Err(e)
            }
        }
        Ok(result)
    }

    fn prepare_command(&self, command: &WrapCommand) -> WrapCommand {
        if let Some(dir) = &self.current_dir {
            command.current_dir(dir.as_str())
        } else {
            command.clone()
        }
    }
}

#[macro_export]
macro_rules! batch {
    (path: $path:expr, commands: $($command:expr), *) => (
        {
            let mut commands: Vec<$crate::wrap_command::WrapCommand> = vec![];
            $(
                commands.push($command);
            )*

            $crate::batch_command::BatchCommand::new().add_commands(commands)
            .current_dir($path)
            .run()
        }
     );
    (options: $($command:expr), *) => (
        {
            let mut commands: Vec<$crate::wrap_command::WrapCommand> = vec![];
            $(
                commands.push($command);
            )*

            $crate::batch_command::BatchCommand::new().add_commands(commands).run(None)
        }
     );
}