mod dfc;
mod interface;

fn main() {
    interface::inform_user_about_self();

    let order_value = interface::obtain_order_value();
    let distance_meters = interface::obtain_distance();
    let number_of_articles = interface::obtain_number_of_aricles();
    let day_of_the_week = interface::obtain_day_of_the_week();
    let hour = interface::obtain_hour();

    let cost: f32 = dfc::full_cost(
        order_value,
        distance_meters,
        number_of_articles,
        day_of_the_week,
        hour,
    );

    interface::inform_about_delivery_cost(cost);
}

