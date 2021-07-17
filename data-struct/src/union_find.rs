//! 并查集：解决节点连接/关联问题
trait UF {
    fn is_connected(&mut self, p: usize, q: usize) -> bool;
    fn union_elements(&mut self, p: usize, q: usize);
    fn get_size(&self) -> usize;
}

/// rank[i]表示以i为根的集合所表示的树的层数
/// 在后续的代码中, 并不会维护rank的语意, 也就是rank的值在路径压缩的过程中, 有可能不在是树的层数值
/// 这也是rank不叫height或者depth的原因, 它只是作为比较的一个标准
#[derive(Debug)]
pub struct UnionFind {
    rank: Vec<i32>,
    parent: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let mut rank = vec![0; size];
        let mut parent = vec![0; size];
        for i in 0..size {
            rank[i] = i as i32;
            parent[i] = i as i32;
        }

        UnionFind { rank, parent }
    }
}

#[test]
fn test_union_find() {
    let uf = UnionFind::new(10);
    println!("UF: {:?}", uf);
}
