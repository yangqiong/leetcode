// https://leetcode.cn/problems/merge-sorted-array/
// 给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
// 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if m > 0 {
            let mut i = m - 1;
            let mut j = n - 1;
            for index in (0..m + n).rev() {
                if i >= 0 && j >= 0 {
                    if nums1[i as usize] >= nums2[j as usize] {
                        nums1[index as usize] = nums1[i as usize];
                        i = i - 1;
                    } else {
                        nums1[index as usize] = nums2[j as usize];
                        j = j - 1;
                    }
                }
            }
            if j >= 0 {
                for index in (0..=j) {
                    nums1[index as usize] = nums2[index as usize]
                }
            }
        }
        if m == 0 {
            for index in (0..n) {
                nums1[index as usize] = nums2[index as usize]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_e88() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn it_works_e88_test2() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let m = 0;
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn it_works_e88_test3() {
        let mut nums1 = vec![2, 0];
        let mut nums2 = vec![1];
        let m = 1;
        let n = 1;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2]);
    }

    #[test]
    fn it_works_e88_test4() {
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        let m = 3;
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
