/// https://leetcode.com/problems/remove-outermost-parentheses/
///
/// https://leetcode-cn.com/problems/remove-outermost-parentheses/
pub struct Solution {}

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut string = String::new();
        let mut count = 0;
        for char in s.chars() {
            if char == '(' {
                if count > 0 {
                    string.push(char);
                }
                count += 1;
            } else {
                if count > 1 {
                    string.push(char);
                }
                count -= 1;
            }
        }
        string
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_outer_parentheses() {
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("()()".to_string()),
            "".to_string()
        );
    }
}
