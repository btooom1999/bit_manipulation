fn is_power_of_two(n: i32) -> bool {
    n > 0 && n & (-n) == n
}

pub fn main() {
    let n = 1;
    println!("{}", is_power_of_two(n));
}
