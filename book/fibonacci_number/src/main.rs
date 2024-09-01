use std::io;

fn main() {
    println!("Hey, There!👋");
    println!("Which Fibonacci number do you want to know?");
    println!("Please enter n:");
    
    let n: u32 = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num > 0 => break num,
            _ => {
                println!("Please type a positive integer greater than 0!");
                continue;
            },
        };
    };
    
    println!("Ah! Just one more thing,");
    println!("Would you like to find it through wisdom or perseverance");

    let choice: String = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "wisdom" | "perseverance" => break input.trim().to_string(),
            _ => {
                println!("Invalid choice. Please type 'wisdom' or 'perseverance'.");
                continue;
            },
        };
    };

    let fib_n = fibonacci(n, &choice);
    println!("Fibonacci number {} is {}!", n, fib_n); 
}

fn fibonacci(n: u32, choice: &str) -> u64 {
    if choice == "wisdom" {
        fibonacci_closed(n as i32)
    } else {
        fibonacci_memoization(n)
    }
}

fn fibonacci_closed(n: i32) -> u64 {
    let phi: f64 = (1.0 + (5.0f64).sqrt()) / 2.0;
    let psi: f64 = (1.0 - (5.0f64).sqrt()) / 2.0;

    // Use Binet's formula
    let fib_n = (phi.powi(n) - psi.powi(n)) / (5.0f64).sqrt();

    // turn into an integer by rounding and converting u32
    fib_n.round() as u64
}

fn fibonacci_memoization(n: u32) -> u64 {
    // Initialize the memoization vector with a size of n + 1
    let mut memo: Vec<u64> = vec![0; (n + 1) as usize];
    memo[1] = 1;

    // The function that performs the actual calculation
    fn fib_gen(n: u32, memo: &mut Vec<u64>) -> u64 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        // Check if the value is already computed
        if memo[n as usize] != 0 {
            return memo[n as usize];
        }

        // Otherwise, Compute and store the value
        memo[n as usize] = fib_gen(n - 1, memo) + fib_gen(n - 2, memo);
        memo[n as usize]
    }

    // Call the helper function
    fib_gen(n, &mut memo)
}