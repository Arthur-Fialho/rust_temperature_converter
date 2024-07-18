use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1: Fahrenheit to Celsius");
    println!("2: Celsius to Fahrenheit");

    let mut choice = String::new();
    
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice
        .trim()
        .parse()
        .expect("Please enter a number");

    match choice {
        1 => {
            println!("Enter temperature in Fahrenheit:");

            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line");

            let fahrenheit: f64 = fahrenheit
                .trim()
                .parse()
                .expect("Please enter a number");

            let celsius = fahrenheit_to_celsius(fahrenheit);
            
            println!("{fahrenheit}°F is {celsius:.2}°C");

        }

        2 => {
            println!("Enter temperature in Celsius:");
            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read line");

            let celsius: f64 = celsius
                .trim()
                .parse()
                .expect("Please enter a number");

            let fahrenheit = celsius_to_fahrenheit(celsius);

            println!("{celsius}°C is {fahrenheit:.2}°F");
        }

        _ => println!("Invalid choice"),
    }

    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - 32.0) * 5.0 / 9.0
    }

    fn celsius_to_fahrenheit(c: f64) -> f64 {
        (c * 9.0 / 5.0) + 32.0
    }
}