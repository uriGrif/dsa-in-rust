#![allow(dead_code)]

use std::{ collections::VecDeque, ops::{ Deref, DerefMut } };

struct Tree<T> {
    value: T,
    children: Vec<Box<Tree<T>>>,
}

impl<T: PartialEq> Tree<T> {
    fn new(value: T) -> Tree<T> {
        Tree {
            value,
            children: vec![],
        }
    }

    fn add_child(&mut self, value: T) {
        self.children.push(Box::new(Tree::new(value)));
    }

    fn get_child(&mut self, index: usize) -> &mut Tree<T> {
        return self.children[index].deref_mut();
    }

    fn pre_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        // these traversals are basically an example of Depth First Search
        visit_func(&self.value);
        for c in &self.children {
            c.pre_order_traverse(visit_func);
        }
    }

    fn post_order_traverse<F: FnMut(&T)>(&self, visit_func: &mut F) {
        for c in &self.children {
            c.post_order_traverse(visit_func);
        }
        visit_func(&self.value);
    }

    pub fn breadth_first_search(&self, value: T) -> bool {
        let mut queue: VecDeque<&Tree<T>> = VecDeque::from([self]);

        while queue.len() > 0 {
            let node: &Tree<T> = queue.pop_front().unwrap();
            if node.value == value {
                return true;
            }
            for c in &node.children {
                queue.push_back(c.deref());
            }
        }
        return false;
    }

    fn compare(a: &Tree<T>, b: &Tree<T>) -> bool {
        if a.value != a.value || a.children.len() != b.children.len() {
            return false;
        } else {
            for i in 0..a.children.len() {
                if !Tree::compare(a.children[i].deref(), b.children[i].deref()) {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverse() {
        let mut pre_order_path: Vec<i32> = Vec::<i32>::new();
        let mut post_order_path: Vec<i32> = Vec::<i32>::new();

        let mut tree: Tree<i32> = Tree::new(1);

        tree.add_child(2);
        tree.add_child(3);
        tree.get_child(0).add_child(4);
        tree.get_child(0).add_child(5);
        tree.get_child(1).add_child(6);
        tree.get_child(1).add_child(7);

        tree.pre_order_traverse(&mut (|value: &i32| pre_order_path.push(*value)));
        tree.post_order_traverse(&mut (|value: &i32| post_order_path.push(*value)));

        assert_eq!(vec![1, 2, 4, 5, 3, 6, 7], pre_order_path);
        assert_eq!(vec![4, 5, 2, 6, 7, 3, 1], post_order_path);
    }

    #[test]
    fn bf_search() {
        let mut tree: Tree<i32> = Tree::new(1);

        tree.add_child(2);
        tree.add_child(3);
        tree.get_child(0).add_child(4);
        tree.get_child(0).add_child(5);
        tree.get_child(1).add_child(6);
        tree.get_child(1).add_child(7);

        assert!(tree.breadth_first_search(2));
        assert!(tree.breadth_first_search(5));
        assert!(tree.breadth_first_search(7));
        assert!(!tree.breadth_first_search(9));
    }

    #[test]
    fn tree_comparison() {
        let mut tree1: Tree<i32> = Tree::new(1);

        tree1.add_child(2);
        tree1.add_child(3);
        tree1.get_child(0).add_child(4);
        tree1.get_child(0).add_child(5);
        tree1.get_child(1).add_child(6);
        tree1.get_child(1).add_child(7);

        let mut tree2: Tree<i32> = Tree::new(1);

        tree2.add_child(2);
        tree2.add_child(3);
        tree2.get_child(0).add_child(4);
        tree2.get_child(0).add_child(5);
        tree2.get_child(1).add_child(6);
        tree2.get_child(1).add_child(7);

        let mut tree3: Tree<i32> = Tree::new(1);

        tree3.add_child(2);
        tree3.add_child(3);
        tree3.get_child(0).add_child(4);
        tree3.get_child(0).add_child(5);
        tree3.get_child(0).add_child(6);
        tree3.get_child(1).add_child(7);

        assert!(Tree::compare(&tree1, &tree2));
        assert!(!Tree::compare(&tree1, &tree3));
        assert!(!Tree::compare(&tree3, &tree2));
    }
}
