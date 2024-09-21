use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct AlphabetMap {
    symbol_width: u32,
    symbol_height: u32,
    symbol_map: HashMap<char, Vec<Vec<u8>>>,
}

pub fn alphabet_map_to_string(alphabet_map: &AlphabetMap) -> String {
    serde_json::to_string(alphabet_map).unwrap()
}

pub fn string_to_alphabet_map(json_string: &str) -> Result<AlphabetMap, serde_json::Error> {
    serde_json::from_str(json_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_map_to_string() {
        let mut symbol_map = HashMap::new();
        symbol_map.insert('A', vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
        let alphabet_map = AlphabetMap {
            symbol_width: 3,
            symbol_height: 3,
            symbol_map: symbol_map,
        };
        let json_string = alphabet_map_to_string(&alphabet_map);
        assert_eq!(json_string, "{\"symbol_width\":3,\"symbol_height\":3,\"symbol_map\":{\"A\":[[1,1,1],[1,0,1],[1,1,1]]}}");
    }

    #[test]
    fn test_string_to_alphabet_map() {
        let json_string = "{\"symbol_width\":3,\"symbol_height\":3,\"symbol_map\":{\"A\":[[1,1,1],[1,0,1],[1,1,1]]}}";
        let alphabet_map = string_to_alphabet_map(json_string).unwrap();
        assert_eq!(alphabet_map.symbol_width, 3);
        assert_eq!(alphabet_map.symbol_height, 3);
        assert_eq!(
            alphabet_map.symbol_map.get(&'A').unwrap(),
            &vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]
        );
    }
}
