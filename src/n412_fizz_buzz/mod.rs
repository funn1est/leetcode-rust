pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|x| match (x % 3 == 0, x % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                _ => x.to_string(),
            })
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec_of_string![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
