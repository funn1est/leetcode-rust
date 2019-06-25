// https://leetcode.com/problems/pascals-triangle/
// https://leetcode-cn.com/problems/pascals-triangle/

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..num_rows as usize {
            let mut row = vec![1; i + 1];
            for j in 1..i {
                row[j] = res[i - 1][j - 1] + res[i - 1][j];
            }
            res.push(row);
        }
        res
    }
    pub fn generate1(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..num_rows as usize {
            let mut row = vec![];
            for j in 0..i + 1 {
                if j == 0 || j == i {
                    row.push(1);
                    continue;
                }
                row.push(res[i - 1][j - 1] + res[i - 1][j]);
            }
            res.push(row);
        }
        res
    }
    pub fn generate2(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..num_rows as usize {
            let mut row = vec![1; i + 1];
            let mid = ((i as f32 + 1.0) / 2.0).ceil() as usize;
            for j in 1..mid {
                let sum = res[i - 1][j - 1] + res[i - 1][j];
                row[j] = sum;
                row[i - j] = sum;
            }
            res.push(row);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_generate() {
        let res = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate(5), res);
        assert_eq!(Solution::generate1(5), res);
        assert_eq!(Solution::generate2(5), res);
    }
}
