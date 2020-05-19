// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
// https://leetcode-cn.com/problems/number-of-steps-to-reduce-a-number-to-zero/

pub struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut count = 0;
        while num != 0 {
            match num % 2 {
                0 => num /= 2,
                _ => num -= 1,
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_number_of_steps() {
        assert_eq!(Solution::number_of_steps(14), 6);
        assert_eq!(Solution::number_of_steps(8), 4);
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
