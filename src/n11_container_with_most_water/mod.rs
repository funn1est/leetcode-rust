// https://leetcode.com/problems/container-with-most-water/
// https://leetcode-cn.com/problems/container-with-most-water/

use std::cmp;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut start, mut end) = (0, height.len() - 1);
        while start < end {
            max = cmp::max(
                max,
                cmp::min(height[start], height[end]) as usize * (end - start),
            );
            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
