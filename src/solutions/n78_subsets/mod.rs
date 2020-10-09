/// https://leetcode.com/problems/subsets/
///
/// https://leetcode-cn.com/problems/subsets/
pub struct Solution {}

impl Solution {
    fn dfs(nums: &Vec<i32>, acc: &mut Vec<Vec<i32>>, index: usize, cur: &mut Vec<i32>) {
        acc.push(cur.clone());
        for i in index..nums.len() {
            cur.push(nums[i]);
            Self::dfs(&nums, acc, i + 1, cur);
            cur.pop();
        }
    }

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        Self::dfs(&nums, &mut res, 0, &mut vec![]);
        res
    }

    fn dfs_1(nums: &Vec<i32>, acc: &mut Vec<Vec<i32>>, index: usize, cur: &mut Vec<i32>) {
        if index == nums.len() {
            acc.push(cur.clone());
            return;
        }
        cur.push(nums[index]);
        Self::dfs_1(&nums, acc, index + 1, cur);
        cur.pop();
        Self::dfs_1(&nums, acc, index + 1, cur);
    }

    pub fn subsets_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        Self::dfs_1(&nums, &mut res, 0, &mut vec![]);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ]
        );

        assert_eq!(
            Solution::subsets_1(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 2],
                vec![1, 3],
                vec![1],
                vec![2, 3],
                vec![2],
                vec![3],
                vec![],
            ]
        );
    }
}
