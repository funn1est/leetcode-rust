/// https://leetcode.com/problems/move-zeroes/
///
/// https://leetcode-cn.com/problems/move-zeroes/
pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums.swap(slow, i);
                slow += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut xs = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut xs);
        assert_eq!(xs, vec![1, 3, 12, 0, 0]);
    }
}
