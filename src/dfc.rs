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

pub fn obtain_number_of_aricles() -> u32 {
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

pub fn calculate_delivery_cost(
    value: f32,
    distance: u32,
    articles: u32,
    day: u32,
    hour: u32,
) -> f32 {
    // The delivery is free (0€) when the cart value is equal or more than 200€.
    if value >= 200.0 {
        0.0
    } else {
        let cost_from_articles = {
            // If the number of items is five or more, an additional 50 cent surcharge is added for each item above and including the fifth item.
            let above_5_surcharge = if articles >= 5 {
                (articles - 4) * 50
            } else {
                0
            };

            // An extra "bulk" fee applies for more than 12 items of 1,20€.
            let bulk_fee = if articles > 12 { 12 * 120 } else { 0 };

            (above_5_surcharge + bulk_fee) as f32
        };

        let cost_from_distance = {
            // As long as the distance doesn't exceed 1km the delivery fee is 2€.
            let base_delivery_fee = 2;

            // If the delivery distance is longer than 1km, 1€ is added for every additional 500 meters started.
            let additional_fee = {
                let distance_extention = distance.saturating_sub(1000);

                let full_half_kms = distance_extention / 500;
                let there_are_additional_meters = distance_extention % 500 != 0;
                full_half_kms + if there_are_additional_meters { 1 } else { 0 }
            };

            (base_delivery_fee + additional_fee) as f32
        };

        // If the cart value is less than 10€, a small order surcharge is added to the delivery price. The surcharge is
        // the difference between the cart value and 10€. For example if the cart value is 8.90€, the surcharge will be 1.10€.
        let cost_from_value = if value < 10.0 { 10.0 - value } else { 0.0 };

        let friday_multiplier = {
            // During the Friday rush, 3 - 7 PM, the delivery fee (the total fee including possible surcharges)
            // will be multiplied by 1.2x.
            if day == 5 && (15..19).contains(&hour) {
                1.2
            } else {
                1.0
            }
        };

        // The delivery fee can never be more than 15€, including possible surcharges.

        let fee = (cost_from_articles + cost_from_distance + cost_from_value) * friday_multiplier;
        fee.clamp(0.0, 15.0)
    }
}

pub fn _calculate_delivery_cost_old() -> f32 {
    use std::io::stdin;
    // Prompt for the user
    println!("Type the value of your order here:");
    // A new, mutable string 'buffer' is created to hold the user's input.
    let mut buffer = String::new();

    // this function reads a line of input from the standard input stream
    // until a newline is reached, and appends it to the provided String buffer.
    // read_line function appends a newline character to the buffer. If successful,
    // this function will return the total number of bytes read. The unwrap()
    // function is used to handle errors
    stdin().read_line(&mut buffer).unwrap();

    // trim() method removes any leading and trailing whitespace (including the newline character)
    // then the parse() function converts the string slice into an f32.
    let cart_value: f32 = buffer.trim().parse().unwrap();

    println!("Type the delivery distance (in meters):");
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let distance: f32 = buffer.trim().parse().unwrap();

    println!("Type the number of items in your cart:");
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let number_of_items: f32 = buffer.trim().parse().unwrap();

    let mut delivery_cost: f32;

    // Calculates the delivery cost based on cart/order value
    if cart_value < 10.0 {
        delivery_cost = 10.0 - cart_value;
    } else {
        delivery_cost = 0.0;
    }

    // Adjusts delivery cost for distance
    if distance <= 500.0 {
        delivery_cost += 1.0;
    }

    // Adjusts delivery cost based on the number of items
    if (5.0..=12.0).contains(&number_of_items) {
        delivery_cost += 0.50 * (number_of_items - 4.0);
    } else if number_of_items > 12.0 {
        delivery_cost += 0.50 * (number_of_items - 4.0) + 1.2;
    }

    if cart_value >= 200.0 {
        delivery_cost = 0.0;
    }

    delivery_cost
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_thing() {
        obtain_distance();
    }
}
