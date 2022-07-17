// https://leetcode.cn/problems/relative-ranks/
// 给你一个长度为 n 的整数数组 score ，其中 score[i] 是第 i 位运动员在比赛中的得分。所有得分都 互不相同 。
// 运动员将根据得分 决定名次 ，其中名次第 1 的运动员得分最高，名次第 2 的运动员得分第 2 高，依此类推。运动员的名次决定了他们的获奖情况：
// 名次第 1 的运动员获金牌 "Gold Medal" 。
// 名次第 2 的运动员获银牌 "Silver Medal" 。
// 名次第 3 的运动员获铜牌 "Bronze Medal" 。
// 从名次第 4 到第 n 的运动员，只能获得他们的名次编号（即，名次第 x 的运动员获得编号 "x"）。
// 使用长度为 n 的数组 answer 返回获奖，其中 answer[i] 是第 i 位运动员的获奖情况。

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut rank = score.clone();
        rank.sort_by(|a, b| b.cmp(a));
        let mut rank_map: HashMap<i32, i32> = HashMap::new();
        for (rank, &score) in rank.iter().enumerate() {
            rank_map.insert(score, (rank + 1) as i32);
        }
        for s in score.iter() {
            match rank_map.get(s) {
                None => (),
                Some(&s) => match s {
                    1 => result.push("Gold Medal".to_string()),
                    2 => result.push("Silver Medal".to_string()),
                    3 => result.push("Bronze Medal".to_string()),
                    _ => result.push(s.to_string()),
                },
            }
        }
        result
    }
}
