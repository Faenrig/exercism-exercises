const MINUTES_EXPECTED: i32 = 40;
const MINUTES_PER_LAYER: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    MINUTES_EXPECTED
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let expected: i32 = expected_minutes_in_oven();
    if actual_minutes_in_oven > expected {
        println!("Your lasagna is already dead.");
        return -1;
    } else if actual_minutes_in_oven < 0 {
        println!("Something isn't right.");
        return -1;
    }
    expected - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    if number_of_layers < 0 {
        println!("You cannot use less than 0 layers.");
        return -1;
    }
    number_of_layers * MINUTES_PER_LAYER
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
