// 102. 二叉树的层序遍历
// https://leetcode.cn/problems/binary-tree-level-order-traversal/

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
var levelOrder = function (root) {
  let result = [];
  if (!root) {
    return result;
  }

  const list = [];
  list.push(root);

  while (list.length > 0) {
    let count = list.length;
    let part = [];
    for (let i = 0; i < count; i++) {
      let node = list.shift();
      part.push(node.val);
      if (node.left) {
        list.push(node.left);
      }
      if (node.right) {
        list.push(node.right);
      }
    }
    result.push(part);
  }
  return result;
};
