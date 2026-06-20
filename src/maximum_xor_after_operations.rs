fn maximum_xor(mut nums: Vec<i32>) -> i32 {
    let mut bits = [0;32];
    let mut bit = 0;
    for &num in &nums {
        bit ^= num;

        for i in 0..32 {
            if num >> i & 1 == 1 {
                bits[i] += 1;
            }
        }
    }

    let mut res = bit;
    let mut n = bits.iter().fold(0, |acc, &num| acc + (num > 0 && num % 2 == 0) as i32);
    for mut j in 0..1 << n {
        let mut i = 0;
        let mut num = 0;
        while j > 0 {
            while i < 32 && (bits[i] % 2 == 1 || bits[i] == 0) {
                i += 1;
            }
            num ^= (j & 1) << i;
            j >>= 1;
            i += 1;
        }

        res = res.max(bit ^ num);
    }

    res
}

pub fn main() {
    // let nums = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20].to_vec();
    let nums = [646,302,695,345,124,266,623,926,550,382,972,431,33,709,628,587,475,748,362,742,419,893,444,225,852,865,982,579,267,174,451,157,311,658,546,302,777,510,745,509,21,690,564,457,765,759,514,675,89,308,733,94,193,987,385,785,854,550,674,299,55,543,5,982,26,1000,165,820,358,392,804,282,823,991,38,358,45,982,164,94,447,197,348,578].to_vec();
    println!("{}", maximum_xor(nums));
}
