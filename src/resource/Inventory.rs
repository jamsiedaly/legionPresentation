pub struct Inventory {
    pub food: i32,
    pub population: i32,
    pub wood: i32,
    pub food_change: i32,
    pub wood_change: i32,
    pub population_change: i32,
}

impl Inventory {
    pub fn new(food: i32, wood: i32) -> Inventory {
        Inventory {
            food,
            population: 0,
            wood,
            food_change: 0,
            population_change: 0,
            wood_change: 0,
        }
    }
}

pub struct Player_Inventory {
    pub food: i32,
    pub wood: i32,
    pub food_change: i32,
    pub wood_change: i32,
}

impl Player_Inventory {
    pub fn new(food: i32, wood: i32) -> Player_Inventory {
        Player_Inventory {
            food,
            wood,
            food_change: 0,
            wood_change: 0,
        }
    }
}

