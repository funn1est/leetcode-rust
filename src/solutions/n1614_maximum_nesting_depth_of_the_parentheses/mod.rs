/// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
///
/// https://leetcode-cn.com/problems/maximum-nesting-depth-of-the-parentheses/
pub struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut count = 0;
        let mut max = 0;
        s.chars().for_each(|c| match c {
            '(' => {
                count += 1;
                if count > max {
                    max = count;
                }
            }
            ')' => {
                count -= 1;
            }
            _ => (),
        });
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_owned()), 3);
        assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_owned()), 3);
        assert_eq!(Solution::max_depth("1+(2*3)/(2-1)".to_owned()), 1);
        assert_eq!(Solution::max_depth("1".to_owned()), 0);
    }
}
