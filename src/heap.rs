#![allow(dead_code)]

pub struct MaxHeap<T: PartialOrd> {
    // also called Priority Queue
    arr: Vec<T>,
}

impl<T: PartialOrd + Copy> MaxHeap<T> {
    pub fn new() -> MaxHeap<T> {
        MaxHeap { arr: Vec::<T>::new() }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn insert(&mut self, value: T) {
        self.arr.push(value);
        self.heapify_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        let r: T = self.arr[0];
        self.arr[0] = self.arr[self.len() - 1];
        self.arr.remove(self.len() - 1);
        self.heapify_down(0);
        return Some(r);
    }

    pub fn edit<F: Fn(&T) -> bool>(&mut self, f: F, new: T) {
        let mut idx: Option<usize> = None;
        let mut go_up: bool = false;
        for (i, e) in self.arr.iter_mut().enumerate() {
            if f(e) {
                if new > *e {
                    go_up = true;
                }
                *e = new;
                idx = Some(i);
                break;
            }
        }
        if idx.is_some() {
            if go_up {
                self.heapify_up(idx.unwrap());
            } else {
                self.heapify_down(idx.unwrap());
            }
        }
    }

    fn parent(index: usize) -> usize {
        return (index - 1) / 2;
    }

    fn left_child(index: usize) -> usize {
        return index * 2 + 1;
    }

    fn right_child(index: usize) -> usize {
        return index * 2 + 2;
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent_idx: usize = MaxHeap::<T>::parent(index);

        if self.arr[index] > self.arr[parent_idx] {
            let swap_aux: T = self.arr[index];
            self.arr[index] = self.arr[parent_idx];
            self.arr[parent_idx] = swap_aux;
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let left_idx: usize = MaxHeap::<T>::left_child(index);
        let right_idx: usize = MaxHeap::<T>::right_child(index);

        if index >= self.len() || left_idx >= self.len() {
            return;
        }
        if
            (right_idx >= self.len() || self.arr[left_idx] > self.arr[right_idx]) &&
            self.arr[left_idx] > self.arr[index]
        {
            let swap_aux: T = self.arr[index];
            self.arr[index] = self.arr[left_idx];
            self.arr[left_idx] = swap_aux;
            self.heapify_down(left_idx);
        } else if
            right_idx < self.len() &&
            self.arr[left_idx] < self.arr[right_idx] &&
            self.arr[right_idx] > self.arr[index]
        {
            let swap_aux: T = self.arr[index];
            self.arr[index] = self.arr[right_idx];
            self.arr[right_idx] = swap_aux;
            self.heapify_down(right_idx);
        }
    }
}

pub struct MinHeap<T: PartialOrd> {
    // also called Priority Queue
    arr: Vec<T>,
}

impl<T: PartialOrd + Copy> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { arr: Vec::<T>::new() }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn insert(&mut self, value: T) {
        self.arr.push(value);
        self.heapify_up(self.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        let r: T = self.arr[0];
        self.arr[0] = self.arr[self.len() - 1];
        self.arr.remove(self.len() - 1);
        self.heapify_down(0);
        return Some(r);
    }

    pub fn edit<F: Fn(&T) -> bool>(&mut self, f: F, new: T) {
        let mut idx: Option<usize> = None;
        let mut go_up: bool = false;
        for (i, e) in self.arr.iter_mut().enumerate() {
            if f(e) {
                if new < *e {
                    go_up = true;
                }
                *e = new;
                idx = Some(i);
                break;
            }
        }
        if idx.is_some() {
            if go_up {
                self.heapify_up(idx.unwrap());
            } else {
                self.heapify_down(idx.unwrap());
            }
        }
    }

    fn parent(index: usize) -> usize {
        return (index - 1) / 2;
    }

    fn left_child(index: usize) -> usize {
        return index * 2 + 1;
    }

    fn right_child(index: usize) -> usize {
        return index * 2 + 2;
    }

    fn heapify_up(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        let parent_idx: usize = MinHeap::<T>::parent(index);

        if self.arr[index] < self.arr[parent_idx] {
            let swap_aux: T = self.arr[index];
            self.arr[index] = self.arr[parent_idx];
            self.arr[parent_idx] = swap_aux;
            self.heapify_up(parent_idx);
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let left_idx: usize = MinHeap::<T>::left_child(index);
        let right_idx: usize = MinHeap::<T>::right_child(index);

        if index >= self.len() || left_idx >= self.len() {
            return;
        }
        if
            (right_idx >= self.len() || self.arr[left_idx] < self.arr[right_idx]) &&
            self.arr[left_idx] < self.arr[index]
        {
            let swap_aux: T = self.arr[index];
            self.arr[index] = self.arr[left_idx];
            self.arr[left_idx] = swap_aux;
            self.heapify_down(left_idx);
        } else if
            right_idx < self.len() &&
            self.arr[left_idx] > self.arr[right_idx] &&
            self.arr[right_idx] < self.arr[index]
        {
            let swap_aux: T = self.arr[index];
            self.arr[index] = self.arr[right_idx];
            self.arr[right_idx] = swap_aux;
            self.heapify_down(right_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_heap_insertion() {
        let mut heap: MaxHeap<i32> = MaxHeap::<i32>::new();
        heap.insert(300);
        heap.insert(30);
        heap.insert(230);
        heap.insert(40);
        heap.insert(50);
        heap.insert(700);
        heap.insert(912);

        assert_eq!(vec![912, 50, 700, 30, 40, 230, 300], heap.arr);
    }

    #[test]
    fn max_heap_pop() {
        let mut heap: MaxHeap<i32> = MaxHeap::<i32>::new();
        heap.insert(300);
        heap.insert(30);
        heap.insert(50);
        heap.insert(700);
        heap.insert(912);

        assert_eq!(Some(912), heap.pop());
        assert_eq!(Some(700), heap.pop());
        assert_eq!(Some(300), heap.pop());
        assert_eq!(Some(50), heap.pop());
        assert_eq!(Some(30), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn min_heap_insertion() {
        let mut heap: MinHeap<i32> = MinHeap::<i32>::new();
        heap.insert(300);
        heap.insert(30);
        heap.insert(230);
        heap.insert(40);
        heap.insert(50);
        heap.insert(700);
        heap.insert(912);

        assert_eq!(vec![30, 40, 230, 300, 50, 700, 912], heap.arr);
    }

    #[test]
    fn min_heap_pop() {
        let mut heap: MinHeap<i32> = MinHeap::<i32>::new();
        heap.insert(300);
        heap.insert(30);
        heap.insert(50);
        heap.insert(700);
        heap.insert(912);

        assert_eq!(Some(30), heap.pop());
        assert_eq!(Some(50), heap.pop());
        assert_eq!(Some(300), heap.pop());
        assert_eq!(Some(700), heap.pop());
        assert_eq!(Some(912), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn heap_edit() {
        let mut heap: MinHeap<i32> = MinHeap::<i32>::new();
        heap.insert(300);
        heap.insert(30);
        heap.insert(50);
        heap.insert(700);
        heap.insert(912);
        heap.edit(|e| *e == 300, 40);

        assert_eq!(Some(30), heap.pop());
        assert_eq!(Some(40), heap.pop());
        assert_eq!(Some(50), heap.pop());
        assert_eq!(Some(700), heap.pop());
        assert_eq!(Some(912), heap.pop());
        assert_eq!(None, heap.pop());
    }
}
