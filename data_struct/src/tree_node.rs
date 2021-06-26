type TreeNode<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
struct Node<K, V>
where
    V: std::fmt::Display,
{
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
}

trait BinaryTree<K, V> {
    fn pre_order(&mut self);
    fn in_order(&mut self);
    fn pos_order(&mut self);
}

trait BinarySearchTree<K, V>: BinaryTree<K, V>
where
    K: PartialOrd,
{
    fn insert(&mut self, key: K, value: V);
    fn contain(&mut self, key: K) -> bool;
}

impl<K, V> Node<K, V>
where
    V: std::fmt::Display,
{
    fn new(key: K, value: V) -> Self {
        Node {
            left: None,
            right: None,
            key,
            value,
        }
    }
}

impl<K, V> BinarySearchTree<K, V> for Node<K, V>
where
    K: PartialOrd,
    V: std::fmt::Display,
{
    fn insert(&mut self, key: K, value: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(Node::new(key, value)));
            }
        } else {
            if let Some(ref mut left) = self.right {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        }
    }

    fn contain(&mut self, key: K) -> bool {
        if self.key == key {
            return true;
        } else if self.key < key {
            match self.right {
                None => false,
                Some(ref mut right) => right.contain(key),
            }
        } else {
            match self.left {
                None => false,
                Some(ref mut left) => left.contain(key),
            }
        }
    }
}

impl<K, V> BinaryTree<K, V> for Node<K, V>
where
    V: std::fmt::Display,
{
    fn pre_order(&mut self) {
        println!("{}", self.value);
        if let Some(ref mut left) = self.right {
            left.pre_order();
        }

        if let Some(ref mut right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&mut self) {
        if let Some(ref mut left) = self.right {
            left.in_order();
        }
        println!("{}", self.value);
        if let Some(ref mut right) = self.right {
            right.in_order();
        }
    }

    fn pos_order(&mut self) {
        if let Some(ref mut left) = self.right {
            left.pos_order();
        }
        if let Some(ref mut right) = self.right {
            right.pos_order();
        }

        println!("{}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let mut root = Node::<i32, i32>::new(3, 4);
        root.insert(2, 3);
        root.insert(4, 6);
        root.insert(5, 5);
        root.insert(6, 6);
        root.insert(1, 8);
        println!("{:?}", root);

        if let Some(ref left) = root.left {
            assert_eq!(left.value, 3);
        }

        if let Some(ref right) = root.right {
            assert_eq!(right.value, 6);
            if let Some(ref right) = right.right {
                assert_eq!(right.value, 5);
            }
        }

        assert_eq!(true, root.contain(2));
        assert_eq!(false, root.contain(7));

        println!("Pre Order traversal");
        root.pre_order();
        println!("In Order traversal");
        root.in_order();
        println!("Pos Order traversal");
        root.pos_order();
    }
}
