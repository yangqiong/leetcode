// https://leetcode.cn/problems/valid-parentheses/
// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

const BRACES: [u8; 3] = [b'[', b'(', b'{'];

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        s.as_bytes().iter().all(|b| match b {
            b'}' => stack.pop() == Some(b'{'),
            b']' => stack.pop() == Some(b'['),
            b')' => stack.pop() == Some(b'('),
            _ => {
                if BRACES.contains(b) {
                    stack.push(*b);
                }
                true
            }
        }) && stack.len() == 0
    }
}
