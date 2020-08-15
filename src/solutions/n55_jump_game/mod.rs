/// https://leetcode.com/problems/jump-game/
/// https://leetcode-cn.com/problems/jump-game/
use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut max = 0;
        for i in 0..len {
            if i > max {
                return false;
            }
            max = cmp::max(max, i + nums[i] as usize);
            if max >= len - 1 {
                return true;
            }
        }
        true
    }

    pub fn can_jump1(nums: Vec<i32>) -> bool {
        let mut max = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i > max {
                return false;
            }
            max = cmp::max(max, i + num as usize);
            if max > nums.len() - 1 {
                return true;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump1(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump1(vec![3, 2, 1, 0, 4]), false);
    }
}
