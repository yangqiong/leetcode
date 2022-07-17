/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function (nums) {
  let length = nums.length;

  let res = [];
  let part = [];
  let isAdd = [];

  function backtracking(row) {
    if (row === length) {
      res.push([...part]);
      return;
    }

    for (let i = 0; i < length; i++) {
      let num = nums[i];
      if (!isAdd[num]) {
        part.push(num);
        isAdd[num] = true;
        backtracking(row + 1);
        isAdd[num] = false;
        part.pop();
      }
    }
  }

  backtracking(0);

  return res;
};

console.log(permute([1, 2, 3]));