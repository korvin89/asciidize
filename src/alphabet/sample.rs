use crate::alphabet::Alphabet;

#[derive(Debug, PartialEq)]
pub struct Sample {
    pub chars: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Sample {
    #[must_use]
    pub fn from_alphabet(alphabet: &Alphabet, width: usize, filler_symbol: char) -> Sample {
        let mut chars = vec![];

        for chunk in alphabet.symbols.chunks(width) {
            let mut row = vec![];
            for symbol in chunk {
                row.push(*symbol);
            }
            for _ in 0..(width - chunk.len()) {
                row.push(filler_symbol);
            }
            chars.push(row);
        }

        let height = chars.len();

        return Sample {
            chars,
            width,
            height,
        };
    }

    pub fn add_padding(&mut self, padding_symbol: char, padding_width: usize) {
        let mut chars = vec![];

        let result_width = self.width + 2 * padding_width;
        let result_height = self.height + 2 * padding_width;

        for _ in 0..padding_width {
            let mut row = vec![];
            for _ in 0..result_width {
                row.push(padding_symbol);
            }
            chars.push(row);
        }

        for row in &self.chars {
            let mut new_row = vec![];
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
            let mut row = vec![];
            for _ in 0..result_width {
                row.push(padding_symbol);
            }
            chars.push(row);
        }

        self.chars = chars;
        self.width = result_width;
        self.height = result_height;
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
    fn test_from_alphabet() {
        let alphabet = Alphabet::new(vec!['A', 'B', 'C', 'D', 'E']);
        let sample = Sample::from_alphabet(&alphabet, 2, ' ');
        assert_eq!(
            sample,
            Sample {
                chars: vec![vec!['A', 'B'], vec!['C', 'D'], vec!['E', ' ']],
                width: 2,
                height: 3,
            }
        );
    }

    #[test]
    fn test_add_padding() {
        let mut sample = Sample {
            chars: vec![vec!['A', 'B'], vec!['C', 'D']],
            width: 2,
            height: 2,
        };
        sample.add_padding(' ', 1);
        assert_eq!(
            sample,
            Sample {
                chars: vec![
                    vec![' ', ' ', ' ', ' '],
                    vec![' ', 'A', 'B', ' '],
                    vec![' ', 'C', 'D', ' '],
                    vec![' ', ' ', ' ', ' '],
                ],
                width: 4,
                height: 4,
            }
        );
    }

    #[test]
    fn test_display() {
        let sample = Sample {
            chars: vec![vec!['A', 'B'], vec!['C', 'D']],
            width: 2,
            height: 2,
        };
        let result = sample.to_string();
        assert_eq!(result, "AB\nCD");
    }
}
