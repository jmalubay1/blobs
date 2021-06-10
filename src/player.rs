//! Control the player entity movement, inventory, and player controls
//! 
use super::{Item, Map, Player, Position, RunState, State, WantsToPickupItem};
use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

/// Compares new locations with all other entites and occupied tiles
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    // Scan other entities for conflcits
    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = map.index(pos.x + delta_x, pos.y + delta_y);
        // Scan for walls
        if !map.occupied[destination_idx] {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            let mut ppos = ecs.write_resource::<Point>();
            // Update position on valid move
            ppos.x = pos.x;
            ppos.y = pos.y
        }
    }
}

/// Verifies the locations of the player and item then
/// sends the pickup request to the wants to pickup system
fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let positions = ecs.read_storage::<Position>();

    // Check locations
    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if position.x == player_pos.x && position.y == player_pos.y {
            target_item = Some(item_entity);
        }
    }

    // Send to WantsToPickUp queue
    match target_item {
        None => (),
        Some(item) => {
            let mut pickup = ecs.write_storage::<WantsToPickupItem>();
            pickup
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item,
                    },
                )
                .expect("Unable to insert want to pickup");
        }
    }
}

/// Waits for player input and returns the running state
pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => return RunState::AwaitingInput, // Nothing happened
        Some(key) => match key {
            // Open Menu
            VirtualKeyCode::Escape => return RunState::Menu,
            // Character Movement
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            // Shoot
            VirtualKeyCode::Up => todo!(),
            VirtualKeyCode::Down => todo!(),
            VirtualKeyCode::Left => todo!(),
            VirtualKeyCode::Right => todo!(),
            // Pickup item
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
