fn main() {
    println!("fib 45 => {}", fib(45));
}

fn fib(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
