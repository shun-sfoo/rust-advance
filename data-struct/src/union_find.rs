//! 并查集：解决节点连接/关联问题
trait UF {
    fn is_connected(&mut self, p: usize, q: usize) -> bool;
    fn union_elements(&mut self, p: usize, q: usize);
    fn get_size(&self) -> usize;
}

/**
 * rank[i]表示以i为根的集合所表示的树的层数
 * 在后续的代码中, 并不会维护rank的语意, 也就是rank的值在路径压缩的过程中, 有可能不在是树的层数值
 * 这也是rank不叫height或者depth的原因, 它只是作为比较的一个标准
 */
#[derive(Debug)]
pub struct UnionFind {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new_with_size(size: usize) -> Self {
        let mut rank = vec![0 as usize; size];
        let mut parent = vec![0 as usize; size];

        // 初始化, 每一个parent[i]指向自己, 表示每一个元素自己自成一个集合
        for i in 0..size {
            rank[i] = i;
            parent[i] = i;
        }

        UnionFind { rank, parent }
    }

    pub fn get_size(&self) -> usize {
        self.parent.len()
    }

    /**
     * 查找过程, 查找元素p所对应的集合编号
     * O(h)复杂度, h为树的高度
     */
    fn find(&mut self, mut p: usize) -> Result<usize, &'static str> {
        if p < 0 || p >= self.parent.len() {
            return Err("p is out of bound.");
        }

        while p != self.parent[p] {
            self.parent[p] = self.parent[self.parent[p]];
            p = self.parent[p];
        }

        Ok(p)
    }
}

impl UF for UnionFind {
    fn is_connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p).unwrap() == self.find(q).unwrap()
    }

    fn union_elements(&mut self, p: usize, q: usize) {
        let p_root = self.find(p).unwrap();
        let q_root = self.find(q).unwrap();
        if p_root == q_root {
            return;
        }

        if self.rank[p_root] < self.rank[q_root] {
            self.parent[p_root] = q_root;
        } else if self.rank[p_root] > self.rank[q_root] {
            self.parent[q_root] = p_root;
        } else {
            self.rank[p_root] = q_root;
            self.rank[p_root] += 1;
        }
    }

    fn get_size(&self) -> usize {
        self.get_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut union_find = UnionFind::new_with_size(10);
        union_find.union_elements(3, 5);
        union_find.union_elements(2, 1);
        union_find.union_elements(5, 1);
        union_find.union_elements(5, 4);
        assert_eq!(union_find.is_connected(4, 1), true);
    }
}
