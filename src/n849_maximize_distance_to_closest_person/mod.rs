// https://leetcode.com/problems/maximize-distance-to-closest-person/
// https://leetcode-cn.com/problems/maximize-distance-to-closest-person/

pub struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let len = seats.len();
        let (mut left_max, mut middle_max, mut right_max) = (0, 0, 0);
        let (mut i, mut j) = (0, len - 1);
        while i < len && seats[i] == 0 {
            left_max += 1;
            i += 1;
        }
        while j >= 0 && seats[j] == 0 {
            right_max += 1;
            j -= 1;
        }

        let mut res = left_max.max(right_max);
        for index in i + 1..j {
            if seats[index] == 0 {
                middle_max += 1;
            } else {
                res = res.max((middle_max as f32 / 2.0).ceil() as i32);
                middle_max = 0;
            }
        }
        res.max((middle_max as f32 / 2.0).ceil() as i32)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_max_dist_to_closest() {
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 1]), 1);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 1]), 1);
    }
}
