/// https://leetcode.com/problems/height-checker/
/// https://leetcode-cn.com/problems/height-checker/

pub struct Solution {}

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut clone = heights.clone();
        clone.sort();
        for (i, &height) in heights.iter().enumerate() {
            if clone[i] != height {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_height_checker() {
        assert_eq!(Solution::height_checker(vec![1, 1, 4, 2, 1, 3]), 3);
    }
}
