// https://leetcode.cn/problems/single-number/submissions/
// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

struct Solution {}

// 异或 a ^ 0 = a; a ^ a = 0;
impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            result ^= nums[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_e136() {
        assert_eq!(Solution::single_number(vec![1, 2, 1]), 2);
    }
}
