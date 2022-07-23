// 802. 找到最终的安全状态
// https://leetcode.cn/problems/find-eventual-safe-states/

/**
 * @param {number[][]} graph
 * @return {number[]}
 */
var eventualSafeNodes = function (graph) {
  let res = [];

  let safe = {};

  function dfs(i) {
    if (safe[i] !== undefined) {
      return safe[i];
    }

    safe[i] = false;

    for (let nei of graph[i]) {
      if (!dfs(nei)) {
        return safe[i]
      }
    }

    safe[i] = true;

    return res;
  }

  for (let i = 0; i < graph.length; i++) {
    if (dfs(i)) {
      res.push(i)
    }
  }

  return res;
};