use crate::alphabet::Alphabet;
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
    let alphabet = match alphabet_string {
        Some(alphabet_string) => Alphabet::from_symbol_str(alphabet_string),
        None => Alphabet::from_default(),
    };

    let sample_string = alphabet.to_sample_string(
        width.unwrap_or(constants::ALPHABET_SAMPLE_STRING_DEFAULT_WIDTH),
        constants::ALPHABET_SAMPLE_STRING_DEFAULT_LEFT_PADDING,
        constants::ALPHABET_SAMPLE_STRING_DEFAULT_RIGHT_PADDING,
        constants::ALPHABET_SAMPLE_STRING_DEFAULT_TOP_PADDING,
        constants::ALPHABET_SAMPLE_STRING_DEFAULT_BOTTOM_PADDING,
        constants::ALPHABET_SAMPLE_STRING_DEFAULT_PADDING_SYMBOL,
    );
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
