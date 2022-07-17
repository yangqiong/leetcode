// https://leetcode.cn/problems/hamming-distance/
// 两个整数之间的 汉明距离 指的是这两个数字对应二进制位不同的位置的数目。

struct Solution {}

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut distance = 0;
        let mut z = x ^ y;
        while z > 0 {
            z = z & (z - 1); // 计算二进制含1的个数（每次减少二进制最右边的1）
            distance += 1;
        }
        distance
    }
}
