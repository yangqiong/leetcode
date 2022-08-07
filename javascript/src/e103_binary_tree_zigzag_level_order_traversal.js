// 103. 二叉树的锯齿形层序遍历
// https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/

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
 * @return {number[][]}
 */
var zigzagLevelOrder = function (root) {
  let result = [];
  if (!root) {
    return result;
  }
  let queue = [];
  queue.push(root);
  let reverse = false;

  while (queue.length > 0) {
    let part = [];
    let count = queue.length;

    for (let i = 0; i < count; i++) {
      let node = queue.shift();
      part.push(node.val);
      if (node.left) {
        queue.push(node.left);
      }
      if (node.right) {
        queue.push(node.right);
      }
    }
    if (reverse) {
      part = part.reverse();
    }
    result.push(part);

    reverse = !reverse;
  }

  return result;
};
