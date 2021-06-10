//! Compenet system holds all attributes that can be
//! assigned to entities

use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

/// Coordinates for any entity
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Character and colors to represent each entity
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

/// Gives Player Status
#[derive(Component, Debug)]
pub struct Player {}

/// Gives Blob Status
#[derive(Component, Debug)]
pub struct Blob {}

/// Shows the tiles that are visible within the range
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
}

/// Name of each entity
#[derive(Component, Debug, Clone)]
pub struct Name {
    pub name: String,
}

/// Stops entities from occipying the same place
#[derive(Component, Debug)]
pub struct BlocksTile {}

/// Entity health
#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
}

/// Sets the target to attack when the melee system runs 
#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

/// Sets the damage when the damage system runs
#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

/// Allows multiple attacks in the same turn stored into the vector
impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

/// Assigns Item status
#[derive(Component, Debug)]
pub struct Item {}

/// Stores the amount to heal for an item
#[derive(Component, Debug)]
pub struct Heal {
    pub heal_amount: i32,
}

/// Items are moved to inventory and assigned an owner
#[derive(Component, Debug, Clone)]
pub struct Inventory {
    pub owner: Entity,
}

/// Used to store a queue for the item system in case multiple
/// entities try to pickup the same item
#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}
