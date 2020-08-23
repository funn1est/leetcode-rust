/// https://leetcode.com/problems/reverse-words-in-a-string-iii/
///
/// https://leetcode-cn.com/problems/reverse-words-in-a-string-iii/
pub struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace()
            .map(|str| str.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn reverse_words_1(s: String) -> String {
        let mut tmp = "".to_owned();
        s.chars()
            .enumerate()
            .fold(String::from(""), |mut all, (ind, c)| {
                if c == ' ' {
                    all.push_str(&(format!("{} ", &tmp)));
                    tmp = "".to_owned();
                } else {
                    tmp = c.to_string() + &tmp;
                    if ind == s.len() - 1 {
                        all.push_str(&tmp);
                    }
                }
                all
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_owned()),
            "s'teL ekat edoCteeL tsetnoc".to_owned()
        );
        assert_eq!(
            Solution::reverse_words_1("Let's take LeetCode contest".to_owned()),
            "s'teL ekat edoCteeL tsetnoc".to_owned()
        );
    }
}
