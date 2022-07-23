// 590. N 叉树的后序遍历
// https://leetcode.cn/problems/n-ary-tree-postorder-traversal/

/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node|null} root
 * @return {number[]}
 */
var postorder = function (root) {
  let res = [];

  function dfs(node) {
    if (node) {
      for (let i = 0; i < node.children.length; i++) {
        dfs(node.children[i]);
      }
      res.push(node.val);
    }
  }

  dfs(root);
  return res;
};