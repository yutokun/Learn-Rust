fn main() {
//    fizzbuzz(20);
//    println!("{}", square_sum(10));
    variable_binding();
}

fn fizzbuzz(n: usize) {
    for i in 1..n + 1 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn square_sum(n: isize) -> isize {
    (0..n)
        .filter(|i| i % 2 == 0)
        .map(|i| i * i)
        .sum()
//    return は不要
}

fn variable_binding() {
    let x = 1 + 2; // immutable by default
    println!("{}", x);

    let mut m = 1 + 2; // mutable pattern
    println!("{}", m + 2);
}
