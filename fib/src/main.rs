fn main() {
    let mut cache: Vec<i64> = Vec::new();
    println!("fib 45 => {}", fib(45, &mut cache));
}

fn fib(n: i64, cache: &mut Vec<i64>) -> i64 {
    if cache.len() > n as usize {
        return cache[n as usize];
    }
    if n < 2 {
        return n;
    }

    fib(n - 1, cache) + fib(n - 2, cache);
    cache.push(result);
    result
}
