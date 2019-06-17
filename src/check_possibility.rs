pub fn test() {
    let mut nums = vec![4, 2, 3];

    println!("{:?}", check_possibility(nums));
}

pub fn check_possibility(nums: Vec<i32>) -> bool {
    let len = nums.len();

    let mut change_count = 0u8;

    for i in 0..len {
        if i < len - 1 {
            let value = nums[i];
            let next_value = nums[i+1];
            if value > next_value {
                change_count += 1;

                if change_count > 1 {
                    return false;
                }

                if i > 0 && i-1 >= 0 && nums[i-1] > next_value && i+2 < len && value > nums[i+2] {
                    return false;
                }
            }
        }
    }

    return true;
}