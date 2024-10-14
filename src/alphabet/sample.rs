use crate::alphabet::Alphabet;

pub struct Sample {
    chars: Vec<Vec<char>>,
}

impl Sample {
    #[must_use]
    fn new(chars: Vec<Vec<char>>) -> Sample {
        return Sample { chars };
    }

    #[must_use]
    pub fn from_alphabet(alphabet: &Alphabet, width: usize, filler_symbol: char) -> Sample {
        let mut chars = Vec::new();

        for chunk in alphabet.symbols.chunks(width) {
            let mut row = Vec::new();
            for symbol in chunk {
                row.push(*symbol);
            }
            for _ in 0..(width - chunk.len()) {
                row.push(filler_symbol);
            }
            chars.push(row);
        }

        return Sample::new(chars);
    }

    pub fn add_padding(&mut self, padding_symbol: char, padding_width: usize) {
        let mut chars = Vec::new();

        let original_width = self.chars[0].len();
        let result_width = original_width + 2 * padding_width;

        for _ in 0..padding_width {
            let mut row = Vec::new();
            for _ in 0..result_width {
                row.push(padding_symbol);
            }
            chars.push(row);
        }

        for row in &self.chars {
            let mut new_row = Vec::new();
            for _ in 0..padding_width {
                new_row.push(padding_symbol);
            }
            for symbol in row {
                new_row.push(*symbol);
            }
            for _ in 0..padding_width {
                new_row.push(padding_symbol);
            }
            chars.push(new_row);
        }

        for _ in 0..padding_width {
            let mut row = Vec::new();
            for _ in 0..result_width {
                row.push(padding_symbol);
            }
            chars.push(row);
        }

        self.chars = chars;
    }
}

impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut result = String::new();
        for row in &self.chars {
            for symbol in row {
                result.push(*symbol);
            }
            result.push('\n');
        }
        if !result.is_empty() {
            result.pop();
        }
        return write!(f, "{result}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sample = Sample::new(vec![vec!['A', 'B'], vec!['C', 'D']]);
        assert_eq!(sample.chars, vec![vec!['A', 'B'], vec!['C', 'D']]);
    }

    #[test]
    fn test_from_alphabet() {
        let alphabet = Alphabet::new(vec!['A', 'B', 'C', 'D', 'E']);
        let sample = Sample::from_alphabet(&alphabet, 2, ' ');
        assert_eq!(
            sample.chars,
            vec![vec!['A', 'B'], vec!['C', 'D'], vec!['E', ' ']]
        );
    }

    #[test]
    fn test_add_padding() {
        let mut sample = Sample::new(vec![vec!['A', 'B'], vec!['C', 'D']]);
        sample.add_padding(' ', 1);
        assert_eq!(
            sample.chars,
            vec![
                vec![' ', ' ', ' ', ' '],
                vec![' ', 'A', 'B', ' '],
                vec![' ', 'C', 'D', ' '],
                vec![' ', ' ', ' ', ' ']
            ]
        );
    }

    #[test]
    fn test_display() {
        let sample = Sample::new(vec![vec!['A', 'B'], vec!['C', 'D']]);
        let result = sample.to_string();
        assert_eq!(result, "AB\nCD");
    }
}
