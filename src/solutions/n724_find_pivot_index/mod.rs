/// https://leetcode.com/problems/find-pivot-index/
/// https://leetcode-cn.com/problems/find-pivot-index/

pub struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().fold(0, |acc, &x| acc + x);
        let mut left = 0;
        for i in 0..nums.len() {
            let right = sum - nums[i] - left;
            if left == right {
                return i as i32;
            }
            left += nums[i];
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pivot_index() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }
}
