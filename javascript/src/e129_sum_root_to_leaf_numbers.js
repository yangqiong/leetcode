// 129. 求根节点到叶节点数字之和
// https://leetcode.cn/problems/sum-root-to-leaf-numbers/

/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var sumNumbers = function (root) {
  function dfs(node, cumsum) {
    if (node) {
      cumsum = cumsum * 10 + node.val;
      if (node.left && node.right) {
        return dfs(node.left, cumsum) + dfs(node.right, cumsum);
      }
      if (node.left && !node.right) {
        return dfs(node.left, cumsum);
      }
      if (!node.left && node.right) {
        return dfs(node.right, cumsum)
      }
      if (!node.left && !node.right) {
        return cumsum
      }
    } else {
      return cumsum;
    }
  }

  if (root) {
    return dfs(root, root.val)
  }

  return 0;
};