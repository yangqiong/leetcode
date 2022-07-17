// N 皇后

/**
 * @param {number} n
 * @return {string[][]}
 */
var solveNQueens = function (n) {
  let res = [];
  let part = [];

  let col = [];
  let main = [];
  let sub = [];

  function backtracking(row) {
    if (row === n) {
      res.push([...part]);
      return;
    }

    for (let j = 0; j < n; j++) {
      if (!col[j] && !main[row - j + n - 1] && !sub[row + j]) {
        part.push(j);
        col[j] = true;
        main[row - j + n - 1] = true;
        sub[row + j] = true;
        backtracking(row + 1);
        col[j] = false
        main[row - j + n - 1] = false;
        sub[row + j] = false;
        part.pop();
      }
    }
  }

  backtracking(0);

  res = res.map(part => {
    return part.map(num => {
      let result = [];
      for (let i = 0; i < n; i++) {
        if (i === num) {
          result.push("Q")
        } else {
          result.push(".")
        }
      }
      return result.join("");
    })
  })
  return res;
};