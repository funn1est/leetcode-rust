// https://leetcode.com/problems/smallest-range-i/
// https://leetcode-cn.com/problems/smallest-range-i/

pub struct Solution;

impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let (mut a_min, mut a_max) = (i32::max_value(), i32::min_value());
        for x in a {
            a_min = a_min.min(x);
            a_max = a_max.max(x);
        }
        ((a_max - k) - (a_min + k)).max(0)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_smallest_range_i() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
