/// https://leetcode.com/problems/defanging-an-ip-address/
/// https://leetcode-cn.com/problems/defanging-an-ip-address/

pub struct Solution {}

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.split('.').collect::<Vec<&str>>().join("[.]")
    }
    pub fn defang_i_paddr1(address: String) -> String {
        let mut str = "".to_string();
        for c in address.chars() {
            if c != '.' {
                str.push(c);
            } else {
                str.push_str("[.]");
            }
        }
        str
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_defang_i_paddr() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
        assert_eq!(
            Solution::defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0".to_string()
        );

        assert_eq!(
            Solution::defang_i_paddr1("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
        assert_eq!(
            Solution::defang_i_paddr1("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0".to_string()
        );
    }
}
