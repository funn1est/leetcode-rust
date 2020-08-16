/// https://leetcode.com/problems/product-of-array-except-self/
///
/// https://leetcode-cn.com/problems/product-of-array-except-self/
pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];
        for i in 1..n {
            res[i] = nums[i - 1] * res[i - 1];
        }
        let mut acc = 1;
        for i in (0..n).rev() {
            res[i] *= acc;
            acc *= nums[i];
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}
