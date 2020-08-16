/// https://leetcode.com/problems/unique-paths/
///
/// https://leetcode-cn.com/problems/unique-paths/
pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // dp
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![1; n]; m];
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m - 1][n - 1]
    }
    pub fn unique_paths1(m: i32, n: i32) -> i32 {
        // recursion
        if m == 1 || n == 1 {
            return 1;
        }
        Solution::unique_paths1(m - 1, n) + Solution::unique_paths1(m, n - 1)
    }
    pub fn unique_paths2(m: i32, n: i32) -> i32 {
        // recursion + memorization
        let m = m as usize;
        let n = n as usize;
        fn helper(m: usize, n: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if m == 0 || n == 0 {
                return 1;
            } else if memo[m][n] == -1 {
                memo[m][n] = helper(m - 1, n, memo) + helper(m, n - 1, memo);
            }
            memo[m][n]
        }
        helper(m - 1, n - 1, &mut vec![vec![-1; n]; m])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths1(3, 2), 3);
        assert_eq!(Solution::unique_paths1(7, 3), 28);
        assert_eq!(Solution::unique_paths2(3, 2), 3);
        assert_eq!(Solution::unique_paths2(7, 3), 28);
    }
}
