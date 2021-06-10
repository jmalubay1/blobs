use super::{BlocksTile, Map, Position};
use specs::prelude::*;

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        // Rescan the map for moved entites and clear old content
        map.occupied();
        map.clear_content_index();

        for (entity, position) in (&entities, &position).join() {
            let idx = map.index(position.x, position.y);

            // If they block, update the blocking list
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.occupied[idx] = true;
            }

            // Copy the entity into the content list 
            map.tile_content[idx].push(entity);
        }
    }
}
