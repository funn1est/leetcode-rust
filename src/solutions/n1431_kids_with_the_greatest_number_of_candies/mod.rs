use std::option::Option::Some;

/// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
///
/// https://leetcode-cn.com/problems/kids-with-the-greatest-number-of-candies/
pub struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        if let Some(&max) = candies.iter().max() {
            candies
                .iter()
                .map(|&i| {
                    if i + extra_candies >= max {
                        true
                    } else {
                        false
                    }
                })
                .collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kids_with_candies() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
