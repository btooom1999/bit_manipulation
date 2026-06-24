fn max_strength(nums: Vec<i32>) -> i64 {
    if nums.len() == 1 {
        return nums[0] as i64;
    }

    let (mut max, mut negative, mut product) = (i32::MIN, 0, 1i64);
    let mut res = 0i64;
    for num in nums {
        if num < 0 {
            negative += 1;
            max = max.max(num);
            product *= num as i64;
        } else if num > 0 {
            res = res.max(1) * num as i64;
        }

    }

    if negative <= 1 {
        return res;
    }

    if negative % 2 == 1 {
        return res.max(1) * (product / max as i64);
    }

    if res == 0 {
        return product;
    }

    res * product
}

pub fn main() {
    let nums = [-2,-3].to_vec();
    println!("{}", max_strength(nums));
}
