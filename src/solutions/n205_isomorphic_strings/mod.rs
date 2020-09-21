/// https://leetcode.com/problems/isomorphic-strings/
///
/// https://leetcode-cn.com/problems/isomorphic-strings/
pub struct Solution {}

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let mut s_buf = vec![0; 256];
        let mut t_buf = vec![0; 256];
        for i in 0..s.len() {
            if i != 0 && s_buf[s[i] as usize] != t_buf[t[i] as usize] {
                return false;
            }
            s_buf[s[i] as usize] = i + 1;
            t_buf[t[i] as usize] = i + 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_owned(), "add".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
    }
}
