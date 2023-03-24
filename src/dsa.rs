#![allow(unused)]
use std::collections::{HashMap, HashSet};

fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

#[test]
fn test_contains_subarray() {
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut result = i32::MIN;
    let mut acc = 0;

    for el in nums {
        let partial_sum = acc + el;
        if el > partial_sum {
            acc = el;
        } else if acc <= partial_sum {
            acc += el;
        } else if acc > partial_sum {
            acc = partial_sum;
        }

        if acc > result {
            result = acc;
        }
    }
    result
}

#[test]
fn test_max_sub_array() {
    assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    assert_eq!(max_sub_array(vec![1]), 1);
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::<i32, usize>::new();
    for (index, &el) in nums.iter().enumerate() {
        let diff = target - el;
        if let Some(&x) = hash.get(&diff) {
            return vec![x as i32, index as i32];
        }
        hash.insert(el, index);
    }
    vec![]
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}

fn binary_search(nums: &mut [i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len();
    while low < high {
        let mid = (low + high) / 2;
        println!("{:?}, {:?}, {:?}", low, mid, high);
        if nums[mid] == target {
            return Some(mid);
        } else if target < nums[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    None
}

#[test]
fn test_binary_search() {
    assert_eq!(
        binary_search(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9),
        Some(8)
    );
    assert_eq!(
        binary_search(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1),
        Some(0)
    );
    assert_eq!(
        binary_search(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 15),
        None
    );
}

fn quick_sort<T: PartialOrd>(nums: &mut [T]) {
    fn partition<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) -> usize {
        todo!()
    }

    fn _quick_sort<T: PartialOrd>(nums: &mut [T], low: usize, high: usize) {
        if (low < high) {
            let pivot_index = partition(nums, low, high);
            _quick_sort(nums, low, pivot_index);
            _quick_sort(nums, pivot_index, high);
        }
    }

    _quick_sort(nums, 0, nums.len());
}

// #[test]
// fn test_quick_sort() {
//     assert_eq!()
// }

fn bubble_sort() {
    todo!();
}

fn insertion_sort() {
    todo!();
}

fn selection_sort<T: PartialOrd>(nums: &mut [T]) {
    fn find_smallest<T: PartialOrd>(nums: &mut [T]) {
        let mut small = &nums[0];

        for el in nums {
            if el < small {
                small = el;
            }
        }
    }
}

#[test]
fn test_selection_sort() {
    let arr1 = [1, 7, 4, 3, 9, 2, 0];
}
