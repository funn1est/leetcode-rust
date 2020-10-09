/// https://leetcode.com/problems/decompress-run-length-encoded-list/
///
/// https://leetcode-cn.com/problems/decompress-run-length-encoded-list/
pub struct Solution {}

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        nums.chunks(2)
            .flat_map(|pair| std::iter::repeat(pair[1]).take(pair[0] as usize))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_rl_elist() {
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
            vec![2, 4, 4, 4]
        );
        assert_eq!(
            Solution::decompress_rl_elist(vec![1, 1, 2, 3]),
            vec![1, 3, 3]
        );
    }
}
