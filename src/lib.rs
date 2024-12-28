mod option_arg;
pub mod wrap_command;
mod error;
pub use error::*;

mod git_command;
pub use git_command::*;

pub mod batch_command;


#[cfg(test)]
mod tests;

pub mod tag;
pub mod add;
pub mod branch;
pub mod checkout;
pub mod clean;
pub mod clone;
pub mod commit;
pub mod config;
pub mod fetch;
pub mod init;
pub mod ls_files;
pub mod merge;
pub mod notes;
pub mod pull;
pub mod push;
pub mod rebase;
pub mod reset;
pub mod rev_parse;
pub mod status;
pub mod generic;