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

    pub fn generate_parenthesis_2(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut dp = vec![vec![]; n + 1];
        dp[0].push("".to_owned());
        for i in 1..=n {
            let mut list = vec![];
            for j in 0..i {
                let str1 = dp[j].clone();
                let str2 = dp[i - 1 - j].clone();
                for s1 in &str1 {
                    for s2 in &str2 {
                        list.push("(".to_owned() + s1 + ")" + s2);
                    }
                }
            }
            dp[i] = list;
        }
        dp[n].clone()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_generate_parenthesis() {
        let mut right = vec_of_string!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parenthesis(3), right);
        assert_eq!(Solution::generate_parenthesis_1(3), right);

        let mut left = Solution::generate_parenthesis_2(3);
        left.sort();
        right.sort();
        assert_eq!(left, right);
    }
}
