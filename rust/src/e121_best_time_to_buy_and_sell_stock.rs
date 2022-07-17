use std::cmp::{max, min};

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        let mut min_num = prices[0];
        let mut max_num = 0;
        for i in 1..prices.len() {
            max_num = max(max_num, prices[i] - min_num);
            min_num = min(min_num, prices[i]);
        }
        max_num
    }
}
