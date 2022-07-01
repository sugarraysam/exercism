fn expected_minutes_in_oven() -> i32 {
    40
}

fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
