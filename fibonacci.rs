fn fibonacci(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-2) + fibonacci(n-1),
    }
}

fn main() {
    println!("{}", fibonacci(20));
}
