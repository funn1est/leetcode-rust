// https://leetcode.com/problems/house-robber/
// https://leetcode-cn.com/problems/house-robber/

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        match len {
            0 => 0,
            1 => nums[0],
            _ => {
                let mut dp = vec![0; len];
                dp[0] = nums[0];
                dp[1] = max(nums[0], nums[1]);
                for i in 2..len {
                    dp[i] = max(dp[i - 1], dp[i - 2] + nums[i]);
                }
                dp[len - 1]
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![]), 0);
        assert_eq!(Solution::rob(vec![1]), 1);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
