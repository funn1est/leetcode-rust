/// https://leetcode.com/problems/longest-palindromic-substring/
///
/// https://leetcode-cn.com/problems/longest-palindromic-substring/
pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![true; len]; len];
        let mut start = 0;
        let mut end = 0;
        for j in 0..len {
            for i in 0..=j {
                if s[i] != s[j] {
                    dp[i][j] = false;
                } else {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }

                if dp[i][j] && j - i + 1 > end - start {
                    start = i;
                    end = j + 1;
                }
            }
        }
        s[start..end].iter().collect()
    }

    fn expand_center(s: &[char], left: usize, mut right: usize) -> isize {
        let mut left = left as isize;
        while left >= 0 && right < s.len() && s[left as usize] == s[right] {
            left -= 1;
            right += 1;
        }
        right as isize - left as isize - 1
    }

    pub fn longest_palindrome_center_expand(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        for i in 0..len {
            let len_odd = Solution::expand_center(&s, i, i);
            let len_even = Solution::expand_center(&s, i, i + 1);
            let max_len = len_odd.max(len_even).max(0) as usize;
            if max_len > end - start {
                start = i - (max_len - 1) / 2;
                end = i + max_len / 2;
            }
        }
        s[start..=end].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_owned()),
            "bab".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_owned()),
            "bb".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome("ac".to_owned()),
            "a".to_owned()
        );

        assert_eq!(
            Solution::longest_palindrome_center_expand("babad".to_owned()),
            "aba".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome_center_expand("cbbd".to_owned()),
            "bb".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome_center_expand("ac".to_owned()),
            "c".to_owned()
        );
        assert_eq!(
            Solution::longest_palindrome_center_expand("bb".to_owned()),
            "bb".to_owned()
        );
    }
}
