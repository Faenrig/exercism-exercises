const SLOWEST_ASSEMBLY_OUTPUT_PER_HOUR: i32 = 221;
const MEDIUM_SPEED_SUCCESS_RATE: f64 = 0.9;
const HIGH_SPEED_SUCCESS_RATE: f64 = 0.77;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed > 10 {
        println!("Assembly line speed ranges from 1 to 10");
        return -1.0
    }
    let mut produced: f64 = SLOWEST_ASSEMBLY_OUTPUT_PER_HOUR
    as f64 * speed as f64;
    if speed >= 5 && speed < 9 {
        produced *= MEDIUM_SPEED_SUCCESS_RATE;
    }
    if speed >= 9 {
        produced *= HIGH_SPEED_SUCCESS_RATE;
    }

    produced
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
