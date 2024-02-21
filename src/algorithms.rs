#![allow(dead_code)]

// searching algorithms

pub fn linear_search<T: PartialEq>(list: &Vec<T>, element: T) -> bool {
    // Complexity: O(n)
    let mut index: usize = 0;
    while index < list.len() {
        if list[index] == element {
            return true;
        }
        index += 1;
    }
    return false;
}

pub fn binary_search<T: PartialEq + PartialOrd>(list: &Vec<T>, element: T) -> bool {
    // list must be in ascending order
    // Complexity: O(log(n))
    let mut lo: usize = 0;
    let mut hi: usize = list.len();
    let mut middle: usize;

    while lo < hi {
        middle = (lo + hi) / 2;
        if list[middle] == element {
            return true;
        } else if list[middle] < element {
            lo = middle + 1;
        } else {
            hi = middle;
        }
    }
    return false;
}

// Sorting algorithms

pub fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>, ascending: bool) {
    // Complexity: O(n^2)

    let mut swap_aux: T;

    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if ascending {
                if list[j] > list[j + 1] {
                    swap_aux = list[j];
                    list[j] = list[j + 1];
                    list[j + 1] = swap_aux;
                }
            } else {
                if list[j] < list[j + 1] {
                    swap_aux = list[j];
                    list[j] = list[j + 1];
                    list[j + 1] = swap_aux;
                }
            }
        }
    }
}

pub fn quick_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    // Complexity: O(n*log(n))
    // sorts in ascending order
    qs(list, 0, list.len());
}

fn qs<T: PartialOrd + Copy>(list: &mut Vec<T>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let pivot: usize = qs_partition(list, lo, hi);

    qs(list, lo, pivot);
    qs(list, pivot + 1, hi);
}

fn qs_partition<T: PartialOrd + Copy>(list: &mut Vec<T>, lo: usize, hi: usize) -> usize {
    let pivot: T = list[hi - 1];

    let mut idx: i32 = (lo as i32) - 1;

    let mut swap_aux: T;

    for i in lo..hi - 1 {
        if list[i] <= pivot {
            idx += 1;
            swap_aux = list[i];
            list[i] = list[idx as usize];
            list[idx as usize] = swap_aux;
        }
    }

    idx += 1;
    swap_aux = list[hi - 1];
    list[hi - 1] = list[idx as usize];
    list[idx as usize] = swap_aux;

    return idx as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_search_test() {
        let list1: Vec<i32> = vec![1, 2, 3, 4];
        let list2: Vec<i32> = vec![];

        assert!(linear_search(&list1, 3));
        assert!(!linear_search(&list1, 5));
        assert!(!linear_search(&list2, 1));
    }

    #[test]
    fn binary_search_test() {
        let list1: Vec<i32> = (1..100).collect();
        let list2: Vec<i32> = vec![542];
        let list3: Vec<i32> = vec![];

        assert!(binary_search(&list1, 56));
        assert!(!binary_search(&list1, 1001));
        assert!(binary_search(&list2, 542));
        assert!(!binary_search(&list3, 1));
    }

    #[test]
    fn bubble_sort_test() {
        let mut list1: Vec<i32> = vec![4, 1, 7, 1, 8, 2, 83, 12, 123, 23];
        let mut list2: Vec<i32> = vec![4, 1, 7, 1, 8, 2, 83, 12, 123, 23];
        let mut list3: Vec<i32> = vec![];

        bubble_sort(&mut list1, true);
        bubble_sort(&mut list2, false);
        bubble_sort(&mut list3, true);

        assert_eq!(vec![1, 1, 2, 4, 7, 8, 12, 23, 83, 123], list1);
        assert_eq!(vec![123, 83, 23, 12, 8, 7, 4, 2, 1, 1], list2);
        assert_eq!(Vec::<i32>::new(), list3);
    }

    #[test]
    fn quick_sort_test() {
        let mut list1: Vec<i32> = vec![4, 1, 7, 1, 8, 2, 83, 12, 123, 23];

        quick_sort(&mut list1);

        assert_eq!(vec![1, 1, 2, 4, 7, 8, 12, 23, 83, 123], list1);
    }
}
