use std::collections::HashMap;

fn main() {
    let nums = vec![2, 11, 7, 15];
    let target = 9;

//    let result = two_sum(nums, target);
    let result = two_sum_hash(nums, target);

    println!("{:?}", result);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let length = nums.len();

    for i in 0..length - 1 {
        for j in i + 1..length {
            if target == nums[i] + nums[j] {
                return vec![i as i32, j as i32];
            }
        }
    }

    return vec![];
}

fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if map.contains_key(&complement) {
            if let Some(index) = map.get(&complement) {
                return vec![*index as i32, i as i32];
            }
        }

        map.insert(num, i);
    }

    return vec![];
}
