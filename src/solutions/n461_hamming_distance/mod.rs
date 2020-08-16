/// https://leetcode.com/problems/hamming-distance/
///
/// https://leetcode-cn.com/problems/hamming-distance/
pub struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        format!("{:b}", x ^ y)
            .chars()
            .fold(0, |acc, x| if x == '1' { acc + 1 } else { acc })
    }
    pub fn hamming_distance1(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
    pub fn hamming_distance2(x: i32, y: i32) -> i32 {
        format!("{:b}", x ^ y).matches('1').count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance1(1, 4), 2);
        assert_eq!(Solution::hamming_distance2(1, 4), 2);
    }
}
