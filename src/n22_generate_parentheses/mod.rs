// https://leetcode.com/problems/generate-parentheses/
// https://leetcode-cn.com/problems/generate-parentheses/

pub struct Solution;

impl Solution {
    fn is_valid(str: &String) -> bool {
        let mut count = 0;
        for c in str.chars() {
            if c == '(' {
                count += 1;
            } else {
                count -= 1;
            }
            if count < 0 {
                return false;
            }
        }
        count == 0
    }

    fn gen_all(len: i32, str: String, list: &mut Vec<String>) {
        if str.chars().count() == len as usize {
            if Self::is_valid(&str) {
                list.push(str);
            }
            return;
        }
        Self::gen_all(len, str.clone() + "(", list);
        Self::gen_all(len, str.clone() + ")", list);
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut list = vec![];
        Self::gen_all(n * 2, "".to_owned(), &mut list);
        list
    }

    fn gen_all_1(left: i32, right: i32, str: String, list: &mut Vec<String>) {
        if left == 0 && right == 0 {
            list.push(str);
            return;
        }
        if left > 0 {
            Self::gen_all_1(left - 1, right, str.clone() + "(", list);
        }
        if right > left {
            Self::gen_all_1(left, right - 1, str.clone() + ")", list);
        }
    }

    pub fn generate_parenthesis_1(n: i32) -> Vec<String> {
        let mut list = vec![];
        Self::gen_all_1(n, n, "".to_owned(), &mut list);
        list
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec_of_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
        assert_eq!(
            Solution::generate_parenthesis_1(3),
            vec_of_string!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
