use std::collections::HashMap;

/// https://leetcode.com/problems/group-anagrams/
///
/// https://leetcode-cn.com/problems/group-anagrams/
pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            let mut v = str.chars().collect::<Vec<char>>();
            v.sort();
            let sorted = v.iter().collect::<String>();
            map.entry(sorted).or_insert(vec![]).push(str);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    fn to_hash_set(vec: Vec<Vec<String>>) -> HashSet<Vec<String>> {
        vec.into_iter().fold(HashSet::new(), |mut set, current| {
            set.insert(current);
            set
        })
    }

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            to_hash_set(Solution::group_anagrams(vec_of_string![
                "eat", "tea", "tan", "ate", "nat", "bat"
            ])),
            to_hash_set(vec![
                vec_of_string!["eat", "tea", "ate"],
                vec_of_string!["tan", "nat"],
                vec_of_string!["bat"]
            ])
        );
    }
}
