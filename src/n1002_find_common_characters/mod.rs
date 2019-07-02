// https://leetcode.com/problems/find-common-characters/
// https://leetcode-cn.com/problems/find-common-characters/

pub struct Solution;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let len = a.len();
        let mut res = vec![];
        let mut count = vec![];

        for string in a {
            let mut row = vec![0u8; 26];
            for char in string.chars() {
                row[(char as usize) - 97] += 1;
            }
            count.push(row);
        }

        for i in 0..26 {
            let mut min = u8::max_value();
            for j in 0..len {
                min = min.min(count[j][i]);
            }

            for _ in 0..min {
                res.push(((i as u8 + 97) as char).to_string());
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_common_chars() {
        assert_eq!(
            Solution::common_chars(vec_of_string!["bella", "label", "roller"]),
            vec_of_string!["e", "l", "l"]
        );
        assert_eq!(
            Solution::common_chars(vec_of_string!["cool", "lock", "cook"]),
            vec_of_string!["c", "o"]
        );
    }
}
