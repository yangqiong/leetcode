// 589. N 叉树的前序遍历
// https://leetcode.cn/problems/n-ary-tree-preorder-traversal/

/**
 * // Definition for a Node.
 * function Node(val, children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node|null} root
 * @return {number[]}
 */
var preorder = function (root) {
  let res = [];

  function dfs(node) {
    if (node) {
      res.push(node.val);
      for (let i = 0; i < node.children; i++) {
        dfs(node.children[i]);
      }
    }
  }

  dfs(root);
  return res;
};

