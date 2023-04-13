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
    fn partition<T: PartialOrd>(nums: &mut [T]) -> usize {
        let pivot_index = nums.len() - 1;
        // I like choosing the middle element of the array as the pivot in case the array is already sorted because this ensures optimal pivot selection.
        // Swap the middle element of the array with the last element just to make things easier (you always know the pivot will be the last element)
        nums.swap(pivot_index, nums.len() / 2);
        // Initialize the index of the elements which are LESS than the pivot, we will call this the "mid-point" of the array
        let mut numbers_lower_than_pivot = 0;

        for current_index in 0..(nums.len() - 1) {
            // Borrow the pivot's value inside the "for loop" to avoid borrowing issues
            let pivot = &nums[pivot_index];

            // If the current element is smaller than the pivot, then increase the "mid-point" of the array, and place this element at this newly allocated position which is still less than the pivot
            if &nums[current_index] < pivot {
                nums.swap(numbers_lower_than_pivot, current_index);
                numbers_lower_than_pivot += 1;
            }
        }

        // Finally swap the pivot back to the mid-point where it belongs
        nums.swap(numbers_lower_than_pivot, pivot_index);
        // Return the mid-point index
        numbers_lower_than_pivot
    }

    fn _quick_sort<T: PartialOrd>(nums: &mut [T]) {
        if (nums.len() > 1) {
            let pivot_index = partition(nums);
            _quick_sort(&mut nums[0..pivot_index]);
            _quick_sort(&mut nums[pivot_index..]);
        }
    }

    _quick_sort(nums);
}

#[test]
fn test_quick_sort() {
    let mut arr = [1, 7, 4, 9, 0, 2, 3];
    quick_sort(&mut arr);
    assert_eq!(arr, [0, 1, 2, 3, 4, 7, 9]);
    let mut arr = [1];
    quick_sort(&mut arr);
    assert_eq!(arr, [1]);
    let mut arr: [i32; 0] = [];
    quick_sort(&mut arr);
    assert_eq!(arr, []);
}

// fn merge_sort<T: PartialOrd>(mut nums: Vec<T>) -> Vec<T> {
//     fn merge<T: PartialOrd>(mut first_half: Vec<T>, mut second_half: Vec<T>) -> Vec<T> {
//         let mut result: Vec<T> = vec![];

//         while !first_half.is_empty() && !second_half.is_empty() {
//             if first_half.first() > second_half.first() {
//                 let el = first_half.remove(0);
//                 result.push(el);
//             } else {
//                 let el = second_half.remove(0);
//                 result.push(el);
//             }
//         }

//         while !first_half.is_empty() {
//             result.push(first_half.remove(0));
//         }

//         while !second_half.is_empty() {
//             result.push(first_half.remove(0));
//         }

//         result
//     }

//     // fn _merge_sort<T>(nums: Vec<T>) -> Vec<T> {
//     //     if (nums.len() > 1) {
//     //         let mid = nums.len().clone() / 2;
//     //         let arr1 = _merge_sort(&nums[0..mid]);
//     //         let arr2 = _merge_sort(&nums[mid..]);
//     //         merge(arr1, arr2);
//     //         todo!();
//     //     }
//     // }
//     todo!();
// }

// #[test]
// fn test_merge_sort() {
//     let mut arr = [1, 7, 4, 9, 0, 2, 3];
//     merge_sort(&mut arr);
//     assert_eq!(arr, [0, 1, 2, 3, 4, 7, 9]);
//     let mut arr = [1];
//     merge_ssort(&mut arr);
//     assert_eq!(arr, [1]);
//     let mut arr: [i32; 0] = [];
//     merge_sort(&mut arr);
//     asert_eq!(arr, []);
// }

fn merge<T: PartialOrd + Debug>(mut first_half: Vec<T>, mut second_half: Vec<T>) -> Vec<T> {
    let mut result: Vec<T> = vec![];

    while !first_half.is_empty() && !second_half.is_empty() {
        if first_half.first() < second_half.first() {
            let el = first_half.remove(0);
            result.push(el);
        } else {
            let el = second_half.remove(0);
            result.push(el);
        }
    }

    while !first_half.is_empty() {
        result.push(first_half.remove(0));
    }

    while !second_half.is_empty() {
        result.push(second_half.remove(0));
    }

    result
}

#[test]
fn test_merge() {
    assert_eq!(merge(vec![1, 2, 3], vec![2, 5, 6]), vec![1, 2, 2, 3, 5, 6]);
    assert_eq!(merge(vec![1], vec![]), vec![1]);
    assert_eq!(merge(vec![], vec![1]), vec![1]);
}

fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut first_head = 0;
    let mut second_head = 0;

    for i in 0..(m + n) {
        if nums1[first_head] < nums2[second_head] {
            first_head += 1;
        } else {
            second_head += 1;
        }
    }
}

// fn test_merge2() {
//     let vec1 = vec![];

//     assert_eq!(merge2(vec![1, 2, 3], vec![2, 5, 6]), vec![1, 2, 2, 3, 5, 6]);
//     assert_eq!(merge2(vec![1], vec![]), vec![1]);
//     assert_eq!(merge2(vec![], vec![1]), vec![1]);
// }
