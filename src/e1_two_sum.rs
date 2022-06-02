// https://leetcode.cn/problems/two-sum/
// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。

use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        let mut result: Vec<i32> = vec![];

        for (index, num) in nums.iter().enumerate() {
            match num_map.get(&(&target - num)) {
                Some(i) => {
                    result.push(*i as i32);
                    result.push(index as i32);
                    break;
                }
                None => {
                    num_map.insert(*num, index);
                }
            }
        }

        result
    }
}
