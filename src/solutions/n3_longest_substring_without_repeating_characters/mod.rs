/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
/// https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
use std::cmp;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let mut max = 0;
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut start = 0;

        for (end, key) in s.chars().enumerate() {
            if let Some(_) = map.get(&key) {
                start = cmp::max(start, map.get(&key).unwrap() + 1);
            }
            map.insert(key, end as i32);
            max = cmp::max(max, end as i32 - start + 1);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);
    }
}
