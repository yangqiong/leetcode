// 剑指 Offer 32 - III. 从上到下打印二叉树 III
// https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-iii-lcof/

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
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
