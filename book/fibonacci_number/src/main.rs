use std::io;

fn main() {
    println!("Welcome!");

    loop {
        println!("Please input the number of the fibonacci sequence.");
        println!("Type 'exit' to quit.");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if n.to_lowercase().trim() == "exit" {
                    println!("Exiting the program...");
                    break;
                } else {
                    println!("Please type a valid number! e.g. a positive integer\n");
                    continue;
                }
            }
        };
        
        println!("The fibonacci number is: {}", fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut sum = 0;
        let mut current = 1;
        let mut last = 0;
        for _i in 1..n {
            sum = current + last;
            last = current;
            current = sum;
        }
        sum
    }
}
