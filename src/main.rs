mod dfc;

use std::io::stdin;

use dfc::*;

fn main() {
    // dear program,

    // tell user what the fuck you do.
    inform_user_about_self();

    // obtain the value of the order from the user,
    let order_value: f32 = obtain_order_value();
    // obtain the delivery distance in meters from the user,
    let distance_meters: u32 = obtain_distance();
    // obtain the number of separate articles (pieces, items) their order consists of,
    let number_of_articles: u32 = obtain_number_of_aricles();
    // obtain the day of the week of the delivery,
    let day_of_the_week: u32 = obtain_day_of_the_week();
    // obtain the minute of the day of the delivery.
    let minute_of_the_day: u32 = obtain_minute_of_the_day();

    // calculate the delivery cost, according to specification.
    let delivery_cost_cents: u32 = calculate_delivery_cost(
        order_value,
        distance_meters,
        number_of_articles,
        day_of_the_week,
        minute_of_the_day,
    );

    // inform the user how much poorer they're going to be (assess the damage to their wallet).
    inform_about_delivery_cost(delivery_cost_cents);

    // best regards,
    // programmer
}

fn _main_old() {
    let delivery_cost = _calculate_delivery_cost_old();
    println!("The cost of your delivery is: {} €", delivery_cost);
}

pub fn _calculate_delivery_cost_old() -> f32 {
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
