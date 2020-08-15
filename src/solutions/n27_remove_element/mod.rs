/// https://leetcode.com/problems/remove-element/
/// https://leetcode-cn.com/problems/remove-element/

pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[index] = nums[i];
                index += 1;
            }
        }
        index as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums1 = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums1, 3), 2);
        assert_eq!(nums1, vec![2, 2, 2, 3]);

        let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums2, 2), 5);
        assert_eq!(nums2, vec![0, 1, 3, 0, 4, 0, 4, 2]);
    }
}
