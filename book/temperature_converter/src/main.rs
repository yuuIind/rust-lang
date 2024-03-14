use std::io;

fn main() {
    // true = fahrenheit to celsius, false = celsius to fahrenheit
    let mut convertion_type : bool = true; 
    println!("Welcome!");

    loop {
        println!("Please input the temperature in fahrenheit.");
        println!("Please 'change' to change the temperature to celsius.");
        println!("Type 'exit' to quit.");

        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if temperature.to_lowercase().trim() == "exit" {
                    println!("Exiting the program...");
                    break;
                } else if temperature.to_lowercase().trim() == "change" {
                    convertion_type = !convertion_type;
                    continue;
                } else {
                    println!("Please type a number!\n");
                    continue;
                }
            },
        };

        if convertion_type {
            println!("The temperature in celsius is: {}", to_celsius(temperature));
        } else {
            println!("The temperature in fahrenheit is: {}", to_fahrenheit(temperature));
        }

        println!("");
    }
}


fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * (9.0 / 5.0) + 32.0
}
