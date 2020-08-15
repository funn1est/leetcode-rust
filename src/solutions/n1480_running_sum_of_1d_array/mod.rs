/// https://leetcode.com/problems/running-sum-of-1d-array/
/// https://leetcode-cn.com/problems/running-sum-of-1d-array/

pub struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.iter().fold(vec![], |mut all, &x| {
            sum += x;
            all.push(sum);
            all
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }
}
