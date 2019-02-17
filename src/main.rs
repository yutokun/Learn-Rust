fn main() {
//	fizzbuzz(20);
//	println!("{}", square_sum(10));
	variable_binding();
	string_types();
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
//	return は不要;
}

fn variable_binding() {
	// immutable by default
	let x = 1 + 2;
	println!("{}", x);

	// mutable pattern
	let mut m = 1 + 2;
	println!("{}", m + 2);
}

fn string_types() {
	// "abc" is &str but could cast as String.
	let mut a: String = "abc".to_string();

	// String + &str
	a += "def";
	println!("{}", a);

	// Other types have to_string()
	let x = 1.0.to_string();

	// String + String is not allowed.
	// So first cast as &str.
	a += x.as_str();
	println!("{}", a);
}