//! Renders the players information and menu
use super::{CombatStats, Inventory, Name, Player};
use rltk::{Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;


/// Draws the player health as a number and a bar
pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    // Main HUD box
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();

    //Draw player stats
    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
        //Draw Health Text
        ctx.print_color(
            6,
            43,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &health,
        );
        //Draw Health Bar
        ctx.draw_bar_horizontal(
            23,
            43,
            51,
            stats.hp,
            stats.max_hp,
            RGB::named(rltk::RED),
            RGB::named(rltk::BLACK),
        );
    }

    // Get the items in the players inventory
    let player_entity = ecs.fetch::<Entity>();
    let inventory = ecs.read_storage::<Inventory>();
    let names = ecs.read_storage::<Name>();
    let mut x = 20;

    // Draw 'Crystals:' text
    ctx.print_color(
        6,
        46,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Crystals: ".to_string(),
    );

    // Draw the crystals the player has in their inventory
    for (_pack, name) in (&inventory, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity)
    {
        let color: RGB;
        // Get the correct color to draw
        if name.name == "RED" {
            color = RGB::named(rltk::RED)
        } else if name.name == "BLUE" {
            color = RGB::named(rltk::BLUE)
        } else if name.name == "PURPLE" {
            color = RGB::named(rltk::PURPLE)
        } else {
            color = RGB::named(rltk::YELLOW)
        }
        // Draw the crystal and increment x to space the next crystal
        ctx.print_color(x, 46, color, RGB::named(rltk::BLACK), "♦".to_string());
        x += 4;
    }
}

/// Controls menu interactions
#[derive(PartialEq, Copy, Clone)]
pub enum MenuResult {
    Cancel,
    NoResponse,
}

/// Brings up the menu for the player to choose from
/// Selection is controlled with specific key selection
pub fn show_menu(ctx: &mut Rltk) -> MenuResult {
    // Number of menu options (row spacing inside box)
    let count = 1;
    let y = (25 - (count / 2)) as i32;

    // Draw menu box in center of screen with
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Menu",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    // Draw first row - Quit
    ctx.set(
        17,
        y,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        rltk::to_cp437('('),
    );
    ctx.set(
        18,
        y,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        113_u16,
    );
    ctx.set(
        19,
        y,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
        rltk::to_cp437(')'),
    );
    ctx.print(21, y, "Quit".to_string());

    //y += 1;
    // To add more options increase 'count' variable above and incriment y for every new row

    // Menu select options by Key
    match ctx.key {
        None => MenuResult::NoResponse,
        Some(key) => {
            match key {
                // Leave the Menu
                VirtualKeyCode::Escape => MenuResult::Cancel,
                // Quit the game
                VirtualKeyCode::Q => std::process::exit(0),
                _ => MenuResult::NoResponse,
            }
        }
    }
}
