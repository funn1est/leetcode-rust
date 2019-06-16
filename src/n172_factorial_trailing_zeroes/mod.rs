pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        fn helper(n: i32, total: i32) -> i32 {
            if n < 5 {
                total
            } else {
                let count = n / 5;
                helper(count, total + count)
            }
        }
        helper(n, 0)
    }
    pub fn trailing_zeroes1(n: i32) -> i32 {
        let mut n = n;
        let mut total = 0;
        while n >= 5 {
            n /= 5;
            total += n;
        }
        total
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(1808548329), 452137076);

        assert_eq!(Solution::trailing_zeroes1(3), 0);
        assert_eq!(Solution::trailing_zeroes1(5), 1);
        assert_eq!(Solution::trailing_zeroes1(1808548329), 452137076);
    }
}
