#![allow(unused)]
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

fn contains_duplicate<T: Eq + Hash>(nums: &[T]) -> bool {
    nums.len() != HashSet::<&T>::from_iter(nums).len()
}

#[test]
fn test_contains_duplicate() {
    assert_eq!(contains_duplicate(&[1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(&[1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}

fn max_sub_array(nums: &[i32]) -> Option<i32> {
    let mut result: Option<i32> = None;
    let mut acc = 0;

    for &el in nums {
        let partial_sum = acc + el;
        if el > partial_sum {
            acc = el;
        } else if acc <= partial_sum {
            acc += el;
        } else if acc > partial_sum {
            acc = partial_sum;
        }

        match result {
            Some(valid_result) => {
                if acc > valid_result {
                    result = Some(acc);
                }
            }
            None => result = Some(acc),
        }
    }
    result
}

#[test]
fn test_max_sub_array() {
    assert_eq!(max_sub_array(&[5, 4, -1, 7, 8]), Some(23));
    assert_eq!(max_sub_array(&[1]), Some(1));
    assert_eq!(max_sub_array(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
    assert_eq!(max_sub_array(&[]), None);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Option<[i32; 2]> {
    let mut hash = HashMap::<i32, usize>::new();
    for (index, &el) in nums.iter().enumerate() {
        let diff = target - el;
        if let Some(&x) = hash.get(&diff) {
            return Some([x as i32, index as i32]);
        }
        hash.insert(el, index);
    }
    None
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9).unwrap(), [0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6).unwrap(), [1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6).unwrap(), [0, 1]);
}

fn binary_search<T: PartialOrd>(nums: &mut [T], target: T) -> Option<usize> {
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

fn bubble_sort<T: PartialOrd>(nums: &mut [T]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if j > 0 && nums[j] < nums[j - 1] {
                nums.swap(j, j - 1);
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    let mut arr = [1, 7, 4, 3, 9, 2, 0];
    bubble_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 7, 9]);
    let mut arr = [1];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1]);
    let mut arr: [i32; 0] = [];
    bubble_sort(&mut arr);
    assert_eq!(arr, []);
}

fn insertion_sort<T: PartialOrd>(nums: &mut [T]) {
    fn swap_until_smallest<T: PartialOrd>(nums: &mut [T]) {
        for i in (0..nums.len()).rev() {
            if i > 0 && nums[i] < nums[i - 1] {
                nums.swap(i, i - 1);
            }
        }
    }

    for i in 0..nums.len() {
        swap_until_smallest(&mut nums[0..=i]);
    }
}

#[test]
fn test_insertion_sort() {
    let mut arr = [1, 7, 4, 3, 9, 2, 0];
    insertion_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 7, 9]);
    let mut arr = [1];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1]);
    let mut arr: [i32; 0] = [];
    insertion_sort(&mut arr);
    assert_eq!(arr, []);
}

fn selection_sort<T: PartialOrd>(nums: &mut [T]) {
    fn find_smallest<T: PartialOrd>(nums: &[T]) -> usize {
        let mut small = &nums[0];
        let mut small_index = 0;

        for (index, el) in nums.into_iter().enumerate() {
            if el < small {
                small = el;
                small_index = index;
            }
        }

        small_index
    }

    for i in 0..nums.len() {
        let small_index = find_smallest(&nums[i..]);
        nums.swap(i, i + small_index);
    }
}

#[test]
fn test_selection_sort() {
    let mut arr = [1, 7, 4, 3, 9, 2, 0];
    selection_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 7, 9]);
    let mut arr = [1];
    selection_sort(&mut arr);
    assert_eq!(arr, [1]);
    let mut arr: [i32; 0] = [];
    selection_sort(&mut arr);
    assert_eq!(arr, []);
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

#[test]
fn test_quick_sort() {}
