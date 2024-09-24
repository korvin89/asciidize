use crate::alphabet_mapper;
use crate::utils;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Alphabet commands
    Alphabet {
        #[command(subcommand)]
        subcommand: Option<alphabet_mapper::cli::AlphabetCommand>,
    },
}

/// # Errors
/// Returns an error if the command is not provided
/// Returns subcommand errors
pub fn root(
    command: &Option<Commands>,
    mut stdout: impl std::io::Write,
    mut stderr: impl std::io::Write,
) -> Result<(), utils::cli::CommandError> {
    match command {
        Some(Commands::Alphabet { subcommand }) => {
            return alphabet_mapper::cli::run(subcommand, &mut stdout, &mut stderr);
        }
        None => {
            return none();
        }
    }
}

fn none() -> Result<(), utils::cli::CommandError> {
    return Err(utils::cli::CommandError {
        message: "No command provided",
        exit_code: exitcode::USAGE,
    });
}
