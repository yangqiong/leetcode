// 111. 二叉树的最小深度
// https://leetcode.cn/problems/minimum-depth-of-binary-tree/

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
var minDepth = function (root) {
  if (root) {
    if (root.left && root.right) {
      return Math.min(minDepth(root.left), minDepth(root.right)) + 1
    }
    if (root.left && !root.right) {
      return minDepth(root.left) + 1;
    }
    if (!root.left && root.right) {
      return minDepth(root.right) + 1;
    }
    if (!root.left && !root.right) {
      return 1;
    }
  } else {
    return 0;
  }
};