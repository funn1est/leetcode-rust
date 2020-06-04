// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/
// https://leetcode-cn.com/problems/minimum-value-to-get-positive-step-by-step-sum/

pub struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        nums.iter().fold(0, |mut start, &num| {
            diff += num;
            if diff < 0 {
                start -= diff;
                diff = 0;
            }
            start
        }) + 1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_min_start_value() {
        assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
        assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
        assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    }
}