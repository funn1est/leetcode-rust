use std::cmp;

/// https://leetcode.com/problems/maximum-subarray/
///
/// https://leetcode-cn.com/problems/maximum-subarray/
pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0; len];
        let mut max = nums[0];
        dp[0] = nums[0];
        for i in 1..len {
            let val = if dp[i - 1] > 0 { dp[i - 1] } else { 0 };
            dp[i] = nums[i] + val;
            max = cmp::max(max, dp[i]);
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
