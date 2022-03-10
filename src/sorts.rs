use std::vec;

use crate::solution::Solution;

impl Solution {
    pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        for i in 0..nums.len() - 1 {
            let mut swiched_flag = false;
            for j in 0..nums.len() - i - 1 {
                if nums[j] > nums[j + 1] {
                    // let tmp = nums[j];
                    // nums[j] = nums[j + 1];
                    // nums[j + 1] = tmp;
                    nums.swap(j, j + 1);
                    swiched_flag = true;
                }
            }
            if !swiched_flag {
                break;
            }
        }

        nums
    }

    pub fn insert_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }
        for i in 1..nums.len() {
            let current = nums[i];
            let mut j = (i - 1) as i32;
            while j >= 0 {
                if nums[j as usize] > current {
                    nums[(j + 1) as usize] = nums[j as usize];
                } else {
                    break;
                }
                j = j - 1;
            }
            nums[(j + 1) as usize] = current;
            println!("{:?}\n", &nums);
        }
        nums
    }

    pub fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        for i in 0..nums.len() - 1 {
            let mut min_idx = i;
            for j in i + 1..nums.len() {
                if nums[j] < nums[min_idx] {
                    min_idx = j;
                }
            }

            if i != min_idx {
                nums.swap(i, min_idx);
            }
            println!("{:?}\n", &nums);
        }
        nums
    }

    pub fn heap_sort(nums: &mut Vec<i32>) {
        build_heap(nums);
        for i in (0..nums.len()).rev() {
            nums.swap(0, i);
            heapify(nums, 0, i);
            println!("{:?}", nums);
        }
    }

    pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let n = nums.len() - 1;
        merge_sort_recursion(&mut nums, 0, n);
        nums
    }

    pub fn quick_sort(mut nums: &mut Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums.to_vec();
        }
        let len = nums.len();
        quick_sort_recursion(&mut nums, 0, len - 1);
        nums.to_vec()
    }
}
fn quick_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let pivot = partition(nums, left, right);
    if pivot != 0 {
        quick_sort_recursion(nums, left, pivot - 1);
    }
    quick_sort_recursion(nums, pivot + 1, right);
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let povit = right;
    let mut i = left;
    for j in left..right {
        if nums[j] < nums[povit] {
            nums.swap(i, j);
            i += 1;
        }
    }
    nums.swap(i, right);
    println!("{:?}", &nums);
    i
}

fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let middle = left + (right - left) / 2;

    merge_sort_recursion(nums, left, middle);
    merge_sort_recursion(nums, middle + 1, right);

    merge(nums, left, middle, right);
}

fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let mut i = left;
    let mut j = middle + 1;
    let mut k = left;
    let mut tmp = vec![];

    while k <= right {
        if i > middle {
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        } else if j > right {
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else if nums[i] < nums[j] {
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else {
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        }
    }

    for i in 0..=(right - left) {
        nums[left + i] = tmp[i];
    }

    println!("{:?}", nums);
}

fn build_heap(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in (0..len / 2).rev() {
        heapify(nums, i, len)
    }
}
fn heapify(nums: &mut Vec<i32>, idx: usize, len: usize) {
    let mut idx = idx;
    loop {
        let mut max_pos = idx;

        let left_child = 2 * idx + 1;
        if left_child < len && nums[idx] < nums[left_child] {
            max_pos = left_child;
        }
        let right_child = 2 * idx + 2;
        if right_child < len && nums[max_pos] < nums[right_child] {
            max_pos = right_child;
        }
        if max_pos == idx {
            break;
        }
        nums.swap(idx, max_pos);
        idx = max_pos;
    }
}
