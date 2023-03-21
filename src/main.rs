#![allow(unused)]
use std::collections::{HashMap, HashSet};

fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
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

fn quick_sort<T: PartialOrd>(nums: &mut [T]) -> &[i32] {
    let length = nums.len();

    nums.swap(length - 1, length / 2);

    let left_part = todo!();
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

fn bubble_sort() {
    todo!();
}

fn insertion_sort() {
    todo!();
}

fn selection_sort() {
    todo!();
}

fn main() {
    let mut arr = vec![1, 2, 3, 4, 6, 7, 11, 14];
    let result = binary_search(&mut arr, 1).unwrap();
    println!("your number index is: {:?}", result);
}
