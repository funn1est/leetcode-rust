/// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
///
/// https://leetcode-cn.com/problems/find-numbers-with-even-number-of-digits/
pub struct Solution {}

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |all, &x| {
            return if (x as u32).to_string().len() % 2 == 0 {
                all + 1
            } else {
                all
            };
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_numbers() {
        assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
        assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
