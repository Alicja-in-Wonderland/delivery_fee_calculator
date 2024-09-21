pub fn inform_user_about_self() {
    println!(
        "I'm gonna help you with calculating the delivery cost of your shit. You are welcome."
    );
}

pub fn obtain_order_value() -> f32 {
    println!("Type the value of your order:");
    read_line()
        .parse()
        .expect("Expected number of format: #.# or #")
}

pub fn obtain_distance() -> u32 {
    println!("Type delivery distance in meters:");
    read_line().parse().expect("Expected positive integer")
}

pub fn obtain_number_of_aricles() -> f32 {
    println!("Type the number of articles in your cart:");
    read_line().parse().expect("Expected positive integer")
}

pub fn obtain_day_of_the_week() -> u32 {
    println!("Type the day of the week you'd like to recieve your order:");
    let input = read_line();

    // check if input is equal or longer than 3
    if input.len() < 3 {
        panic!("Invalid input");
    }

    // match lowercased 3 first characters and return appropriate number
    match &input.to_lowercase()[0..3] {
        "mon" => 1,
        "tue" => 2,
        "wed" => 3,
        "thu" => 4,
        "fri" => 5,
        "sat" => 6,
        "sun" => 7,
        _ => panic!("Invalid day of the week"),
    }
}

pub fn obtain_hour() -> u32 {
    println!("What time do you want to receive your order?");
    println!("Type the hour (0..=23):");
    let hour: u32 = read_line().parse().expect("Expected positive integer");
    if hour >= 24 {
        panic!("Hour must be between 0 and 23.")
    }
    hour
}

pub fn inform_about_delivery_cost(cost: f32) {
    println!("The delivery fee is: {} €", cost);
}

fn read_line() -> String {
    use std::io::stdin;
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Read line failed.");
    buffer.trim().to_string()
}