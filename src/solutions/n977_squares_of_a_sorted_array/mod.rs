/// https://leetcode.com/problems/squares-of-a-sorted-array/
///
/// https://leetcode-cn.com/problems/squares-of-a-sorted-array/
pub struct Solution {}

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; a.len()];
        let mut i = 0;
        let mut j = a.len() - 1;
        let mut idx = j;
        while i <= j {
            if a[i].abs() < a[j].abs() {
                res[idx] = a[j] * a[j];
                j -= 1;
            } else {
                res[idx] = a[i] * a[i];
                i += 1;
            }
            if idx != 0 {
                idx -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
