/// https://leetcode.com/problems/xor-operation-in-an-array/
///
/// https://leetcode-cn.com/problems/xor-operation-in-an-array/
pub struct Solution {}

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (start..)
            .step_by(2)
            .take(n as usize)
            .fold(0, |all, x| all ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_operation() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
        assert_eq!(Solution::xor_operation(4, 3), 8);
        assert_eq!(Solution::xor_operation(1, 7), 7);
        assert_eq!(Solution::xor_operation(10, 5), 2);
    }
}
