/// https://leetcode.com/problems/compare-version-numbers/
///
/// https://leetcode-cn.com/problems/compare-version-numbers/
pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1 = version1.split('.').map(|str| str.parse::<i32>().unwrap());
        let mut version2 = version2.split('.').map(|str| str.parse::<i32>().unwrap());
        loop {
            match (version1.next(), version2.next()) {
                (None, None) => break 0,
                (n1, n2) => match n1.unwrap_or(0).cmp(&n2.unwrap_or(0)) {
                    Ordering::Less => break -1,
                    Ordering::Greater => break 1,
                    Ordering::Equal => (),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_version() {
        assert_eq!(
            Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
            1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.01".to_owned(), "1.001".to_owned()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_owned(), "1.0.0".to_owned()),
            0
        );
    }
}
