/// https://leetcode.com/problems/detect-capital/
/// https://leetcode-cn.com/problems/detect-capital/

pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        let first_char = chars.nth(0).unwrap();
        let is_all_low = first_char.is_lowercase();
        let mut upper_count = 0;
        for (index, char) in chars.enumerate() {
            if char.is_uppercase() {
                if is_all_low {
                    return false;
                }
                upper_count += 1;
            }
            if upper_count != 0 && upper_count != index + 1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_detect_capital_use() {
        assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
        assert_eq!(Solution::detect_capital_use("leetcode".to_string()), true);
        assert_eq!(Solution::detect_capital_use("Google".to_string()), true);
        assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
        assert_eq!(Solution::detect_capital_use("flaG".to_string()), false);
    }
}
