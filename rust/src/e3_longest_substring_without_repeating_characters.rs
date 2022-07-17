// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。

struct Solution {}

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = 0;
        let mut hash_map: HashMap<char, usize> = HashMap::new();
        let mut result = 0;

        loop {
            let c = chars.get(j);
            match c {
                None => break,
                Some(c) => {
                    let index = hash_map.get(&c);
                    match index {
                        None => {
                            hash_map.insert(*c, j);
                            j += 1;
                            result = max(result, j - i);
                        }
                        Some(&index) => {
                            for k in i..index {
                                hash_map.remove(&chars[k]);
                            }
                            hash_map.insert(*c, j);
                            i = index + 1;
                            j += 1;
                        }
                    };
                }
            };
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_e3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("tmmzuxt".to_string()),
            5
        );
    }
}
