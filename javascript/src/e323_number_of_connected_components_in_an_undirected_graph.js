/**
 * @param {number} n
 * @param {number[][]} edges
 * @return {number}
 */
var countComponents = function (n, edges) {
  // 1. 构件图
  let adj = [];
  for (let i = 0; i < n; i++) {
    adj.push([])
  }

  // 2. 构建边
  for (let i = 0; i < edges.length; i++) {
    let edgi = edges[i];
    adj[edgi[0]].push(edgi[1]);
    adj[edgi[1]].push(edgi[0]);
  }

  // 3. 深度优先遍历
  let count = 0;
  let isVisit = [];
  for (let i = 0; i < n; i++) {
    if (isVisit[i] !== true) {
      count++;
      dfs(adj, i);
    }
  }



  function dfs(adj, i) {
    isVisit[i] = true;
    for (let j = 0; j < adj[i].length; j++) {
      let edge = adj[i][j];
      if (isVisit[edge] !== true) {
        dfs(adj, edge);
      }
    }
  }

  return count;
};