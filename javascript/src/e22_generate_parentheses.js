// 22. 括号生成

/**
 * @param {number} n
 * @return {string[]}
 */
var generateParenthesis = function (n) {
  let res = [];
  let part = [];

  function backtracking(openN, closeN) {
    if (openN === closeN && openN === n) {
      res.push(part.join(""));
      return;
    }

    if (openN < n) {
      part.push("(");
      backtracking(openN + 1, closeN);
      part.pop();
    }

    if (closeN < openN) {
      part.push(")")
      backtracking(openN, closeN + 1);
      part.pop();
    }
  }

  backtracking(0, 0);

  return res;
};