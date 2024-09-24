pub struct Alphabet {
    simbols: Vec<char>,
}

#[must_use]
pub fn to_sample_string(
    alphabet: &Alphabet,
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

    for row in alphabet.simbols.chunks(width) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sample_string() {
        let alphabet = Alphabet {
            simbols: vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'],
        };
        let sample_string = to_sample_string(&alphabet, 3, 1, 1, 1, 1, ' ');
        assert_eq!(sample_string, "     \n ABC \n DEF \n GHI \n J   \n     ");
    }
}
