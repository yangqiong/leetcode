// 429. N 叉树的层序遍历
// https://leetcode.cn/problems/n-ary-tree-level-order-traversal/

/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node|null} root
 * @return {number[][]}
 */
var levelOrder = function (root) {
  let result = [];
  if (!root) {
    return result;
  }

  let queue = [];
  queue.push(root);

  while (queue.length > 0) {
    let count = queue.length;
    let part = [];
    for (let i = 0; i < count; i++) {
      let node = queue.shift();
      part.push(node.val);

      for (let j = 0; j < node.children.length; j++) {
        queue.push(node.children[j]);
      }
    }
    result.push(part);
  }

  return result;
};
