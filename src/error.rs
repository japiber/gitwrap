use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum WrapError {
    FailedExecuteProcess(String),
    ExitStatus(String, i32),
}

impl Error for WrapError {}

impl WrapError {
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WrapError::FailedExecuteProcess(s) => write!(f, "failed to execute process: {}", s),
            WrapError::ExitStatus(o, x) => write!(f, "exit status: {}: {}", x, o),
        }
    }
}

impl Display for WrapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl Debug for WrapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}
