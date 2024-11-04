use std::io::Write;

fn parse_with_units(input: &str) -> (f64, String) {
    // Separate the numeric part and the unit
    let mut numeric_part = String::new();
    let mut unit_part = String::new();

    // Separate the input into numeric and unit parts
    for c in input.chars() {
        if c.is_numeric() || c == '.' {
            numeric_part.push(c);
        } else if !c.is_whitespace() {
            unit_part.push(c);
        }
    }

    // Structure the input for further processing
    let value = numeric_part.parse::<f64>().expect("Invalid number format");

    (value, unit_part.trim().to_string())
}

fn ratio(input: &str) -> f64 {
    match input {
        "M" | "m" | "male" | "Male" | "MALE" => 0.68,
        "F" | "f" | "female" | "Female" | "FEMALE" => 0.55,
        _ => panic!("Could not determine ratio")
    }
}

fn weight(input: &str) -> f64 {
    // Separate the numeric part and the unit
    let value;
    let unit;
    (value, unit) = parse_with_units(input);

    // Convert the weight based on the unit
    match unit.as_str() {
        "" | "kg" | "Kg" | "KG" => value * 1000.0,      
        "lbs" | "LBS" | "lb" | "LB" => value * 453.592, 
        "g" | "G" => value,
        _ => panic!("Unknown unit! Use 'kg' or 'lbs'.")
    }
}

fn time_since_first_drink(input: &str) -> f64 {
    // Split input into hours and minutes
    let parts: Vec<&str> = input.split(':').collect();

    // Parse hours, defaulting to 0 if missing
    let hours = parts[0].parse::<f64>().unwrap_or(0.0);

    // Parse minutes if provided, otherwise default to 0
    let minutes = if parts.len() > 1 {
        parts[1].parse::<f64>().unwrap_or(0.0)
    } else {
        5.0
    };
    hours + (minutes / 60.0)
}

fn grams_of_alcohol(types_of_drinks: i64) -> f64 {
    let mut total = 0.0;
    for i in 0..types_of_drinks {
        // Name of the drink
        let mut input = String::new();
        print!("Name of drink {}: " , i + 1);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // Number of those drinks
        let mut number_of_drinks = String::new();
        print!("Number of {}s consumed: ", input.trim());
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut number_of_drinks).expect("Failed to read line");
        let number_of_drinks = number_of_drinks.trim().parse::<f64>().unwrap();

        // Size of each drink
        let mut size = String::new();
        print!("Size of one {} (Defaults to mL): ", input.trim());
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut size).expect("Failed to read line");
        let size = volume(size.trim());
        
        // ABV of the drink
        let mut abv = String::new();
        print!("ABV of one {} (Decimal): ", input.trim());
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut abv).expect("Failed to read line");
        let abv = abv.trim().parse::<f64>().unwrap();

        // Increment the total
        total += number_of_drinks * size * abv * 0.789;
        println!()
    }
    total
}

fn volume(input: &str) -> f64 {
    // Separate the numeric part and the unit
    let value;
    let unit;
    (value, unit) = parse_with_units(input);

    match unit.as_str() {
        "" | "ml" | "ML" | "mL" => value,
        "l" | "L" => value * 1000.0,
        "oz" | "OZ" | "oz." | "Oz." | "Oz" => value * 29.5735,
        _ => panic!("Unknown unit! Use 'ml' or 'l'.")
    }
}

fn number_of_standard_drinks() {
    // Collect the size of the drink
    let mut input = String::new();
    print!("Size of drink (Defaults to mL): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let volume = volume(input.trim());

    // Collect the ABV of the drink
    let mut input = String::new();
    print!("ABV of drink (Decimal): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let abv = input.trim().parse::<f64>().unwrap();

    // Calculate the number of standard drinks
    let number_of_standard_drinks = volume * abv * 0.789 / 14.;
    println!("That's {:.2} standard drinks.", number_of_standard_drinks);
}

fn run_bac_calculator() {
    // Collect the users gender
    let mut input = String::new();
    print!("Gender (M/F): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let ratio = ratio(input.trim());

    // Collect the users weight
    let mut input = String::new();
    print!("Weight (Defaults to kg): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let weight_in_grams = weight(input.trim());

    // Collect the users time since first drink
    let mut input = String::new();
    print!("Time since first drink (HH:MM): ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let time_since_first_drink = time_since_first_drink(input.trim());

    // Collect the user drinking amount
    let mut input = String::new();
    print!("Total types of drinks: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let number_of_drinks = input.trim().parse::<i64>().unwrap();
    println!("Number of drinks: {}", number_of_drinks);
    let alcohol_consumed = grams_of_alcohol(number_of_drinks);

    // Calculate the BAC
    let bac = bac(weight_in_grams, alcohol_consumed, ratio, time_since_first_drink);
    println!("Your BAC is: {:.4}", bac);
}

fn bac(weight: f64, alcohol_consumed: f64, ratio: f64, time_since_first_drink: f64) -> f64 {
    (100. * alcohol_consumed) / (weight * ratio) - 0.015 * time_since_first_drink
}

fn main() {
    loop {
        // Determine what  the user wants to do
        let mut input = String::new();
        println!("What would you like to do?");
        println!("1. Calculate BAC");
        println!("2. Calculate number of standard drinks");
        println!("3. Quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().parse::<i16>().unwrap();

        // Execute the user's choice
        match input {
            1 => run_bac_calculator(),
            2 => number_of_standard_drinks(),
            3 => break,
            _ => panic!("Invalid input!")
        }
        println!();
    }
}
