mod binary_search;
mod climb_r;
mod heap_r;
mod linked_list;
mod n_queens;
mod solution;
mod sorts;

use crate::solution::Solution;
use std::{
    collections::{HashMap, LinkedList, VecDeque},
    vec,
};

use linked_list::ListNode;

use crate::heap_r::{build_heap_down_up, insert};

fn main() {
    // let mut v = vec![32, 3, 9, 27, 11, 22, 13, 93, 112, 2, 8];
    // Solution::move_zeros(&mut v);

    // println!("remove dumplicate: \n");
    // Solution::remove_dumplications(&mut v);
    // let r = Solution::max_sliding_window(&mut v, 3);
    // let r = Solution::two_sum(&mut v, 5);

    // let v = v;
    // let head = linked_list::to_list(v);
    // let r = Solution::reverse_list(head);
    // let r = Solution::middle_list(head);
    // println!("middle list");
    // let v2 = vec![3, 9, 3, 2, 7];
    // println!("{:?}", &v2);
    // let head2 = linked_list::to_list(v2);
    // println!("merge two list");
    // let r = Solution::merge_two_list(head, head2);
    // let r = Solution::remove_nth_from_end(head, 4);

    // for i in (1..5).rev() {
    // println!("{}", i);
    // }
    // build_heap_down_up(&mut v);
    // println!("{:?}", v);
    // insert(&mut v, 11);
    // println!("{:?}", v);
    // let r = Solution::climb_stairs(10);
    // println!("clime stairs : {}", r);
    // println!("8 皇后");
    // let r = Solution::solve_n_queues(6);
    // println!("{:?}", r);
    // let r = Solution::solve_n_queues(12);
    // println!("{:?}", r);

    // ========sorts =======
    // let r = Solution::bubble_sort(v);
    // println!("bubble sorted {:?}", r);
    // let r = Solution::insert_sort(v);
    // println!("insert sort {:?}", r);
    // let r = Solution::selection_sort(v);
    // println!("select sort = {:?}", r);
    let mut v = vec![11, 9, 7, 3, 6, 12];
    // Solution::heap_sort(&mut v);
    // println!("heap sort {:?}", v);

    let r = Solution::merge_sort(v);
    println!("merge sort: {:?}", r);

    // println!("binary search ");
    // let r = Solution::selection_sort(&v);
    // println!("select sort = {:?}", r);
    // let nums = vec![0, 2, 3, 5, 9, 13, 22, 39];
    // let r = Solution::b_search(nums, 9);
    // println!("find 9 in position{}", r);
}

fn push(deque: &mut VecDeque<i32>, n: i32) {
    while !deque.is_empty() && *deque.back().unwrap() < n {
        deque.pop_back();
    }

    deque.push_back(n);
}

fn pop(deque: &mut VecDeque<i32>, n: i32) {
    if !deque.is_empty() && *deque.front().unwrap() == n {
        deque.pop_front();
    }
}

fn max(deque: &VecDeque<i32>) -> i32 {
    *deque.front().unwrap()
}

impl Solution {
    pub fn move_zeros(nums: &mut Vec<i32>) {
        let mut _i = 0;

        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[_i] = nums[j];
                _i += 1;
            }
        }

        for k in _i..nums.len() {
            nums[k] = 0;
        }
    }

    pub fn remove_dumplications(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut i = 0;

        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                if j - i > 1 {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
        }
        (i + 1) as i32
    }

    pub fn max_sliding_window(nums: &mut Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 0 || k == 1 {
            return nums.to_vec();
        }
        let mut res: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();

        for i in 0..nums.len() {
            push(&mut deque, nums[i]);

            if (i as i32) > k - 1 {
                pop(&mut deque, nums[i - k as usize]);
                res.push(max(&deque))
            } else if (i as i32) == k - 1 {
                res.push(max(&deque))
            }
        }

        return res;
    }

    pub fn two_sum(nums: &mut Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            map.insert(nums[i], i);
        }
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if map.contains_key(&complement) && map[&complement] != i {
                return vec![i as i32, map[&complement] as i32];
            }
        }

        return vec![];
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr.take() {
            let next_temp = curr_node.next.take();
            curr_node.next = prev.take();
            prev = Some(curr_node);
            curr = next_temp;
        }

        prev
    }

    pub fn middle_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast_p = &head;
        let mut slow_p = &head;
        while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
            slow_p = &slow_p.as_ref().unwrap().next;
            fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow_p.clone()
    }

    pub fn merge_two_list(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_list(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_list(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: None }));
        let mut cur = &mut dummy;
        let mut length = 0;
        while let Some(node) = cur.as_mut() {
            cur = &mut node.next;
            if let Some(_node) = cur {
                length += 1;
            }
        }

        let mut new_cur = dummy.as_mut();

        let idx = length - n;
        for _ in 0..idx {
            new_cur = new_cur.unwrap().next.as_mut();
        }
        let next = new_cur.as_mut().unwrap().next.as_mut().unwrap().next.take();
        new_cur.as_mut().unwrap().next = next;
        dummy.unwrap().next
    }
}
