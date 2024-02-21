#![allow(dead_code)]

use crate::errordsa;

// About the Drop trait
// Rust will automatically handle the dropping for the linkedList, however, as explained in the following link, it's extremely inefficient
// https://rust-unofficial.github.io/too-many-lists/first-drop.html
// Here there's an example of how we could implement a Drop trait for our linked list so that it's better

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: std::fmt::Display + PartialOrd> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::from("[");
        let mut node: &Node<T> = self.head.as_ref().unwrap();
        loop {
            result.push_str(node.value.to_string().as_str());
            match &node.next {
                Some(n) => {
                    result.push_str(", ");
                    node = n;
                }
                None => {
                    result.push_str("]");
                    break;
                }
            }
        }
        result
    }

    pub fn push_back(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(value)));
        } else {
            let mut last_node: &mut Node<T> = self.head.as_mut().unwrap();
            while last_node.next.is_some() {
                last_node = last_node.next.as_mut().unwrap();
            }
            last_node.next = Some(Box::new(Node::new(value)));
        }
        self.length += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let prev_head: Option<Box<Node<T>>> = match self.head.take() {
            Some(n) => Some(n),
            None => None,
        };
        let mut new_node: Node<T> = Node::new(value);
        new_node.next = prev_head;
        self.head = Some(Box::new(new_node));
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Result<&T, errordsa::Error> {
        let mut node: &Node<T> = self.head.as_ref().unwrap();
        let mut iter: usize = 0;

        if index >= self.length {
            return Err(errordsa::Error::IndexOutOfBound);
        }

        while iter < index {
            node = node.next.as_ref().unwrap();
            iter += 1;
        }

        Ok(&node.value)
    }

    pub fn remove(&mut self, index: usize) -> Result<(), errordsa::Error> {
        if index >= self.length {
            return Err(errordsa::Error::IndexOutOfBound);
        }

        if index == 0 {
            self.head = self.head.take().unwrap().next;
            return Ok(());
        }

        let mut prev_node: &mut Node<T> = self.head.as_mut().unwrap();
        let mut iter: usize = 0;

        while iter < index - 1 {
            prev_node = prev_node.next.as_mut().unwrap();
            iter += 1;
        }
        prev_node.next = prev_node.next.as_mut().unwrap().next.take();
        self.length -= 1;
        Ok(())
    }

    pub fn edit(&mut self, index: usize, value: T) -> Result<(), errordsa::Error> {
        let mut node: &mut Node<T> = self.head.as_mut().unwrap();
        let mut iter: usize = 0;

        if index >= self.length {
            return Err(errordsa::Error::IndexOutOfBound);
        }

        while iter < index {
            node = node.next.as_mut().unwrap();
            iter += 1;
        }

        node.value = value;

        Ok(())
    }

    pub fn insert_ordered_asc(&mut self, value: T) {
        // precondition: the list is ordered
        if self.length == 0 {
            self.push_front(value);
            return;
        }

        let mut node: &mut Node<T> = self.head.as_mut().unwrap();

        if node.value > value {
            self.push_front(value);
            return;
        }

        while node.next.is_some() && node.next.as_ref().unwrap().value < value {
            node = node.next.as_mut().unwrap();
        }

        let mut new_node: Node<T> = Node::new(value);
        new_node.next = node.next.take();
        node.next = Some(Box::new(new_node));
    }

    pub fn insert_ordered_desc(&mut self, value: T) {
        // precondition: the list is ordered
        if self.length == 0 {
            self.push_front(value);
            return;
        }

        let mut node: &mut Node<T> = self.head.as_mut().unwrap();

        if node.value < value {
            self.push_front(value);
            return;
        }

        while node.next.is_some() && node.next.as_ref().unwrap().value > value {
            node = node.next.as_mut().unwrap();
        }

        let mut new_node: Node<T> = Node::new(value);
        new_node.next = node.next.take();
        node.next = Some(Box::new(new_node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back_and_print() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_back(101);
        list.push_back(202);
        list.push_back(303);
        assert_eq!("[101, 202, 303]".to_owned(), list.to_string());
        assert_eq!(3, list.len());
    }

    #[test]
    fn get() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_back(101);
        list.push_back(202);
        list.push_back(303);

        assert_eq!(202, *list.get(1).unwrap());

        assert_eq!(errordsa::Error::IndexOutOfBound, list.get(3).unwrap_err());
    }

    #[test]
    fn edit() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_back(101);
        list.push_back(202);
        list.push_back(303);
        list.edit(0, 777).unwrap();
        list.edit(2, 777).unwrap();

        assert_eq!(777, *list.get(0).unwrap());
        assert_eq!(777, *list.get(2).unwrap());

        assert_eq!(errordsa::Error::IndexOutOfBound, list.get(3).unwrap_err());
    }

    #[test]
    fn push_front() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_front(101);
        list.push_front(202);
        list.push_front(303);

        assert_eq!(303, *list.get(0).unwrap())
    }

    #[test]
    fn remove() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_back(101);
        list.push_back(202);
        list.push_back(303);
        list.push_back(404);

        list.remove(0).unwrap();
        list.remove(2).unwrap();
        list.remove(0).unwrap();

        assert_eq!(303, *list.get(0).unwrap());
    }

    #[test]
    fn insert_ordered() {
        let mut list_asc: LinkedList<i32> = LinkedList::<i32>::new();
        list_asc.insert_ordered_asc(324);
        list_asc.insert_ordered_asc(12);
        list_asc.insert_ordered_asc(24);
        list_asc.insert_ordered_asc(400);

        assert_eq!("[12, 24, 324, 400]".to_owned(), list_asc.to_string());

        let mut list_desc: LinkedList<i32> = LinkedList::<i32>::new();
        list_desc.insert_ordered_desc(324);
        list_desc.insert_ordered_desc(12);
        list_desc.insert_ordered_desc(24);
        list_desc.insert_ordered_desc(400);

        assert_eq!("[400, 324, 24, 12]".to_owned(), list_desc.to_string());
    }
}
