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
    pub fn new(size: usize) -> Self {
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

#[test]
fn test_union_find() {
    let uf = UnionFind::new(10);
    println!("UF: {:?}", uf);
}
