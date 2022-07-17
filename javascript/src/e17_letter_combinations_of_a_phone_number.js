// 17. 电话号码的字母组合
// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/

/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function (digits) {
  if (digits.length === 0) {
    return [];
  }

  let map = {
    "2": ["a", "b", "c"],
    "3": ["d", "e", "f"],
    "4": ["g", "h", "i"],
    "5": ["j", "k", "l"],
    "6": ["m", "n", "o"],
    "7": ["p", "q", "r", "s"],
    "8": ["t", "u", "v"],
    "9": ["w", "x", "y", "z"],
  }

  let res = [];
  let part = [];

  let length = digits.length;
  let chars = [];
  for (let i = 0; i < length; i++) {
    chars.push(map[digits[i]]);
  }

  function backtracking(i) {
    if (i === length) {
      res.push(part.join(""));
      return res;
    }

    for (let j = 0; j < chars[i].length; j++) {
      part.push(chars[i][j]);
      backtracking(i + 1);
      part.pop();
    }
  }

  backtracking(0)

  return res;
};
