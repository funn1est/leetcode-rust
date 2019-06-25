// https://leetcode.com/problems/reverse-string/
// https://leetcode-cn.com/problems/reverse-string/

pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len();
        for i in 0..len / 2 {
            // let tmp = s[i];
            // s[i] = s[len - i - 1];
            // s[len - i - 1] = tmp;
            s.swap(i, len - i - 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_reverse_string() {
        let mut s1 = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s1);
        assert_eq!(s1, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
