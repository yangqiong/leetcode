// https://leetcode.cn/problems/binary-search/
// 给定一个 n 个元素有序的（升序）整型数组 nums 和一个目标值 target  ，写一个函数搜索 nums 中的 target，如果目标值存在返回下标，否则返回 -1。

use std::cmp::{Ord, Ordering};

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = -1;
        let mut l = 0;
        let mut h = nums.len();
        while l < h {
            let mid = (l + h) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => l = mid + 1,
                Ordering::Greater => h = mid,
            }
        }

        result
    }
}
