#[test]
fn time_multiplier() {
    assert_eq!(1.0, super::apply_time_multiplier(1.0, 1, 1));

    let day = 3;
    let hour = 12;
    let fee = 66.6;
    assert_eq!(super::apply_time_multiplier(fee, day, hour), 66.6);
    assert_eq!(super::apply_time_multiplier(fee, 5, 16), 66.6 * 1.2);
}

#[test]
fn cost_from_articles() {
    assert_eq!(super::cost_from_articles(3.0), 0.0);
    assert_eq!(super::cost_from_articles(4.0), 0.0);
    assert_eq!(super::cost_from_articles(5.0), 0.5);
    assert_eq!(super::cost_from_articles(6.0), 1.0);
}

#[test]
fn cost_from_distance() {
    assert_eq!(super::cost_from_distance(500), 2.0);
    assert_eq!(super::cost_from_distance(1499), 3.0);
    assert_eq!(super::cost_from_distance(1500), 3.0);
    assert_eq!(super::cost_from_distance(1501), 4.0);
}

#[test]
fn cost_from_value() {
    assert_eq!(super::cost_from_value(7.0), 3.0);
    assert_eq!(super::cost_from_value(12.0), 0.0);
}
