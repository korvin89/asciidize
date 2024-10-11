use core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
    symbol_width: u32,
    symbol_height: u32,
    symbol_map: HashMap<char, Vec<Vec<u8>>>,
}

#[derive(Debug, Clone)]
pub struct InvalidDataError {
    message: String,
}

impl fmt::Display for InvalidDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Invalid data: {}", self.message);
    }
}

impl FromStr for Map {
    type Err = InvalidDataError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return serde_json::from_str(s).map_err(|e| {
            return InvalidDataError {
                message: e.to_string(),
            };
        });
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", serde_json::to_string(self).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let mut symbol_map = HashMap::new();
        symbol_map.insert('A', vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
        let alphabet_map = Map {
            symbol_width: 3,
            symbol_height: 3,
            symbol_map: symbol_map,
        };
        let json_string = alphabet_map.to_string();
        assert_eq!(json_string, "{\"symbol_width\":3,\"symbol_height\":3,\"symbol_map\":{\"A\":[[1,1,1],[1,0,1],[1,1,1]]}}");
    }

    #[test]
    fn test_from_str() {
        let json_string = "{\"symbol_width\":3,\"symbol_height\":3,\"symbol_map\":{\"A\":[[1,1,1],[1,0,1],[1,1,1]]}}";
        let alphabet_map = Map::from_str(json_string).unwrap();
        assert_eq!(alphabet_map.symbol_width, 3);
        assert_eq!(alphabet_map.symbol_height, 3);
        assert_eq!(
            alphabet_map.symbol_map.get(&'A').unwrap(),
            &vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]
        );
    }

    #[test]
    fn test_from_str_invalid_data() {
        let json_string = "{";
        let alphabet_map = Map::from_str(json_string);
        assert_eq!(alphabet_map.is_err(), true);
        assert_eq!(
            alphabet_map.err().unwrap().to_string(),
            "Invalid data: EOF while parsing an object at line 1 column 1"
        );
    }
}
