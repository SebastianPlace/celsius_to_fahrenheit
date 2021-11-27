use std::io;

// Get user input
// Convert to f64 so that we can deal with negative numbers and decimals
// Formula x°F = (y°C * 9/5) + 32
fn main() {
    loop {
        println!("Enter degrees celsius:");
        let mut celsius = String::new();

        match io::stdin().read_line(&mut celsius) {
            Ok(_) => {},
            Err(_) => continue, // Enter new input on error
        }

        // We use trim() since our input will have a newline char at the end, which means it cannot be converted to a number.
        let celsius: f64 = match celsius.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue, // Enter new input on error
        };

        let conversion = celsius_to_fahrenheit(celsius);

        println!("{}°C is {}°F", celsius, conversion);
    }
}

// We cast our ints to floats because rust doesn't allow arithmetic with multiple types.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * (9 / 5) as f64) + 32 as f64
}
