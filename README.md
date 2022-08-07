# 数据结构

## 图

### 简单

[323. 无向图中连通分量的数目](https://leetcode.cn/problems/number-of-connected-components-in-an-undirected-graph/)

1. 构建图（邻接表）
2. 构建边
3. 深度优先遍历

### 中等

- [684. 冗余连接](https://leetcode.cn/problems/redundant-connection/)

# 算法

## 广度优先遍历

- 学习[广度优先遍历](https://leetcode.cn/leetbook/read/bfs/eqj8ps/)

树的广度优先遍历的写法模式相对固定：

- 使用队列
- 在队列非空的时候，取出队列元素。（记住队列的长度）
- 根据记住的队列长度遍历，并向队列中添加回子元素

实例：

1. [102. 二叉树的层序遍历](https://leetcode.cn/problems/binary-tree-level-order-traversal/)

## 深度优先遍历 （javascript）

- 学习[深度优先遍历](https://leetcode.cn/leetbook/read/dfs/euoui2/)

### 递归

代码风格上有几种形式

1. 直接递归本身，函数本身返回结果，函数内部有终止和递归。参考[104. 二叉树的最大深度](https://leetcode.cn/problems/maximum-depth-of-binary-tree/)
2. 需要用`res`存放结果，闭包函数`dfs(node)`调用存放结果，不返回值。参考[144. 二叉树的前序遍历](https://leetcode.cn/problems/binary-tree-preorder-traversal/)
3. 使用闭包函数`dfs(node)`返回结果。参考[\*129. 求根节点到叶节点数字之和](https://leetcode.cn/problems/sum-root-to-leaf-numbers/)

## 回溯算法

- 回溯算法是在一棵隐式的树或者图进行一次深度优先遍历。
- 当要访问同级下个节点时，需要恢复第一次来到此同级的状态。（涉及状态保存和恢复）

### 题目

- [51. N 皇后](https://leetcode.cn/problems/n-queens/)
- [46. 全排列](https://leetcode.cn/problems/permutations/)
