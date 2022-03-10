pub fn build_heap_down_up(nums: &mut Vec<i32>) {
    for i in 1..nums.len() {
        heapify_down_up(nums, i);
    }
}

fn heapify_down_up(nums: &mut Vec<i32>, idx: usize) {
    let mut idx = idx;
    let mut parent_idx = (idx - 1) / 2;
    while nums[idx] > nums[parent_idx] {
        //子节点 > 父节点
        nums.swap(idx, parent_idx);
        idx = parent_idx;
        if idx == 0 {
            break;
        }
        parent_idx = (idx - 1) / 2;
    }
}

pub fn build_heap_up_down(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in (0..len / 2).rev() {
        heapify_up_down(nums, i, len)
    }
}
fn heapify_up_down(nums: &mut Vec<i32>, idx: usize, num_len: usize) {
    let mut idx = idx;

    loop {
        let mut max_pos = idx;

        // 判断当前节点是否小于左子节点，如果是则江左子节点设为较大节点
        if 2 * idx + 1 < num_len && nums[idx] < nums[2 * idx + 1] {
            max_pos = 2 * idx + 1;
        }
        // 判断当前节点是否小于右子节点，如果是则江右子节点设为较大节点
        if 2 * idx + 2 < num_len && nums[idx] < nums[2 * idx + 2] {
            max_pos = 2 * idx + 2;
        }

        if max_pos == idx {
            break;
        }
        nums.swap(idx, max_pos);
        idx = max_pos;
    }
}

pub fn insert(nums: &mut Vec<i32>, x: i32) -> bool {
    nums.push(x);
    if nums.len() > 1 {
        heapify_down_up(nums, nums.len() - 1)
    }
    true
}

pub fn remove_max(nums: &mut Vec<i32>) -> Option<i32> {
    if nums.len() == 0 {
        return None;
    }

    let max_value = nums[0];
    nums[0] = nums[nums.len() - 1];
    nums.remove(nums.len() - 1);
    if nums.len() > 1 {
        heapify_up_down(nums, 0, nums.len());
    }
    Some(max_value)
}
