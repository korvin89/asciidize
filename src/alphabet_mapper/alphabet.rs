pub struct Alphabet {
    simbols: Vec<char>,
}

const DEFAULT_ALPHABET_STR: &str = "
 !\"#$%&'()*+,-./0123456789:;<=>?[\\]^_`{|}~@
ABCDEFGHIJKLMNOPQRSTUVWXYZ
abcdefghijklmnopqrstuvwxyz";

impl Alphabet {
    #[must_use]
    pub fn new(simbols: Vec<char>) -> Alphabet {
        return Alphabet { simbols };
    }

    #[must_use]
    pub fn from_symbol_str(s: &str) -> Alphabet {
        return Alphabet::new(s.chars().collect());
    }

    #[must_use]
    pub fn from_default() -> Alphabet {
        return Alphabet::from_symbol_str(DEFAULT_ALPHABET_STR);
    }

    #[must_use]
    pub fn to_sample_string(
        &self,
        width: usize,
        left_padding: usize,
        right_padding: usize,
        top_padding: usize,
        bottom_padding: usize,
        padding_symbol: char,
    ) -> String {
        let mut result = String::new();

        for _ in 0..top_padding {
            for _ in 0..(width + left_padding + right_padding) {
                result.push(padding_symbol);
            }
            result.push('\n');
        }

        for row in self.simbols.chunks(width) {
            for _ in 0..left_padding {
                result.push(padding_symbol);
            }
            for symbol in row {
                result.push(*symbol);
            }
            for _ in 0..(width - row.len() + right_padding) {
                result.push(padding_symbol);
            }
            result.push('\n');
        }

        for _ in 0..bottom_padding {
            for _ in 0..(width + left_padding + right_padding) {
                result.push(padding_symbol);
            }
            result.push('\n');
        }
        result.pop();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let alphabet = Alphabet::new(vec!['A', 'B', 'C']);
        assert_eq!(alphabet.simbols.len(), 3);
    }

    #[test]
    fn test_from_symbol_str() {
        let alphabet = Alphabet::from_symbol_str("ABC");
        assert_eq!(alphabet.simbols.len(), 3);
    }

    #[test]
    fn test_from_default() {
        let alphabet = Alphabet::from_default();
        assert_eq!(alphabet.simbols.len(), 98);
    }

    #[test]
    fn test_to_sample_string() {
        let alphabet = Alphabet::from_symbol_str("ABCDEFGHIJ");
        let sample_string = alphabet.to_sample_string(3, 1, 1, 1, 1, ' ');
        assert_eq!(sample_string, "     \n ABC \n DEF \n GHI \n J   \n     ");
    }
}
