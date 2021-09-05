#[derive(Clone, Copy, Debug, PartialEq)]
pub struct House {
    pub time_since_last_spawn: u128,
    pub population: i32}

impl House {
    pub const TIME_BETWEEN_SPAWNS: u128 = 5000;

    pub fn new() -> House {
        return House {
            population: 0,
            time_since_last_spawn: 0
        }
    }
}