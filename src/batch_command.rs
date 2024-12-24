use crate::wrap_command::WrapCommand;
use crate::WrapError;

pub struct BatchCommand {
    commands: Vec<WrapCommand>
}

impl BatchCommand {
    pub fn new<C>(commands: C) -> BatchCommand
    where C: IntoIterator<Item=WrapCommand>
    {
        Self {
            commands: commands.into_iter().collect::<Vec<WrapCommand>>()
        }
    }

    pub fn add_command(self, command: WrapCommand) -> Self {
        let mut v1 = self.commands.clone();
        v1.push(command);
        Self {
            commands: v1,
        }
    }

    pub fn add_commands<I>(&self, args: I) -> Self
    where
        I: IntoIterator<Item = WrapCommand>
    {
        let mut v1 = self.commands.clone();
        let mut v2 = args.into_iter().collect::<Vec<WrapCommand>>();
        v1.append(&mut v2);
        Self {
            commands: v1,
        }
    }

    pub fn run(&self, current_dir: Option<&str>) -> Result<Vec<String>, WrapError> {
        let mut result: Vec<String> = vec![];
        for command in &self.commands {
            match command.run(current_dir) {
                Ok(r) => result.push(r),
                Err(e) => return Err(e)
            }
        }
        Ok(result)
    }

    pub fn dry_run(&self) -> Result<Vec<String>, WrapError> {
        let mut result: Vec<String> = vec![];
        for command in &self.commands {
            match command.dry_run() {
                Ok(r) => result.push(r),
                Err(e) => return Err(e)
            }
        }
        Ok(result)
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

            $crate::batch_command::BatchCommand::new(commands).run(Some($path))
        }
     );
    (options: $($command:expr), *) => (
        {
            let mut commands: Vec<$crate::wrap_command::WrapCommand> = vec![];
            $(
                commands.push($command);
            )*

            $crate::batch_command::BatchCommand::new(commands).run(None)
        }
     );
}