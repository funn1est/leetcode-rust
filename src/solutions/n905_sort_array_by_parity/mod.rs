/// https://leetcode.com/problems/sort-array-by-parity/
/// https://leetcode-cn.com/problems/sort-array-by-parity/

pub struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(a.len());
        for num in a {
            if num % 2 == 0 {
                res.insert(0, num);
            } else {
                res.push(num);
            }
        }
        res
    }

    pub fn sort_array_by_parity1(a: Vec<i32>) -> Vec<i32> {
        let mut res = a.clone();
        let (mut start, mut end) = (0, a.len() - 1);
        for num in a {
            if num % 2 == 0 {
                res[start] = num;
                start += 1;
            } else {
                res[end] = num;
                end -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(
            Solution::sort_array_by_parity(vec![3, 1, 2, 4]),
            vec![4, 2, 3, 1]
        );

        assert_eq!(
            Solution::sort_array_by_parity1(vec![3, 1, 2, 4]),
            vec![2, 4, 1, 3]
        );
    }
}
