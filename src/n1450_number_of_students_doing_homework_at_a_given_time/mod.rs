// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
// https://leetcode-cn.com/problems/number-of-students-doing-homework-at-a-given-time/

pub struct Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .enumerate()
            .fold(0, |all, (index, &start)| {
                if start <= query_time && query_time <= end_time[index] {
                    all + 1
                } else {
                    all
                }
            })
    }

    pub fn busy_student1(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .fold(0, |all, (&start, &end)| {
                if start <= query_time && query_time <= end {
                    all + 1
                } else {
                    all
                }
            })
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_busy_student() {
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
        assert_eq!(Solution::busy_student(vec![4], vec![4], 5), 0);
        assert_eq!(
            Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7),
            0
        );
        assert_eq!(
            Solution::busy_student(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
                5
            ),
            5
        );
    }

    #[test]
    fn test_busy_student1() {
        assert_eq!(Solution::busy_student1(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
        assert_eq!(Solution::busy_student1(vec![4], vec![4], 4), 1);
        assert_eq!(Solution::busy_student1(vec![4], vec![4], 5), 0);
        assert_eq!(
            Solution::busy_student1(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7),
            0
        );
        assert_eq!(
            Solution::busy_student1(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
                5
            ),
            5
        );
    }
}
