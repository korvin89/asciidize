use crate::alphabet::{Alphabet, Sample};
use crate::constants;
use crate::utils;
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum Command {
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
    command: &Option<Command>,
    mut stdout: impl std::io::Write,
    mut stderr: impl std::io::Write,
) -> Result<(), utils::cli::CommandError> {
    match command {
        Some(Command::Print { width, alphabet }) => {
            return print(width, alphabet, &mut stdout, &mut stderr);
        }
        None => {
            return none(&mut stdout, &mut stderr);
        }
    }
}

fn print(
    width: &Option<usize>,
    alphabet_string: &Option<String>,
    mut stdout: impl std::io::Write,
    mut _stderr: impl std::io::Write,
) -> Result<(), utils::cli::CommandError> {
    let alphabet = Alphabet::from_symbol_str(match alphabet_string {
        Some(alphabet_string) => alphabet_string,
        None => constants::ALPHABET_SYMBOL_STR,
    });

    let mut sample = Sample::from_alphabet(
        &alphabet,
        width.unwrap_or(constants::ALPHABET_SAMPLE_WIDTH),
        constants::ALPHABET_SAMPLE_FILLER_SYMBOL,
    );
    sample.add_padding(
        constants::ALPHABET_SAMPLE_PADDING_SYMBOL,
        constants::ALPHABET_SAMPLE_PADDING_WIDTH,
    );

    writeln!(stdout, "{sample}").unwrap();

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
