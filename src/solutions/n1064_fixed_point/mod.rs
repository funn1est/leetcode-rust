/// https://leetcode.com/problems/fixed-point/
///
/// https://leetcode-cn.com/problems/fixed-point/
pub struct Solution {}

impl Solution {
    pub fn fixed_point(a: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = a.len() - 1;
        while start <= end {
            let mid = (start + end) / 2;
            if a[mid] == mid as i32 {
                return mid as i32;
            } else if a[mid] < mid as i32 {
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fixed_point() {
        assert_eq!(Solution::fixed_point(vec![-10, -5, 0, 3, 7]), 3);
        assert_eq!(Solution::fixed_point(vec![0, 2, 5, 8, 17]), 0);
        assert_eq!(Solution::fixed_point(vec![-10, -5, 3, 4, 7, 9]), -1);
    }
}
