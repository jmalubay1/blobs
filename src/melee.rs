//! Processes pending melee combat events

use super::{CombatStats, Name, SufferDamage, WantsToMelee};
use specs::prelude::*;

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    ///Scan pending attacks and sent valid attacks to the damage system
    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut wants_melee, names, combat_stats, mut inflict_damage) = data;

        for (_entity, wants_melee, name, stats) in
            (&entities, &wants_melee, &names, &combat_stats).join()
        {
            // Check if attacker should be dead
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                
                // Check if target should be dead
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    println!(
                        "{} hits {}, for {} hp.",
                        &name.name, &target_name.name, 1
                    );
                    // Send to damage system
                    SufferDamage::new_damage(&mut inflict_damage, wants_melee.target, 1);
                    
                }
            }
        }
        // Clear any events that could not process
        wants_melee.clear();
    }
}
