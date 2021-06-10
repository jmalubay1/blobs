//! Attack of the Blobs
//! Roguelike game binary
//! 
//! Jordan Malubay CS410 - June 2021

use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod map_index;
pub use map_index::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod blob;
pub use blob::*;
mod view;
pub use view::VisibilitySystem;
mod melee;
pub use melee::*;
mod damage_system;
pub use damage_system::*;
mod gui;
mod inventory;
pub use inventory::*;

/// States used to control the flow of the game
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    BlobTurn,
    Menu,
}

/// World is the Entity Control System 
pub struct State {
    pub ecs: World,
}

/// Set and run the World control systems from each module
impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = BlobAi {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);

        self.ecs.maintain();
    }
}


impl GameState for State {
    /// Each tick represents a round of turns within the game
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear the screen and draw the map
        ctx.cls();
        draw_map(&self.ecs, ctx);

        // Get all the entities and draw them
        {
            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            for (pos, render) in (&positions, &renderables).join() {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
            }
        }

        // Draw the HUD
        gui::draw_ui(&self.ecs, ctx);

        // Set the run state
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        // Check the run state
        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::BlobTurn;
            }
            RunState::BlobTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::Menu => {
                if gui::show_menu(ctx) == gui::MenuResult::Cancel {
                    newrunstate = RunState::AwaitingInput;
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);
    }
}

fn main() -> rltk::BError {
    // Initialize a new window
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    context.with_post_scanlines(true);

    // Initialize the gamestate
    let mut gs = State { ecs: World::new() };

    // Start all the compenent systems in the gamestate
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Blob>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Heal>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<Inventory>();

    // Generate map and player start location in one of the rooms
    let map = Map::map_gen();
    let (player_x, player_y) = map.rooms[0].center();

    // Spawn the Player
    let player_entity = gs
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .with(Name {
            name: "Player".to_string(),
        })
        .with(CombatStats { max_hp: 10, hp: 10 })
        .build();

    // Spawn blobs, first 4 are 'boss' blobs, any after are regular
    let mut rng = rltk::RandomNumberGenerator::new();
    for i in 0..8 {
        //Ensure blobs are outside of rooms
        let (mut x, mut y);
        loop {
            let mut inside = false;
            x = rng.roll_dice(1, 79);
            y = rng.roll_dice(1, 42);
            for room in map.rooms.iter() {
                if room.inside((x, y)) {
                    inside = true;
                    break;
                }
            }
            if !inside {
                break;
            }
        }

        // Select the main colored blobs or generic grey blobs
        let glyph: rltk::FontCharType;
        let name: String;
        let color: RGB;
        match i {
            //Important Blobs
            1 => {
                glyph = rltk::to_cp437('O');
                name = "RED".to_string();
                color = RGB::named(rltk::RED)
            }
            2 => {
                glyph = rltk::to_cp437('O');
                name = "BLUE".to_string();
                color = RGB::named(rltk::BLUE)
            }
            3 => {
                glyph = rltk::to_cp437('O');
                name = "PURPLE".to_string();
                color = RGB::named(rltk::PURPLE)
            }
            4 => {
                glyph = rltk::to_cp437('O');
                name = "YELLOW".to_string();
                color = RGB::named(rltk::YELLOW)
            }
            // Extra Blobs
            _ => {
                glyph = rltk::to_cp437('O');
                name = "GREY".to_string();
                color = RGB::named(rltk::GREY)
            }
        }

        // Create each blob
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: color,
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
            })
            .with(Blob {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(BlocksTile {})
            .with(CombatStats { max_hp: 1, hp: 1 })
            .build();
    }

    // Add the map, player and set the initial run state
    gs.ecs.insert(map);
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    gs.ecs.insert(Point::new(player_x, player_y));

    // Run the game
    rltk::main_loop(context, gs)
}
