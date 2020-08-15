/// https://leetcode.com/problems/keyboard-row/
/// https://leetcode-cn.com/problems/keyboard-row/
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let map: HashMap<char, i32> = [
            ('q', 1),
            ('w', 1),
            ('e', 1),
            ('r', 1),
            ('t', 1),
            ('y', 1),
            ('u', 1),
            ('i', 1),
            ('o', 1),
            ('p', 1),
            ('a', 2),
            ('s', 2),
            ('d', 2),
            ('f', 2),
            ('g', 2),
            ('h', 2),
            ('j', 2),
            ('k', 2),
            ('l', 2),
            ('z', 3),
            ('x', 3),
            ('c', 3),
            ('v', 3),
            ('b', 3),
            ('n', 3),
            ('m', 3),
        ]
        .iter()
        .cloned()
        .collect();
        let mut result = vec![];

        for word in words {
            let mut chars = word.chars();
            let first = chars.next().unwrap().to_lowercase().next().unwrap();
            let row = map.get(&first).unwrap();
            if chars.all(|c| map.get(&c.to_lowercase().next().unwrap()).unwrap() == row) {
                result.push(word);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(vec_of_string!["Hello", "Alaska", "Dad", "Peace"]),
            vec_of_string!["Alaska", "Dad"]
        );
    }
}
