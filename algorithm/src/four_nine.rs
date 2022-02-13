use std::collections::HashMap;
use core::fmt::Write;
use std::fmt::format;

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::<String, Vec<String>>::new();
        let mut count = [0; 26];
        for w in strs {
            count.fill(0);
            for c in w.as_bytes() {
                count[(*c as u8 - 'a' as u8) as usize] += 1;
            }
            let mut cs = String::new();
            for c in count {
                cs += &*format!(",{}", c);
            }
            let v = map.get_mut(&cs);
            if v.is_some() {
                v.unwrap().push(w);
            } else {
                map.insert(cs, vec![w]);
            }
        }
        let mut res = vec![];
        for v in map.into_values() {
            res.push(v);
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use crate::four_nine::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::group_anagrams(vec!["bdddddddddd".to_string(), "bbbbbbbbbbc".to_string()]),
                   vec![vec!["bdddddddddd".to_string()], vec!["bbbbbbbbbbc".to_string()]]);
    }
}
