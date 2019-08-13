// https://leetcode.com/problems/sort-colors/
// https://leetcode-cn.com/problems/sort-colors/

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }

        let (mut p0, mut p2) = (0, nums.len() - 1);
        let mut current = 0;

        while current <= p2 && p2 > 0 {
            if nums[current] == 0 {
                nums.swap(current, p0);
                current += 1;
                p0 += 1;
            } else if nums[current] == 2 {
                nums.swap(current, p2);
                p2 -= 1;
            } else {
                current += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_sort_colors() {
        let mut nums1 = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums1);
        assert_eq!(nums1, vec![0, 0, 1, 1, 2, 2]);

        let mut nums2 = vec![2, 0, 1];
        Solution::sort_colors(&mut nums2);
        assert_eq!(nums2, vec![0, 1, 2]);
    }
}
