// Alphabet

pub const ALPHABET_SYMBOL_STR: &str = "\
 !\"#$%&'()*+,-./0123456789:;<=>?@\
ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`\
abcdefghijklmnopqrstuvwxyz{|}~"; // ASCII symbols from 32 to 126

// AlphabetSampleString
pub const ALPHABET_SAMPLE_WIDTH: usize = 10; // sqrt(ALPHABET_SYMBOL_STR.len())
pub const ALPHABET_SAMPLE_FILLER_SYMBOL: char = ' ';
pub const ALPHABET_SAMPLE_BORDER_SYMBOL: char = 'X';
pub const ALPHABET_SAMPLE_BORDER_WIDTH: usize = 1;
pub const ALPHABET_SAMPLE_PADDING_SYMBOL: char = ' ';
pub const ALPHABET_SAMPLE_PADDING_WIDTH: usize = 3;
