use std::io::stdin;

fn main() {
    // The value returned from get_cart_value is stored in delivery_cost variable
    let delivery_cost = calculate_delivery_cost();
    println!("The cost of your delivery is: {} €", delivery_cost);
}

pub fn calculate_delivery_cost() -> f32 {
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

    println!("Type the number of items in your cart:");
    buffer.clear();
    stdin().read_line(&mut buffer).unwrap();
    let number_of_items: f32 = buffer.trim().parse().unwrap();

    const BASE_FEE: f32 = 1.0;
    let mut delivery_cost: f32;

    // Checks different conditions to calculate the delivery cost based on cart/order value
    if cart_value < 10.0 {
        delivery_cost = BASE_FEE + (10.0 - cart_value);
    } else {
        delivery_cost = BASE_FEE;
    }

    // Adjusts delivery cost based on the number of items
    if (5.0..=12.0).contains(&number_of_items) {
        delivery_cost += 0.50 * (number_of_items - 4.0);
    } else if number_of_items > 12.0 {
        delivery_cost = delivery_cost + 0.50 * (number_of_items - 4.0) + 1.2;
    }

    if cart_value >= 200.0 {
        delivery_cost = 0.0;
    }

    delivery_cost

}
