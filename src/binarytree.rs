#![allow(dead_code)]

use std::{ collections::VecDeque, ops::Deref };

enum BranchSide {
    Left,
    Right,
}

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Copy> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn pre_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        // these traversals are basically an example of Depth First Search
        visit_func(&self.value);
        match self.left.as_ref() {
            None => {}
            Some(t) => t.pre_order_traverse(visit_func),
        }
        match self.right.as_ref() {
            None => {}
            Some(t) => t.pre_order_traverse(visit_func),
        }
    }

    fn post_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        match self.left.as_ref() {
            None => {}
            Some(t) => t.post_order_traverse(visit_func),
        }
        match self.right.as_ref() {
            None => {}
            Some(t) => t.post_order_traverse(visit_func),
        }
        visit_func(&self.value);
    }

    fn in_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        match self.left.as_ref() {
            None => {}
            Some(t) => t.in_order_traverse(visit_func),
        }
        visit_func(&self.value);
        match self.right.as_ref() {
            None => {}
            Some(t) => t.in_order_traverse(visit_func),
        }
    }

    fn breadth_first_search(&self, value: T) -> bool {
        // for unordered trees
        let mut queue: VecDeque<&Node<T>> = VecDeque::from([self]);

        while queue.len() > 0 {
            let node: &Node<T> = queue.pop_front().unwrap();
            if node.value == value {
                return true;
            }
            match node.left.as_ref() {
                None => {}
                Some(t) => queue.push_back(t.deref()),
            }
            match node.right.as_ref() {
                None => {}
                Some(t) => queue.push_back(t.deref()),
            }
        }
        return false;
    }

    fn compare(a: &Node<T>, b: &Node<T>) -> bool {
        if a.value != a.value {
            return false;
        }

        let check_left: bool =
            (a.left.is_none() && b.left.is_none()) ||
            (a.left.is_some() &&
                b.left.is_some() &&
                Node::compare(a.left.as_ref().unwrap(), b.left.as_ref().unwrap()));

        let check_right: bool =
            (a.right.is_none() && b.right.is_none()) ||
            (a.right.is_some() &&
                b.right.is_some() &&
                Node::compare(a.right.as_ref().unwrap(), b.right.as_ref().unwrap()));

        return check_left && check_right;
    }

    fn node_insert(&mut self, value: T, branch: BranchSide) {
        // If the branch exists, it is overwritten
        match branch {
            BranchSide::Left => {
                self.left = Some(Box::new(Node::new(value)));
            }
            BranchSide::Right => {
                self.right = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn depth_first_search(&self, value: T) -> bool {
        // tree must be ordered. Left branch has smaller or equal values and Right has bigger values
        if self.value == value {
            return true;
        }
        if value < self.value {
            return self.left.is_some() && self.left.as_ref().unwrap().depth_first_search(value);
        } else {
            return self.right.is_some() && self.right.as_ref().unwrap().depth_first_search(value);
        }
    }

    fn insert_ordered(&mut self, value: T) {
        if value <= self.value {
            match self.left.as_mut() {
                None => { self.node_insert(value, BranchSide::Left) }
                Some(t) => {
                    t.insert_ordered(value);
                }
            }
        } else if value > self.value {
            match self.right.as_mut() {
                None => { self.node_insert(value, BranchSide::Right) }
                Some(t) => { t.insert_ordered(value) }
            }
        }
    }

    fn delete(mut this: Box<Node<T>>, target: &T) -> Option<Box<Node<T>>> {
        // source: https://stackoverflow.com/questions/64043682/how-to-write-a-delete-function-for-a-binary-tree-in-rust
        if target < &this.value {
            if let Some(left) = this.left.take() {
                this.left = Self::delete(left, target);
            }
            return Some(this);
        }

        if target > &this.value {
            if let Some(right) = this.right.take() {
                this.right = Self::delete(right, target);
            }
            return Some(this);
        }

        match (this.left.take(), this.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(right)) => {
                if let Some(mut rightmost) = left.rightmost_child() {
                    rightmost.left = Some(left);
                    rightmost.right = Some(right);
                    Some(rightmost)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            }
        }
    }

    //  Returns the rightmost child, unless the node itself is that child.
    fn rightmost_child(&mut self) -> Option<Box<Node<T>>> {
        match self.right.as_mut() {
            Some(right) => {
                if let Some(t) = right.rightmost_child() {
                    Some(t)
                } else {
                    let mut r = self.right.take();
                    if let Some(ref mut r) = r {
                        self.right = std::mem::replace(&mut r.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }
}

pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: PartialEq + PartialOrd + Copy> BinaryTree<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: None,
        }
    }

    pub fn pre_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        if let Some(root) = self.root.as_ref() {
            root.pre_order_traverse(visit_func);
        }
    }

    pub fn post_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        if let Some(root) = self.root.as_ref() {
            root.post_order_traverse(visit_func);
        }
    }

    pub fn in_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        if let Some(root) = self.root.as_ref() {
            root.in_order_traverse(visit_func);
        }
    }

    pub fn breadth_first_search(&self, value: T) -> bool {
        // for unordered trees
        if let Some(root) = self.root.as_ref() {
            return root.breadth_first_search(value);
        }
        return false;
    }

    pub fn compare(a: &BinaryTree<T>, b: &BinaryTree<T>) -> bool {
        match (a.root.as_ref(), b.root.as_ref()) {
            (None, None) => true,
            (Some(a), Some(b)) => Node::compare(a.as_ref(), b.as_ref()),
            _ => false,
        }
    }

    pub fn depth_first_search(&self, value: T) -> bool {
        // tree must be ordered. Left branch has smaller or equal values and Right has bigger values
        if let Some(root) = self.root.as_ref() {
            return root.depth_first_search(value);
        }
        return false;
    }

    pub fn insert_ordered(&mut self, value: T) {
        match self.root.as_mut() {
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
            Some(r) => { r.insert_ordered(value) }
        }
    }

    pub fn delete(&mut self, value: T) {
        if let Some(root) = self.root.take() {
            self.root = Node::delete(root, &value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_traverse() {
        let mut tree: Node<i32> = Node::new(1);
        // not ordered
        tree.node_insert(2, BranchSide::Left);
        tree.node_insert(3, BranchSide::Right);
        tree.left.as_mut().unwrap().node_insert(4, BranchSide::Left);
        tree.left.as_mut().unwrap().node_insert(5, BranchSide::Right);
        tree.right.as_mut().unwrap().node_insert(6, BranchSide::Left);
        tree.right.as_mut().unwrap().node_insert(7, BranchSide::Right);

        let mut pre_order_path: Vec<i32> = Vec::<i32>::new();
        let mut post_order_path: Vec<i32> = Vec::<i32>::new();
        let mut in_order_path: Vec<i32> = Vec::<i32>::new();

        tree.pre_order_traverse(&mut (|value: &i32| pre_order_path.push(*value)));
        tree.post_order_traverse(&mut (|value: &i32| post_order_path.push(*value)));
        tree.in_order_traverse(&mut (|value: &i32| in_order_path.push(*value)));

        assert_eq!(vec![1, 2, 4, 5, 3, 6, 7], pre_order_path);
        assert_eq!(vec![4, 5, 2, 6, 7, 3, 1], post_order_path);
        assert_eq!(vec![4, 2, 5, 1, 6, 3, 7], in_order_path);
    }

    #[test]
    fn node_bf_search() {
        let mut tree: Node<i32> = Node::new(1);
        // not ordered
        tree.node_insert(2, BranchSide::Left);
        tree.node_insert(3, BranchSide::Right);
        tree.left.as_mut().unwrap().node_insert(4, BranchSide::Left);
        tree.left.as_mut().unwrap().node_insert(5, BranchSide::Right);
        tree.right.as_mut().unwrap().node_insert(6, BranchSide::Left);
        tree.right.as_mut().unwrap().node_insert(7, BranchSide::Right);

        assert!(tree.breadth_first_search(1));
        assert!(tree.breadth_first_search(3));
        assert!(!tree.breadth_first_search(10));
    }

    #[test]
    fn df_search() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.insert_ordered(1);
        tree.insert_ordered(4);
        tree.insert_ordered(7);
        tree.insert_ordered(23);
        tree.insert_ordered(2);
        tree.insert_ordered(12);
        tree.insert_ordered(9);

        assert!(tree.depth_first_search(1));
        assert!(tree.depth_first_search(23));
        assert!(!tree.depth_first_search(10));
    }

    #[test]
    fn btree_ordered_insert() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.insert_ordered(10);
        tree.insert_ordered(2);
        tree.insert_ordered(3);
        tree.insert_ordered(2);
        tree.insert_ordered(12);
        tree.insert_ordered(14);
        tree.insert_ordered(11);

        let mut pre_order_path: Vec<i32> = Vec::<i32>::new();

        tree.pre_order_traverse(&mut (|value: &i32| pre_order_path.push(*value)));

        assert_eq!(vec![10, 2, 2, 3, 12, 11, 14], pre_order_path);
    }

    #[test]
    fn btree_comparison() {
        let mut tree1: BinaryTree<i32> = BinaryTree::new();
        tree1.insert_ordered(10);
        tree1.insert_ordered(2);
        tree1.insert_ordered(3);
        tree1.insert_ordered(2);
        tree1.insert_ordered(12);
        tree1.insert_ordered(14);
        tree1.insert_ordered(11);

        let mut tree2: BinaryTree<i32> = BinaryTree::new();
        tree2.insert_ordered(10);
        tree2.insert_ordered(2);
        tree2.insert_ordered(2);
        tree2.insert_ordered(3);
        tree2.insert_ordered(12);
        tree2.insert_ordered(11);
        tree2.insert_ordered(14);

        let mut tree3: BinaryTree<i32> = BinaryTree::new();
        tree3.insert_ordered(10);
        tree3.insert_ordered(2);
        tree3.insert_ordered(3);
        tree3.insert_ordered(2);
        tree3.insert_ordered(14);
        tree3.insert_ordered(12);
        tree3.insert_ordered(11);

        assert!(BinaryTree::compare(&tree1, &tree2));
        assert!(!BinaryTree::compare(&tree1, &tree3));
        assert!(!BinaryTree::compare(&tree3, &tree2));
    }

    #[test]
    fn btree_delete() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.insert_ordered(10);
        tree.insert_ordered(2);
        tree.insert_ordered(3);
        tree.insert_ordered(2);
        tree.insert_ordered(12);
        tree.insert_ordered(14);
        tree.insert_ordered(11);

        let mut pre_order_path: Vec<i32> = Vec::<i32>::new();
        tree.delete(10);
        tree.pre_order_traverse(&mut (|value: &i32| pre_order_path.push(*value)));
        assert_eq!(vec![3, 2, 2, 12, 11, 14], pre_order_path);

        let mut pre_order_path: Vec<i32> = Vec::<i32>::new();
        tree.delete(14);
        tree.pre_order_traverse(&mut (|value: &i32| pre_order_path.push(*value)));
        assert_eq!(vec![3, 2, 2, 12, 11], pre_order_path);

        let mut pre_order_path: Vec<i32> = Vec::<i32>::new();
        tree.delete(12);
        tree.pre_order_traverse(&mut (|value: &i32| pre_order_path.push(*value)));
        assert_eq!(vec![3, 2, 2, 11], pre_order_path);
    }
}
