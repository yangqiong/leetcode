// 剑指 Offer 32 - I. 从上到下打印二叉树
// https://leetcode.cn/problems/cong-shang-dao-xia-da-yin-er-cha-shu-lcof/

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[]}
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
    for (let i = 0; i < count; i++) {
      let node = queue.shift();
      result.push(node.val);
      if (node.left) {
        queue.push(node.left);
      }
      if (node.right) {
        queue.push(node.right);
      }
    }
  }

  return result;
};
