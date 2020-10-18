use std::collections::HashMap;

/// https://leetcode.com/problems/single-number/
///
/// https://leetcode-cn.com/problems/single-number/
pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |all, &x| all ^ x)
    }

    pub fn single_number_1(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|&x| {
            if let None = map.remove(&x) {
                map.insert(x, 1);
            }
        });
        *map.keys().next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);

        assert_eq!(Solution::single_number_1(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number_1(vec![4, 1, 2, 1, 2]), 4);
    }
}
