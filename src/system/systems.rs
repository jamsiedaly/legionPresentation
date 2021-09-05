use crate::resource::inventory::{Player_Inventory, Inventory};

use legion::*;
use rand::rngs::ThreadRng;
use game_time::GameClock;
use crate::component::building::House;
use crate::component::position::Position;

#[system(for_each)]
pub fn sync_resources(player_inventory: &mut Player_Inventory, #[resource] inventory: &mut Inventory) {
    inventory.wood += player_inventory.wood_change + inventory.wood_change;
    inventory.food += player_inventory.food_change + inventory.food_change;
    player_inventory.wood = inventory.wood;
    player_inventory.food = inventory.food;
    player_inventory.food_change = 0;
    player_inventory.wood_change = 0;
    inventory.food_change = 0;
    inventory.food_change = 0;
}

#[system(for_each)]
fn housing_system(house: &mut House, #[resource] inventory: &mut Inventory, #[resource] clock: &GameClock) {
    house.time_since_last_spawn += clock.last_frame_time().elapsed_game_time().as_milliseconds();
    if house.population <= 4 && inventory.food >= 50 && house.time_since_last_spawn >= House::TIME_BETWEEN_SPAWNS {
        house.population += 1;
        inventory.population_change += 1;
    }
}

#[system(for_each)]
fn harvest(house: &mut House, #[resource] inventory: &mut Inventory) {
    inventory.food_change += house.population * 2
}

