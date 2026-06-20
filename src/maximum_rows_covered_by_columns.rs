fn dfs(
    n: usize,
    i: usize,
    selected: i32,
    columns: &mut i32,
    matrix: &Vec<i32>,
) -> i32 {
    if selected == 0 {
        return matrix.iter().fold(0, |acc, &num| {
            for i in 0..32 {
                if num >> i & 1 == 1 && *columns >> i & 1 == 0 {
                    return acc;
                }
            }
            acc+1
        });
    }

    let mut res = 0;
    for i in i..n {
        *columns ^= 1 << i;
        res = res.max(dfs(n, i+1, selected-1, columns, matrix));
        *columns ^= 1 << i;
    }

    res
}

fn maximum_rows(matrix: Vec<Vec<i32>>, num_select: i32) -> i32 {
    let n = matrix[0].len();
    let matrix = matrix
        .into_iter()
        .map(|v| v
            .into_iter()
            .enumerate()
            .fold(0, |acc, (i, num)| acc ^ num << (n-i-1))
        )
        .collect::<Vec<_>>();

    dfs(n, 0, num_select, &mut 0, &matrix)
}

pub fn main() {
    let matrix = [
        [0,0,0,1,0],
        [1,0,1,0,1],
        [0,1,1,1,0],
        [0,0,1,0,0],
        [1,1,1,1,1],
    ].into_iter().map(Vec::from).collect();
    let num_select = 3;
    println!("{}", maximum_rows(matrix, num_select));
}
