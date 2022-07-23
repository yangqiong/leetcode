// 112. 路径总和
// https://leetcode.cn/problems/path-sum/

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
 * @param {number} targetSum
 * @return {boolean}
 */
var hasPathSum = function (root, targetSum) {
  if (root) {
    if (!root.left && !root.right) {
      if (root.val === targetSum) {
        return true;
      } else {
        return false;
      }
    }

    if (root.left && !root.right) {
      return hasPathSum(root.left, targetSum - root.val);
    }

    if (!root.left && root.right) {
      return hasPathSum(root.right, targetSum - root.val);
    }

    if (root.left && root.right) {
      return hasPathSum(root.left, targetSum - root.val) || hasPathSum(root.right, targetSum - root.val);
    }
  } else {
    return false;
  }
};