fn fizzbuzz(n: usize) {
    for i in 0..n {
        match i {
            15 => println!("FizzBuzz"),
            3 => println!("Fizz"),
            5 => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    fizzbuzz(20);
}
