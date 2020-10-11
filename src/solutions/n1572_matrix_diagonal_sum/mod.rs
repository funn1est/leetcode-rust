/// https://leetcode.com/problems/matrix-diagonal-sum/
///
/// https://leetcode-cn.com/problems/matrix-diagonal-sum/
pub struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len = mat.len();
        let mut res = (0..len).fold(0, |mut acc, i| {
            acc += mat[i][i] + mat[i][len - 1 - i];
            acc
        });
        if len % 2 != 0 {
            let mid = len / 2;
            res -= mat[mid][mid];
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_sum() {
        assert_eq!(
            Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );
        assert_eq!(
            Solution::diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ]),
            8
        );
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }
}
