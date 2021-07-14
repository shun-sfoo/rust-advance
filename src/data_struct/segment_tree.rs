//! 线段树

/**
 * 线段树是满二叉树，None代替没有元素的叶子节点
 * 线段树用于解决区间查询问题，所以不关心元素的增加删除操作。
 * 简单实现线段树的结构，所以这里没有写到修改线段树的元素
 * 查询时间复杂度O(logn)，直接更新子节点的时间复杂度为O(n)
 */

// 线段区间元素的融合方式trait 定义及其实现
trait Merger {
    fn merge(&self, l: i32, r: i32) -> i32;
}

impl Merger for i32 {
    fn merge(&self, l: i32, r: i32) -> i32 {
        return l + r;
    }
}

// 线段树定义
pub struct SegmentTree {
    data: Vec<i32>,
    tree: Vec<Option<i32>>,
    // trait object
    merger: Box<dyn Merger>,
}

impl SegmentTree {
    fn new_segment_tree(arr: Vec<i32>, merger: Box<dyn Merger>) -> SegmentTree {
        let data_len = arr.len();
        Self {
            data: arr,
            tree: vec![None; 4 * data_len],
            merger,
        }
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.data.len() {
            return None;
        }
        return Some(self.data[index]);
    }

    pub fn get_size(&self) -> usize {
        return self.data.len();
    }

    fn left_child(index: usize) -> usize {
        return 2 * index + 1;
    }

    fn right_child(index: usize) -> usize {
        return 2 * index + 2;
    }

    pub fn build(&mut self) {
        self.build_segment_tree(0, 0, self.data.len() - 1);
    }

    // 递归
    fn build_segment_tree(&mut self, tree_index: usize, left: usize, right: usize) {
        if left == right {
            self.tree[tree_index] = Some(self.data[left]);
            return;
        }
        let left_tree_index = Self::left_child(tree_index);
        let right_tree_index = Self::right_child(tree_index);
        let mid = (right - left) / 2 + left;
        self.build_segment_tree(left_tree_index, left, mid);
        self.build_segment_tree(right_tree_index, mid + 1, right);
        // 左右子树数据处理方式
        if let Some(l) = self.tree[left_tree_index] {
            if let Some(r) = self.tree[right_tree_index] {
                self.tree[tree_index] = Some(self.merger.merge(l, r))
            }
        }
    }

    // [l:r]
    pub fn query(&self, l: usize, r: usize) -> Result<i32, &'static str> {
        if l > self.data.len() || r > self.data.len() || l > r {
            return Err("参数错误");
        }

        Ok(self.recursion_query(0, 0, self.data.len() - 1, l, r))
    }

    // tree_index的[l..r]范围中查询[query_left,query_right]值
    fn recursion_query(
        &self,
        tree_index: usize,
        l: usize,
        r: usize,
        query_left: usize,
        query_right: usize,
    ) -> i32 {
        if l == query_left && r == query_right {
            if let Some(d) = self.tree[tree_index] {
                return d;
            }
            return 0;
        }
        let mid = l + (r - l) / 2;
        let l_t_ind = Self::left_child(tree_index);
        let r_t_ind = Self::right_child(tree_index);
        if query_left >= mid + 1 {
            // 都在右子树
            return self.recursion_query(r_t_ind, mid + 1, r, query_left, query_right);
        } else if query_right <= mid {
            // 都在左子树
            return self.recursion_query(l_t_ind, l, mid, query_left, query_right);
        }
        // 查询范围在两侧
        let l_res = self.recursion_query(l_t_ind, l, mid, query_left, mid);
        let r_res = self.recursion_query(r_t_ind, mid + 1, r, mid + 1, query_right);
        self.merger.merge(l_res, r_res)
    }
    // 更新index为e
    pub fn set(&mut self, index: usize, e: i32) -> Result<(), &'static str> {
        if index >= self.data.len() {
            return Err("参数错误");
        }
        // 更新数据
        self.data[index] = e;
        // 递归更新树
        self.recursion_set(0, 0, self.data.len() - 1, index, e);
        Ok(())
    }

    fn recursion_set(&mut self, index_tree: usize, l: usize, r: usize, index: usize, e: i32) {
        if l == r {
            self.tree[index_tree] = Some(e);
            return;
        }
        let mid = l + (r - l) / 2;
        let left_child = Self::left_child(index_tree);
        let right_child = Self::right_child(index_tree);
        if index >= mid + 1 {
            self.recursion_set(right_child, mid + 1, r, index, e);
        } else {
            self.recursion_set(left_child, l, mid, index, e);
        }
        // 左右子树数据求和
        if let Some(l_d) = self.tree[left_child] {
            if let Some(r_d) = self.tree[right_child] {
                self.tree[index_tree] = Some(self.merger.merge(l_d, r_d));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = vec![2, 0, -3, 55, 2, -1];
        let merger = Box::new(0_i32);
        assert_eq!(merger.merge(1, 2), 3);
        let mut tree = SegmentTree::new_segment_tree(data, merger);
        tree.build();
        assert_eq!(tree.query(0, 2), Ok(-1));
        assert_eq!(tree.query(2, 5), Ok(53));
        assert_eq!(tree.query(0, 5), Ok(55));
        tree.set(1, 1);
        assert_eq!(tree.query(0, 2), Ok(0));
    }
}
