/// https://leetcode.com/problems/split-a-string-in-balanced-strings/
/// https://leetcode-cn.com/problems/split-a-string-in-balanced-strings/

pub struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut left_count = 0;
        s.chars().fold(0, |all, x| {
            left_count += if x == 'L' { 1 } else { -1 };
            if left_count == 0 {
                return all + 1;
            }
            all
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_balanced_string_split() {
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_string()), 4);
        assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_string()), 3);
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_string()), 1);
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_string()), 2);
    }
}
