// https://leetcode.cn/problems/total-hamming-distance/
// 两个整数的 汉明距离 指的是这两个数字的二进制数对应位不同的数量。
// 给你一个整数数组 nums，请你计算并返回 nums 中任意两个数之间 汉明距离的总和

struct Solution {}

// 4： 0 1 0 0
// 14：1 1 1 0
// 2： 0 0 1 0
// -------------
// 第高位：两个0，一个1， 有2*1个
// 其次为：两个1，一个0， 有2*1个
// 其次位：两个1，一个0， 有2*1个
// 最低位：三个零，      有3*0个
// 最后得6

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut mut_nums = nums;
        let mut sum = 0;
        for _ in 0..32 {
            let mut num_of_0 = 0;
            let mut num_of_1 = 0;
            for num in mut_nums.iter_mut() {
                if *num & 01 == 01 {
                    num_of_1 += 1;
                } else {
                    num_of_0 += 1;
                }
                *num = (*num) >> 1;
            }
            sum += num_of_0 * num_of_1
        }
        sum
    }
}
