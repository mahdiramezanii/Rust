fn recursive_factorial(n: i32) -> i32 {
    if n <= 1 { 1 }
    else { n * recursive_factorial(n - 1) }
}

fn iterative_factorial(n: i32) -> i32 {
    // Variables are declared with `let`.
    // The `mut` keyword allows these variables to be mutated.
    let mut i = 1;
    let mut result = 1;
    while i <= n {
        result *= i;
        i += 1;
    }
    return result; // An explicit return, in contrast to the prior function.
}

fn main() {
    println!("Recursive result: {:?}", recursive_factorial(10));
    println!("Iterative result: {:?}", iterative_factorial(10));
}