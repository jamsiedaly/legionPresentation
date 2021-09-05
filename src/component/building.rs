#[derive(Clone, Copy, Debug, PartialEq)]
pub struct House {
    pub time_since_last_spawn: f64,
    pub population: i32,
}

impl House {
    pub const TIME_BETWEEN_SPAWNS: f64 = 5.0;

    pub fn new() -> House {
        return House {
            population: 0,
            time_since_last_spawn: 0.0,
        };
    }
}
