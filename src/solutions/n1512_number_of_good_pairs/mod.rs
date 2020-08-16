use std::collections::HashMap;

/// https://leetcode.com/problems/number-of-good-pairs/
///
/// https://leetcode-cn.com/problems/number-of-good-pairs/
pub struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        nums.iter().enumerate().for_each(|(i, &x)| {
            let indexes = map.entry(x).or_insert(vec![]);
            indexes.push(i as i32);
        });
        map.iter().fold(0, |all, (_, indexes)| {
            let len = indexes.len() as i32;
            if len < 2 {
                all
            } else {
                all + (len * (len - 1) / 2)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_identical_pairs() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
