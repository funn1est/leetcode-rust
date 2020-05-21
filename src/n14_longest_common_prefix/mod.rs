// https://leetcode.com/problems/longest-common-prefix/
// https://leetcode-cn.com/problems/longest-common-prefix/

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        match strs.len() {
            0 => "".to_string(),
            1 => strs[0].to_string(),
            _ => {
                strs.sort_by(|a, b| a.len().cmp(&b.len()));
                let mut strs = strs.iter();
                let head = strs.next().unwrap();
                strs.fold(head.to_string(), |all, cur| {
                    all.chars()
                        .zip(cur.chars())
                        .take_while(|(left, right)| left == right)
                        .map(|(char, _)| char)
                        .collect()
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_number_of_steps() {
        assert_eq!(
            Solution::longest_common_prefix(vec_of_string!["flower", "flow", "flight"]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_of_string!["dog", "racecar", "car"]),
            "".to_string()
        );
    }
}
