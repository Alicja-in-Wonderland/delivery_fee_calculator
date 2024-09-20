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

