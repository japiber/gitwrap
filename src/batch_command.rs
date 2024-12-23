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

    pub fn add_command(&mut self, command: WrapCommand) {
        self.commands.push(command);
    }

    pub fn execute(&self)-> Result<Vec<String>, WrapError> {
        let mut result: Vec<String> = vec![];
        for command in &self.commands {
            match command.execute() {
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
    ($($command:expr), *) => (
        {
            let mut commands: Vec<$crate::wrap_command::WrapCommand> = vec![];
            $(
                commands.push($command);
            )*

            $crate::batch_command::BatchCommand::new(commands).execute()
        }
     );
}