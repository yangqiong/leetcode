// https://leetcode.cn/problems/move-zeroes/
// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut p0: Option<usize> = None;
        for i in 0..nums.len() {
            if p0.is_none() && nums[i] == 0 {
                p0 = Some(i);
            }
            if nums[i] != 0 && p0.is_some() {
                let p00 = p0.unwrap();
                let tmp = nums[p00];
                nums[p00] = nums[i];
                nums[i] = tmp;
                p0 = Some(p00 + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_283() {
        let mut nums = vec![0, 0, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);

        let mut nums = vec![1, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);

        let mut nums = vec![2, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![2, 1])
    }
}
