/// https://leetcode.com/problems/length-of-last-word/
///
/// https://leetcode-cn.com/problems/length-of-last-word/
pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        for c in s.chars().rev() {
            if c == ' ' && count == 0 {
                continue;
            } else if c != ' ' {
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn length_of_last_word_1(s: String) -> i32 {
        s.chars()
            .rev()
            .skip_while(|&c| ' ' == c)
            .take_while(|&c| ' ' != c)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("ab".to_owned()), 2);
        assert_eq!(Solution::length_of_last_word("".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word("a ".to_owned()), 1);

        assert_eq!(Solution::length_of_last_word_1("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word_1("ab".to_owned()), 2);
        assert_eq!(Solution::length_of_last_word_1("".to_owned()), 0);
        assert_eq!(Solution::length_of_last_word_1("a ".to_owned()), 1);
    }
}
