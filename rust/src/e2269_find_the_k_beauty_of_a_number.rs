// https://leetcode.cn/problems/find-the-k-beauty-of-a-number/
// 一个整数 num 的 k 美丽值定义为 num 中符合以下条件的 子字符串 数目：
// 子字符串长度为 k 。
// 子字符串能整除 num 。
// 给你整数 num 和 k ，请你返回 num 的 k 美丽值。

struct Solution {}

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let s = num.to_string();
        let chars = s.chars().collect::<Vec<char>>();
        chars
            .windows(k as usize)
            .filter(|w| {
                let child: i32 = w.into_iter().collect::<String>().parse().unwrap();
                if child != 0 {
                    num % child == 0
                } else {
                    false
                }
            })
            .count() as i32
    }
}
