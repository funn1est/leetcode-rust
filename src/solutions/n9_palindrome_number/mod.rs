/// https://leetcode.com/problems/palindrome-number/
/// https://leetcode-cn.com/problems/palindrome-number/

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        #[allow(unused_doc_comments)]
        /// https://stackoverflow.com/a/24542608
        if x < 0 {
            return false;
        }
        let string = x.to_string();
        let half = string.len() / 2;
        string
            .bytes()
            .take(half)
            .eq(string.bytes().rev().take(half))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
