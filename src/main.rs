use std::io::stdin;

fn main() {

    let delivery_cost = get_cart_value();
    println!("The cost of your delivery is: {}", delivery_cost);

}

pub fn get_cart_value() -> u32 {
    println!("Type the value of your order here:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    // To change - cart_value should be of type float
    let cart_value: u32 = buffer.trim().parse().unwrap();

    const BASE_FEE: u32 = 1;
    let delivery_cost: u32;

    if cart_value >= 200 {
            delivery_cost = 0;
    } else if cart_value < 10 {
            delivery_cost = BASE_FEE + (10 - cart_value);
    } else {
            delivery_cost = BASE_FEE;
    }

    delivery_cost

}
