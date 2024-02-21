#![allow(dead_code)]

pub struct RingBuffer<T: Copy> {
    buffer: Vec<Option<T>>, // TODO: learn about memory allocation in rust and do it on my own
    start: usize,
    length: usize,
    capacity: usize,
}

impl<T: Copy> RingBuffer<T> {
    pub fn new(capacity: usize) -> RingBuffer<T> {
        let mut vec = Vec::<Option<T>>::with_capacity(capacity);
        for _ in 0..capacity {
            vec.push(None);
        }
        RingBuffer {
            buffer: vec,
            start: 0,
            length: 0,
            capacity,
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.length == self.capacity {
            panic!("RingBuffer has overflowed its capacity"); // actually it should resize de buffer
        }
        self.buffer[(self.start + self.length) % self.capacity] = Some(value);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.start += 1;
        self.length -= 1;
        return self.buffer[(self.start - 1) % self.capacity]; // it could be overwritten with None, but it's not necessary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut rf: RingBuffer<i32> = RingBuffer::new(5);

        rf.push_back(0);
        rf.push_back(1);
        rf.push_back(2);
        rf.push_back(3);

        assert_eq!(0, rf.pop_front().unwrap());
        assert_eq!(1, rf.pop_front().unwrap());

        rf.push_back(4);
        rf.push_back(5);
        rf.push_back(6);

        assert_eq!(2, rf.pop_front().unwrap());
        assert_eq!(3, rf.pop_front().unwrap());
        assert_eq!(4, rf.pop_front().unwrap());
        assert_eq!(5, rf.pop_front().unwrap());
        assert_eq!(6, rf.pop_front().unwrap());
    }
}
