//! Damage system
//! Scans through all pending damage and assess damage
//! When entities have no health deletes them and drops items

use super::{CombatStats, Heal, Item, Name, Position, Renderable, SufferDamage};
use rltk::RGB;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    /// Assess all pending damage events and then clears them
    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

/// When blobs die boss blobs will drop a colored crystal
/// grey blobs will drop a heal
pub fn drop_item(ecs: &mut World, x: i32, y: i32, i: String) -> Entity {
    // Pick the color
    let color: RGB;
    if i == "RED" {
        color = RGB::named(rltk::RED)
    } else if i == "BLUE" {
        color = RGB::named(rltk::BLUE)
    } else if i == "PURPLE" {
        color = RGB::named(rltk::PURPLE)
    } else if i == "YELLOW" {
        color = RGB::named(rltk::YELLOW)
    } else {
        color = RGB::named(rltk::GREY)
    }

    // Drop a heal
    if i == "GREY" {
        ecs.create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('+'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Name {
                name: "Health".to_string(),
            })
            .with(Item {})
            .with(Heal { heal_amount: 4 })
            .build()
    // Drop a colored crystal
    } else {
        ecs.create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('♦'),
                fg: color,
                bg: RGB::named(rltk::BLACK),
            })
            .with(Name { name: i })
            .with(Item {})
            .with(Heal { heal_amount: 4 })
            .build()
    }
}

/// Entities with no health are assessed for their drop
/// then removed from the world
pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    let mut drops: Vec<(String, i32, i32)> = Vec::new();
    // Using a scope to make the borrow checker happy
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();
        let names = ecs.read_storage::<Name>();
        let pos = ecs.read_storage::<Position>();
        for (entity, stats, name, pos) in (&entities, &combat_stats, &names, &pos).join() {
            if stats.hp < 1 {
                dead.push(entity);
                if name.name != "Player" {
                    drops.push((name.name.clone(), pos.x, pos.y))
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }

    for items in drops {
        let drop = drop_item(ecs, items.1, items.2, items.0);
        ecs.insert(drop);
    }
}
