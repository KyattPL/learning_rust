fn main() {
    let n = 8;
    println!("{}th Fibonacci number is {}", n, fibonacci(n - 1));
}

fn fibonacci(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}
