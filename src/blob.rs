///! Blob Module
///! Contains the AI function to move the blob toward the player 
use super::{Blob, Map, Position, Viewshed, WantsToMelee};
use rltk::Point;
use specs::prelude::*;

// Struct used to reference from main
pub struct BlobAi {}

impl<'a> System<'a> for BlobAi {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Blob>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
    );
    /// Scans through the entities looking for blobs
    /// Blobs in attacking range will hit the player
    /// Blobs in visual range will move toward the player
    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            entities,
            mut viewshed,
            blob,
            mut position,
            mut wants_to_melee,
        ) = data;

        for (entity, viewshed, _blob, mut pos) in
            (&entities, &mut viewshed, &blob, &mut position).join()
        {
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            // Attack the player
            if distance < 1.5 {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *player_entity,
                        },
                    )
                    .expect("Unable to insert attack");
            // Move toward the player
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                let path = rltk::a_star_search(
                    map.index(pos.x, pos.y),
                    map.index(player_pos.x, player_pos.y),
                    &*map,
                );
                // If move is succesful set the map tile to occupied to prevent overlap
                if path.success && path.steps.len() > 1 {
                    let mut idx = map.index(pos.x, pos.y);
                    map.occupied[idx] = false;
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    idx = map.index(pos.x, pos.y);
                    map.occupied[idx] = true;
                }
            }
        }
    }
}
