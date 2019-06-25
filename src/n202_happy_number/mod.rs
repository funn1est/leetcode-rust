// https://leetcode.com/problems/happy-number/
// https://leetcode-cn.com/problems/happy-number/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;
        let mut sum = 0;
        let mut map: HashMap<i32, bool> = HashMap::new();
        while sum != 1 {
            if map.contains_key(&n) {
                return false;
            }
            map.insert(n, true);
            sum = 0;
            while n > 0 {
                sum += (n % 10).pow(2);
                n /= 10;
            }
            n = sum;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(20), false);
    }
}
