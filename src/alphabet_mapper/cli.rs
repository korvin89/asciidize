use crate::alphabet_mapper::alphabet::Alphabet;
use crate::utils;
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum AlphabetCommand {
    /// Prints alphabet
    Print {
        #[clap(short, long)]
        width: Option<usize>,

        #[clap(short, long)]
        alphabet: Option<String>,
    },
}

/// # Errors
/// Returns an error if the command is not provided
pub fn run(
    command: &Option<AlphabetCommand>,
    mut stdout: impl std::io::Write,
    mut stderr: impl std::io::Write,
) -> Result<(), utils::cli::CommandError> {
    match command {
        Some(AlphabetCommand::Print { width, alphabet }) => {
            return print(width, alphabet, &mut stdout, &mut stderr);
        }
        None => {
            return none(&mut stdout, &mut stderr);
        }
    }
}

const DEFAULT_WIDTH: usize = 20;

fn print(
    width: &Option<usize>,
    alphabet_string: &Option<String>,
    mut stdout: impl std::io::Write,
    mut _stderr: impl std::io::Write,
) -> Result<(), utils::cli::CommandError> {
    let width = width.unwrap_or(DEFAULT_WIDTH);

    let alphabet = match alphabet_string {
        Some(alphabet_string) => Alphabet::from_symbol_str(alphabet_string),
        None => Alphabet::from_default(),
    };

    let sample_string = alphabet.to_sample_string(width, 1, 1, 1, 1, ' ');
    writeln!(stdout, "{sample_string}").unwrap();

    return Ok(());
}

fn none(
    mut _stdout: impl std::io::Write,
    mut _stderr: impl std::io::Write,
) -> Result<(), utils::cli::CommandError> {
    return Err(utils::cli::CommandError {
        message: "No command provided",
        exit_code: exitcode::USAGE,
    });
}
