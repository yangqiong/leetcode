// 784. 字母大小写全排列
// https://leetcode.cn/problems/letter-case-permutation/

/**
 * @param {string} s
 * @return {string[]}
 */
var letterCasePermutation = function (s) {
  let res = [];

  let length = s.length;
  function backtracking(i, part) {
    if (i === length) {
      res.push(part.join(""));
      return;
    }

    let c = s[i];
    if (/[0-9]/.test(c)) {
      backtracking(i + 1, [...part, c]);
    }

    if (/[a-z]/.test(c)) {
      backtracking(i + 1, [...part, c]);
      backtracking(i + 1, [...part, c.toUpperCase()]);
    }

    if (/[A-Z]/.test(c)) {
      backtracking(i + 1, [...part, c]);
      backtracking(i + 1, [...part, c.toLowerCase()]);
    }

  }

  backtracking(0, []);

  return res;
};
