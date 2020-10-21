/// https://leetcode.com/problems/long-pressed-name/
///
/// https://leetcode-cn.com/problems/long-pressed-name/
pub struct Solution {}

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let name = name.chars().collect::<Vec<char>>();
        let typed = typed.chars().collect::<Vec<char>>();
        while j < typed.len() {
            if i < name.len() && name[i] == typed[j] {
                i += 1;
                j += 1;
            } else if j > 0 && typed[j] == typed[j - 1] {
                j += 1
            } else {
                return false;
            }
        }
        i == name.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_long_pressed_name() {
        assert_eq!(
            Solution::is_long_pressed_name("alex".to_owned(), "aaleex".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name("saeed".to_owned(), "ssaaedd".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_long_pressed_name("leelee".to_owned(), "lleeelee".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_long_pressed_name("laiden".to_owned(), "laiden".to_owned()),
            true
        );
    }
}
