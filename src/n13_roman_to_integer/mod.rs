// https://leetcode.com/problems/roman-to-integer/
// https://leetcode-cn.com/problems/roman-to-integer/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut last = 0;
        for char in s.chars() {
            let val = match char {
                'M' => 1000,
                'D' => 500,
                'C' => 100,
                'L' => 50,
                'X' => 10,
                'V' => 5,
                'I' => 1,
                _ => 0,
            };
            if val > last {
                sum += val - last - last;
            } else {
                sum += val;
            }
            last = val;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
