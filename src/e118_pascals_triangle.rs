// https://leetcode.cn/problems/pascals-triangle/
// 给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。

use std::result;

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        for i in 0..num_rows {
            match i {
                0 => {
                    result.push(vec![1]);
                }
                1 => {
                    result.push(vec![1, 1]);
                }
                _ => {
                    let mut nums = vec![1];
                    let last_nums = &result[(i - 1) as usize];
                    for j in 0..(last_nums.len() - 1) {
                        nums.push(last_nums[j] + last_nums[j + 1]);
                    }
                    nums.push(1);
                    result.push(nums)
                }
            }
        }
        result
    }
}
