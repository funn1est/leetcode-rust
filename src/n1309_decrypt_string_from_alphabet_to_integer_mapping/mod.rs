// https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
// https://leetcode-cn.com/problems/decrypt-string-from-alphabet-to-integer-mapping/

pub struct Solution;

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut i = 0;
        let len = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut res = "".to_string();
        while i < len {
            let is_after_i = i + 2 < len && s[i + 2] == '#';
            res.push(
                (if is_after_i {
                    (s[i] as u8 - b'0') * 10 + (s[i + 1] as u8 - b'0')
                } else {
                    s[i] as u8 - b'0'
                } - 1
                    + b'a') as char,
            );
            i += if is_after_i { 3 } else { 1 };
        }
        res
    }

    pub fn freq_alphabets_1(s: String) -> String {
        let mut s = s.chars().rev();
        let mut res = "".to_string();
        while let Some(char) = s.next() {
            let num = if char == '#' {
                s.next().unwrap() as u8 - b'0' + (s.next().unwrap() as u8 - b'0') * 10
            } else {
                char as u8 - b'0'
            } - 1
                + b'a';
            res.insert(0, num as char);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_freq_alphabets() {
        assert_eq!(
            Solution::freq_alphabets("10#11#12".to_string()),
            "jkab".to_string()
        );
        assert_eq!(
            Solution::freq_alphabets("1326#".to_string()),
            "acz".to_string()
        );
        assert_eq!(Solution::freq_alphabets("25#".to_string()), "y".to_string());
        assert_eq!(
            Solution::freq_alphabets(
                "12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()
            ),
            "abcdefghijklmnopqrstuvwxyz".to_string()
        );
    }
}
