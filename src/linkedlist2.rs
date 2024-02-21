#![allow(dead_code)]

use crate::errordsa;

#[derive(Debug)]
pub struct LinkedList<T> {
    pub value: Option<T>,
    pub next: Option<Box<LinkedList<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            value: None,
            next: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut last_node: &mut LinkedList<T> = self;
        while last_node.next.is_some() {
            last_node = last_node.next.as_mut().unwrap();
        }
        if last_node.value.is_some() {
            last_node.next = Some(Box::new(LinkedList { value: Some(value), next: None }));
        } else {
            last_node.value = Some(value);
        }
    }

    pub fn push_front(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
            return;
        }
        let prev_head: LinkedList<T> = LinkedList {
            value: self.value.take(),
            next: self.next.take(),
        };
        *self = LinkedList { value: Some(value), next: Some(Box::new(prev_head)) };
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::from("[");
        let mut current_node: &LinkedList<T> = self;
        loop {
            result.push_str(current_node.value.as_ref().unwrap().to_string().as_str());
            match &current_node.next {
                Some(n) => {
                    result.push_str(", ");
                    current_node = n;
                }
                None => {
                    result.push_str("]");
                    break;
                }
            }
        }
        result
    }

    pub fn get(&self, index: usize) -> Result<&T, errordsa::Error> {
        let mut i: usize = 0;
        let mut current_node: &LinkedList<T> = self;
        while i < index {
            match current_node.next.as_ref() {
                Some(n) => {
                    current_node = n;
                }
                None => {
                    return Err(errordsa::Error::IndexOutOfBound);
                }
            }
            i += 1;
        }
        if current_node.value.is_none() {
            return Err(errordsa::Error::IndexOutOfBound);
        }
        Ok(current_node.value.as_ref().unwrap())
    }

    pub fn remove(&mut self, index: usize) -> Result<(), errordsa::Error> {
        if self.value.is_none() {
            return Err(errordsa::Error::IndexOutOfBound);
        }

        let mut i: usize = 0;
        let mut prev_node: &mut LinkedList<T> = self;
        while i < index {
            match prev_node.next.as_mut() {
                Some(n) => {
                    prev_node = n;
                }
                None => {
                    return Err(errordsa::Error::IndexOutOfBound);
                }
            }
            i += 1;
        }
        match prev_node.next.as_mut() {
            Some(n) => {
                if index == 0 {
                    prev_node.value = n.value.take();
                }
                prev_node.next = n.next.take();
            }
            None => {
                prev_node.next = None;
                if index == 0 {
                    prev_node.value = None;
                }
            }
        }
        Ok(())
    }

    pub fn edit(&mut self, index: usize, value: T) -> Result<(), errordsa::Error> {
        if self.value.is_none() {
            return Err(errordsa::Error::IndexOutOfBound);
        }

        let mut i: usize = 0;
        let mut current_node: &mut LinkedList<T> = self;
        while i < index {
            match current_node.next.as_mut() {
                Some(n) => {
                    current_node = n;
                }
                None => {
                    return Err(errordsa::Error::IndexOutOfBound);
                }
            }
            i += 1;
        }
        current_node.value = Some(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_print() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();
        list.push_back(101);
        list.push_back(202);
        list.push_back(303);
        list.push_front(1);
        list.push_front(2);
        assert_eq!("[2, 1, 101, 202, 303]".to_owned(), list.to_string());
    }

    #[test]
    fn get() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();

        assert_eq!(errordsa::Error::IndexOutOfBound, list.get(0).unwrap_err());

        list.push_back(101);
        list.push_back(202);
        list.push_back(303);

        assert_eq!(202, *list.get(1).unwrap());

        assert_eq!(errordsa::Error::IndexOutOfBound, list.get(3).unwrap_err());
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
        list.remove(1).unwrap();

        assert_eq!(202, *list.get(0).unwrap());
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
}
