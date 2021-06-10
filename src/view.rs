///! Vision system that allows the blobs to see the player

use super::{Map, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Position>,
    );
    /// Scans the map for all entities in range of other entities
    fn run(&mut self, data: Self::SystemData) {
        let (map, entities, mut viewshed, pos) = data;

        for (_ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            // Updates all tiles in view 
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        }
    }
}
