//! Inventory system
//! Scans the vector of items pending pickup
//! and assigns them on a first come first serve basis
//! then removes them from the map

use super::{Inventory, Position, WantsToPickupItem};
use specs::prelude::*;

pub struct ItemCollectionSystem {}

impl<'a> System<'a> for ItemCollectionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Inventory>,
    );

    /// Scan pending items remove their position and assign an owner
    fn run(&mut self, data: Self::SystemData) {
        let (mut wants_pickup, mut positions, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack
                .insert(
                    pickup.item,
                    Inventory {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack entry");
        }

        wants_pickup.clear();
    }
}
