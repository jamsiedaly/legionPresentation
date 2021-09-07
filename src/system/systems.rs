use crate::resource::inventory::{Inventory, PlayerInventory};

use crate::component::building::House;
use game_time::GameClock;
use legion::*;

#[system(for_each)]
pub fn sync_resources(
    player_inventory: &mut PlayerInventory,
    #[resource] inventory: &mut Inventory,
) {
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
pub fn housing(
    house: &mut House,
    #[resource] inventory: &mut Inventory,
    #[resource] clock: &GameClock,
) {
    house.time_since_last_spawn += clock
        .last_frame_time()
        .elapsed_game_time()
        .as_milliseconds();
    if house.population <= 4
        && inventory.food >= 50
        && house.time_since_last_spawn >= House::TIME_BETWEEN_SPAWNS
    {
        house.population += 1;
        inventory.population_change += 1;
    }
}

#[system(for_each)]
pub fn harvest(house: &mut House, #[resource] inventory: &mut Inventory) {
    inventory.food_change += house.population * 2
}
