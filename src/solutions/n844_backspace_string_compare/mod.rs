/// https://leetcode.com/problems/backspace-string-compare/
///
/// https://leetcode-cn.com/problems/backspace-string-compare/
pub struct Solution {}

impl Solution {
    fn get_str(s: String) -> String {
        s.chars().fold("".to_owned(), |mut acc, c| match c {
            '#' => {
                if !acc.is_empty() {
                    acc.pop();
                }
                acc
            }
            _ => {
                acc.push(c);
                acc
            }
        })
    }
    pub fn backspace_compare(s: String, t: String) -> bool {
        Self::get_str(s) == Self::get_str(t)
    }

    fn eat_char<T: Iterator<Item = char>>(chars: &mut T) -> Option<char> {
        let mut skip = 0;
        loop {
            let char = chars.next();
            match char {
                None => break None,
                Some('#') => {
                    skip += 1;
                }
                Some(x) => {
                    if skip == 0 {
                        break Some(x);
                    }
                    skip -= 1;
                }
            }
        }
    }

    pub fn backspace_compare_1(s: String, t: String) -> bool {
        let mut s = s.chars().rev();
        let mut t = t.chars().rev();
        loop {
            let s_char = Self::eat_char(&mut s);
            let t_char = Self::eat_char(&mut t);
            if s_char == None && t_char == None {
                break true;
            }
            if s_char != t_char {
                break false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_compare() {
        assert_eq!(
            Solution::backspace_compare("ab#c".to_owned(), "ad#c".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("ab##".to_owned(), "c#d#".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a##c".to_owned(), "#a#c".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("xywrrmp".to_owned(), "xywrrmu#p".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare("a#c".to_owned(), "b".to_owned()),
            false
        );

        assert_eq!(
            Solution::backspace_compare_1("ab#c".to_owned(), "ad#c".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare_1("ab##".to_owned(), "c#d#".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare_1("a##c".to_owned(), "#a#c".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare_1("xywrrmp".to_owned(), "xywrrmu#p".to_owned()),
            true
        );
        assert_eq!(
            Solution::backspace_compare_1("a#c".to_owned(), "b".to_owned()),
            false
        );
    }
}
