fn is_valid(s: &[u8], bit: usize) -> bool {
    let mut i = 0;
    let mut j = 30;

    while i < j {
        if bit >> i & 1 == 0 {
            i += 1;
        } else if bit >> j & 1 == 0 {
            j -= 1;
        } else if s[i] != s[j] {
            return false;
        } else {
            i += 1;
            j -= 1;
        }
    }

    true
}

fn dfs(
    s: &[u8],
    bits: &mut (usize, usize),
    i: usize,
    j: usize,
    n: usize,
    result: &mut i32,
    memo: &mut Vec<Vec<bool>>,
) {
    if memo[bits.0][bits.1] {
        return;
    }

    if bits.0 != 0 && bits.1 != 0 && is_valid(s, bits.0) && is_valid(s, bits.1) {
        *result = std::cmp::max(*result, bits.0.count_ones() as i32 * bits.1.count_ones() as i32);
    }

    if i < n && j < n {
        for i in i..n {
            if bits.1 >> i & 1 == 0 {
                bits.0 ^= 1 << i;
                dfs(s, bits, i+1, j, n, result, memo);
                bits.0 ^= 1 << i;
            }
        }

        for j in j..n {
            if bits.0 >> j & 1 == 0 {
                bits.1 ^= 1 << j;
                dfs(s, bits, i, j+1, n, result, memo);
                bits.1 ^= 1 << j;
            }
        }
    }

    memo[bits.0][bits.1] = true;
}

fn max_product(s: String) -> i32 {
    let mut result = 1;

    dfs(s.as_bytes(), &mut (0, 0), 0, 0, s.len(), &mut result, &mut vec![vec![false; 1 << s.len()]; 1 << s.len()]);
    result
}

pub fn main() {
    let s = "leetcodecom".to_string();
    println!("{}", max_product(s));
}
