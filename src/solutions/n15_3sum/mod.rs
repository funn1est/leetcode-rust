/// https://leetcode.com/problems/3sum/
///
/// https://leetcode-cn.com/problems/3sum/
pub struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res: Vec<Vec<i32>> = vec![];
        if nums.len() < 3 {
            return res;
        }
        nums.sort();
        for i in 0..(nums.len() - 2) {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let target = -nums[i];
            let mut right = nums.len() - 1;
            for j in i + 1..nums.len() {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                while j < right && nums[j] + nums[right] > target {
                    right -= 1;
                }
                if j == right {
                    break;
                }
                if nums[j] + nums[right] == target {
                    res.push(vec![nums[i], nums[j], nums[right]]);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }
}
