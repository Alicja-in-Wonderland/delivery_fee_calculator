#[cfg(test)]
mod tests;

pub fn full_cost(value: f32, distance: u32, articles: f32, day: u32, hour: u32) -> f32 {
    // The delivery is free (0€) when the cart value is equal or more than 200€.
    if value >= 200.0 {
        0.0
    } else {
        apply_time_multiplier(
            cost_from_articles(articles) + cost_from_distance(distance) + cost_from_value(value),
            day,
            hour,
        )
        .clamp(0.0, 15.0)
    }
}

fn cost_from_articles(articles: f32) -> f32 {
    // If the number of items is five or more, an additional 50 cent surcharge is added for each item above and including the fifth item.
    let above_5_surcharge = if articles >= 5.0 {
        (articles - 4.0) * 0.5
    } else {
        0.0
    };

    // An extra "bulk" fee applies for more than 12 items of 1,20€.
    let bulk_fee = if articles > 12.0 { 12.0 * 120.0 } else { 0.0 };

    above_5_surcharge + bulk_fee
}

fn cost_from_distance(distance: u32) -> f32 {
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
}

// If the cart value is less than 10€, a small order surcharge is added to the delivery price. The surcharge is
// the difference between the cart value and 10€. For example if the cart value is 8.90€, the surcharge will be 1.10€.
fn cost_from_value(value: f32) -> f32 {
    if value < 10.0 {
        10.0 - value
    } else {
        0.0
    }
}

fn apply_time_multiplier(fee: f32, day: u32, hour: u32) -> f32 {
    // During the Friday rush, 3 - 7 PM, the delivery fee (the total fee including possible surcharges)
    // will be multiplied by 1.2x.
    let multiplier = if day == 5 && (15..19).contains(&hour) {
        1.2
    } else {
        1.0
    };

    multiplier * fee
}
