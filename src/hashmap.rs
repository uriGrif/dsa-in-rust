#![allow(dead_code)]

// https://betterprogramming.pub/implementing-a-hashmap-in-rust-35d055b5ac2b
const MAX_SIZE: usize = 256;

fn my_hashing_function(key: &String) -> usize {
    // this is just a simple hashing function to understand the concept (of course, it's a terrible one and it will cause a lot of collisions)
    let mut result: usize = 0;
    for c in key.chars() {
        result += c as usize;
    }
    result
}

pub struct HashMap<T> {
    length: usize,
    arr: [Option<Box<KeyValue<T>>>; MAX_SIZE],
}

pub struct KeyValue<T> {
    key: String,
    value: T,
    next: Option<Box<KeyValue<T>>>,
}

impl<T: Copy> KeyValue<T> {
    pub fn new(key: String, value: T) -> KeyValue<T> {
        KeyValue {
            key,
            value,
            next: None,
        }
    }

    pub fn push_back(&mut self, key: String, value: T) {
        let mut last_node: &mut KeyValue<T> = self;
        while last_node.next.is_some() {
            if last_node.key.eq(&key) {
                // if the key already exists, its value gets updated instead of pushing it to the back
                // it could be handled someway else, like returning an error o not doing anything
                last_node.value = value;
                return;
            }
            last_node = last_node.next.as_mut().unwrap();
        }
        last_node.next = Some(Box::new(KeyValue::new(key, value)));
    }

    pub fn get(&self, key: String) -> Option<T> {
        let mut node: &KeyValue<T> = self;
        loop {
            if node.key.eq(&key) {
                return Some(node.value);
            }
            if node.next.is_some() {
                node = node.next.as_ref().unwrap();
            } else {
                break;
            }
        }
        None
    }

    pub fn delete(&mut self, key: String) -> Option<KeyValue<T>> {
        if self.key.eq(&key) {
            match self.next.as_mut() {
                Some(n) => {
                    return Some(KeyValue {
                        key: n.key.clone(),
                        value: n.value,
                        next: n.next.take(),
                    });
                }
                None => {
                    return None;
                }
            }
        }
        let mut prev_node: &mut KeyValue<T> = self;
        while prev_node.next.is_some() && !prev_node.next.as_ref().unwrap().key.eq(&key) {
            prev_node = prev_node.next.as_mut().unwrap();
        }
        prev_node.next = prev_node.next.as_mut().unwrap().next.take();
        return None;
    }

    pub fn edit(&mut self, key: String, value: T) {
        let mut node: &mut KeyValue<T> = self;
        loop {
            if node.key.eq(&key) {
                node.value = value;
                break;
            }
            if node.next.is_some() {
                node = node.next.as_mut().unwrap();
            } else {
                break;
            }
        }
    }
}

impl<T: Copy> HashMap<T> {
    const INIT: Option<Box<KeyValue<T>>> = None;

    pub fn new() -> HashMap<T> {
        HashMap {
            length: 0,
            arr: [Self::INIT; MAX_SIZE],
        }
    }

    pub fn add(&mut self, key: String, value: T) {
        let index: usize = my_hashing_function(&key) % MAX_SIZE;

        match self.arr[index].as_mut() {
            Some(kv) => {
                kv.push_back(key, value);
            }
            None => {
                self.arr[index] = Some(Box::new(KeyValue::new(key, value)));
            }
        }
    }

    pub fn get(&self, key: String) -> Option<T> {
        let index: usize = my_hashing_function(&key) % MAX_SIZE;

        match &self.arr[index] {
            Some(kv) => { kv.get(key) }
            None => { None }
        }
    }

    pub fn remove(&mut self, key: String) {
        let index: usize = my_hashing_function(&key) % MAX_SIZE;

        if self.arr[index].is_some() {
            match self.arr[index].as_mut().unwrap().delete(key) {
                None => {}
                Some(n) => {
                    self.arr[index] = Some(Box::new(n));
                }
            }
        }
    }

    pub fn edit(&mut self, key: String, value: T) {
        let index: usize = my_hashing_function(&key) % MAX_SIZE;
        match self.arr[index].as_mut() {
            None => {}
            Some(n) => {
                n.edit(key, value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get() {
        let mut hm: HashMap<i32> = HashMap::<i32>::new();
        hm.add("uriel".to_string(), 10);
        hm.add("leiru".to_string(), 11); // should be a collision, it is added to the linked list
        hm.add("uri".to_string(), 3);

        assert_eq!(Some(10), hm.get("uriel".to_string()));
        assert_eq!(Some(11), hm.get("leiru".to_string()));
        assert_eq!(Some(3), hm.get("uri".to_string()));
        assert_eq!(None, hm.get("efwe".to_string()))
    }

    #[test]
    fn delete() {
        let mut hm: HashMap<i32> = HashMap::<i32>::new();
        hm.add("uriel".to_string(), 10);
        hm.add("leiru".to_string(), 11); // should be a collision, it is added to the linked list
        hm.add("ulire".to_string(), 12);
        hm.add("uri".to_string(), 3);

        hm.remove("uriel".to_string());

        hm.add("uriel".to_string(), 100);

        assert_eq!(Some(100), hm.get("uriel".to_string()));
        assert_eq!(Some(11), hm.get("leiru".to_string()));
        assert_eq!(Some(12), hm.get("ulire".to_string()));
        assert_eq!(Some(3), hm.get("uri".to_string()));
    }

    #[test]
    fn edit() {
        let mut hm: HashMap<i32> = HashMap::<i32>::new();
        hm.add("uriel".to_string(), 10);
        hm.add("leiru".to_string(), 11); // should be a collision, it is added to the linked list
        hm.add("ulire".to_string(), 12);
        hm.add("uri".to_string(), 3);

        hm.edit("uriel".to_string(), 100);
        hm.edit("ulire".to_string(), 112);

        assert_eq!(Some(100), hm.get("uriel".to_string()));
        assert_eq!(Some(11), hm.get("leiru".to_string()));
        assert_eq!(Some(112), hm.get("ulire".to_string()));
        assert_eq!(Some(3), hm.get("uri".to_string()));
    }
}
