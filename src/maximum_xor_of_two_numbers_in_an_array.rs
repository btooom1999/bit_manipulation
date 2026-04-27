fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let max = *nums.iter().max().unwrap();
    let mut res = 0;

    for num in nums {
        res = res.max(max ^ num);
    }

    res
}

pub fn main() {
    let nums = [3,10,5,25,2,8].to_vec();
    println!("{}", find_maximum_xor(nums));
}
