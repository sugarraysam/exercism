const CARS_PER_SPEED: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let raw_production: f64 = speed as f64 * CARS_PER_SPEED;

    // can panic, but exercism does not provide signature to handle errors
    raw_production * success_rate(speed).unwrap()
}

fn success_rate(speed: u8) -> Option<f64> {
    match speed {
        0 => Some(0.0),
        1..=4 => Some(1.0),
        5..=8 => Some(0.9),
        9 | 10 => Some(0.77),
        _ => None,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_rate_per_hour_at_speed_zero() {
        assert_eq!(0.0, production_rate_per_hour(0));
    }

    fn test_production_rate_per_minute_at_zero_speed() {
        assert_eq!(0, working_items_per_minute(0));
    }
}
