pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let expected = (1 + len) * len / 2;
        let sum = nums.iter().fold(0, |acc, &x| acc + x);
        expected - sum
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_missing_number() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
