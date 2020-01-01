// https://leetcode.com/problems/largest-perimeter-triangle/solution/
// https://leetcode-cn.com/problems/largest-perimeter-triangle/solution/

pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        a.sort();
        for i in (0..(a.len() - 2)).rev() {
            let small = a[i] + a[i + 1];
            let big = a[i + 2];
            if small > big {
                return small + big;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_largest_perimeter() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
        assert_eq!(Solution::largest_perimeter(vec![3, 2, 3, 4]), 10);
        assert_eq!(Solution::largest_perimeter(vec![3, 6, 2, 3]), 8);
    }
}
