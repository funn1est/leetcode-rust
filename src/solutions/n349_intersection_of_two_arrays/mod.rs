use std::collections::HashSet;

/// https://leetcode.com/problems/intersection-of-two-arrays/
///
/// https://leetcode-cn.com/problems/intersection-of-two-arrays/
pub struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = HashSet::new();
        let mut set = HashSet::new();
        nums1.iter().for_each(|&x| {
            set.insert(x);
        });
        nums2.iter().for_each(|&x| {
            if set.contains(&x) {
                res.insert(x);
            }
        });
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_hash_set(vec: Vec<i32>) -> HashSet<i32> {
        vec.into_iter().fold(HashSet::new(), |mut set, current| {
            set.insert(current);
            set
        })
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            to_hash_set(Solution::intersection(vec![1, 2, 2, 1], vec![2, 2])),
            to_hash_set(vec![2])
        );
        assert_eq!(
            to_hash_set(Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4])),
            to_hash_set(vec![4, 9])
        );
    }
}
