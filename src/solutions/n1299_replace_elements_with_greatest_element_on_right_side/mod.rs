/// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
///
/// https://leetcode-cn.com/problems/replace-elements-with-greatest-element-on-right-side/
pub struct Solution {}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        arr.iter().rev().fold(vec![], |mut all, &x| {
            all.insert(0, max);
            max = x.max(max);
            all
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_elements() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
    }
}
