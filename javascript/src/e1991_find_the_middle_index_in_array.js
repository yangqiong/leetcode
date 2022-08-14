// 1991. 找到数组的中间位置
// https://leetcode.cn/problems/find-the-middle-index-in-array/

/**
 * @param {number[]} nums
 * @return {number}
 */
var findMiddleIndex = function (nums) {
  let sum = 0;
  for (let i = 0; i < nums.length; i++) {
    sum += nums[i];
  }

  let left = 0;
  let right = sum;

  for (let i = 0; i < nums.length; i++) {
    right = right - nums[i];
    if (left === right) {
      return i;
    }
    left += nums[i];
  }
  return -1;
};
