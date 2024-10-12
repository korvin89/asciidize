pub struct Alphabet {
    pub symbols: Vec<char>,
}

impl Alphabet {
    #[must_use]
    pub fn new(symbols: Vec<char>) -> Alphabet {
        return Alphabet { symbols };
    }

    #[must_use]
    pub fn from_symbol_str(s: &str) -> Alphabet {
        return Alphabet::new(s.chars().collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let alphabet = Alphabet::new(vec!['A', 'B', 'C']);
        assert_eq!(alphabet.symbols.len(), 3);
    }

    #[test]
    fn test_from_symbol_str() {
        let alphabet = Alphabet::from_symbol_str("ABC");
        assert_eq!(alphabet.symbols.len(), 3);
    }
}
