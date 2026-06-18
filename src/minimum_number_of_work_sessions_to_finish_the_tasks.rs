fn dfs(
    tasks: &Vec<i32>,
    visited: &mut usize,
    time: usize,
    session_time: usize,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if *visited == (1 << tasks.len()) - 1 {
        return 1;
    }

    if memo[*visited][time] != -1 {
        return memo[*visited][time];
    }

    let mut res = i32::MAX;
    for i in 0..tasks.len() {
        let task = tasks[i] as usize;
        if *visited >> i & 1 == 0 {
            *visited ^= 1 << i;
            if time + task > session_time {
                res = res.min(1+dfs(tasks, visited, task, session_time, memo));
            } else {
                res = res.min(dfs(tasks, visited, time+task, session_time, memo));
            }
            *visited ^= 1 << i;
        }
    }

    memo[*visited][time] = res;
    res
}

fn min_sessions(mut tasks: Vec<i32>, session_time: i32) -> i32 {
    let session_time = session_time as usize;
    dfs(&tasks, &mut 0, 0, session_time, &mut vec![vec![-1; session_time+1]; 1 << tasks.len()])
}

pub fn main() {
    let tasks = [9,8,8,4,6].to_vec();
    let session_time = 14;
    println!("{}", min_sessions(tasks, session_time));
}
