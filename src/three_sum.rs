pub fn test() {
    let mut nums = vec![-1, 0, 1, 2, -1, -4];

    println!("{:?}", three_sum(nums));
}

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    if len < 3 {
        return Vec::new();
    }

    nums.sort();


    let mut out = Vec::new();

    for i in 0..len - 2 {
        if i == 0 || i > 0 && nums[i] != nums[i - 1] {
            let mut l_index = i + 1;
            let mut r_index = len - 1;

            while l_index < r_index {
                let sum = nums[i] + nums[l_index] + nums[r_index];

                if sum == 0 {
                    out.push(vec![nums[i], nums[l_index], nums[r_index]]);

                    while l_index < r_index && nums[l_index] == nums[l_index + 1] {
                        l_index += 1;
                    }

                    while l_index < r_index && nums[r_index] == nums[r_index - 1] {
                        r_index -= 1;
                    }

                    l_index += 1;
                    r_index -= 1;
                } else if sum > 0 {
                    while l_index < r_index && nums[r_index] == nums[r_index - 1] {
                        r_index -= 1;
                    }

                    r_index -= 1;
                } else {
                    while l_index < r_index && nums[l_index] == nums[l_index + 1] {
                        l_index += 1;
                    }

                    l_index += 1;
                }
            }
        }
    }
    return out;
}