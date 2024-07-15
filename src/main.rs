fn main() {
    let fibonacci_result: i32 = fibonnaci(10);
    println!("{fibonacci_result}");
}

fn fibonnaci(n: i32) -> i32 {
    (0..n)
		.fold((1, 0), |(prev1, prev2), _| (prev2, prev1 + prev2))
		.1
}
