// 93. 复原 IP 地址
// https://leetcode.cn/problems/restore-ip-addresses/

/**
 * @param {string} s
 * @return {string[]}
 */
var restoreIpAddresses = function (s) {
  let res = [];
  let part = [];

  function backtracking(i) {
    if (part.length === 4 && i === s.length) {
      res.push(part.join("."));
      return;
    }

    for (let j = i; j < s.length; j++) {
      let str = s.slice(i, j + 1);
      if (isValid(str)) {
        part.push(str);
        backtracking(j + 1);
        part.pop();
      }
    }
  }

  backtracking(0);

  return res;
};

function isValid(str) {
  if (str.length === 1 && str === "0") {
    return true;
  }

  let num = parseInt(str);

  if (num >= 1 && num <= 255 && str[0] !== "0") {
    return true;
  }

  return false;
}