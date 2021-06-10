//! Constructs and displays the map as well as
//! controls the list occupied tiles and their contents 

use super::Rect;
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use specs::prelude::*;

/// Adjust these to the terminal size in main
/// MAPWIDTH is terminal width
/// MAPHEIGHT is terminal height - 7
const MAPWIDTH: usize = 80;
const MAPHEIGHT: usize = 43;
const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

///Map tiles for drawing all background elements
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    VWall,
    HWall,
    Floor,
    HDoor,
    VDoor,
    Cur,
    Cul,
    Clr,
    Cll,
}

/// All Map info vectors are width * height
#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub occupied: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    /// Finds the index from the map vector given an appropriate x,y
    pub fn index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /// Adds a room from the Rectangle type
    /// Each room has doors on all sides halfway on the wall
    fn add_room(&mut self, room: &Rect) {
        // Draw the walls and doors
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.index(x, y);
                if y == room.y1 + 1 || y == room.y2 {
                    // Horizontal doors
                    if x == room.x1 + 1 + ((room.x2 - room.x1) / 2) {
                        self.tiles[idx] = TileType::HDoor
                    }
                    // Upper left corner
                    else if x == room.x1 + 1 && y == room.y1 + 1 {
                        self.tiles[idx] = TileType::Cul
                    }
                    // Upper right corner
                    else if x == room.x2 && y == room.y1 + 1 {
                        self.tiles[idx] = TileType::Cur
                    }
                    // Lower left corner
                    else if x == room.x1 + 1 && y == room.y2 {
                        self.tiles[idx] = TileType::Cll
                    }
                    // Lower right corner
                    else if x == room.x2 && y == room.y2 {
                        self.tiles[idx] = TileType::Clr
                    }
                    // Horizonal Wall
                    else {
                        self.tiles[idx] = TileType::HWall;
                    }
                } else if x == room.x1 + 1 || x == room.x2 {
                    // Vertical doors
                    if y == room.y1 + 1 + ((room.y2 - room.y1) / 2) {
                        self.tiles[idx] = TileType::VDoor
                    }
                    // Vertical Wall
                    else {
                        self.tiles[idx] = TileType::VWall;
                    }
                }
            }
        }
    }

    /// Places 5 rooms randomly around the map
    pub fn map_gen() -> Map {
        // Initialize map data
        let mut map = Map {
            tiles: vec![TileType::Floor; MAPCOUNT],
            rooms: Vec::new(),
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            occupied: vec![false; MAPCOUNT],
            tile_content: vec![Vec::new(); MAPCOUNT],
        };

        // Adjust to changes the number of rooms and size
        const NUM_ROOMS: i32 = 4;
        const WIDTH: i32 = 9;
        const HEIGHT: i32 = 7;

        let mut rng = RandomNumberGenerator::new();

        // Randomly place all the rooms with no overlap
        let mut i = 0;
        while i < NUM_ROOMS {
            let x = rng.roll_dice(1, MAPWIDTH as i32 - WIDTH - 1) - 1;
            let y = rng.roll_dice(1, MAPHEIGHT as i32 - HEIGHT - 1) - 1;
            let new_room = Rect::new(x, y, WIDTH, HEIGHT);
            let mut overlap = false;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    overlap = true
                }
            }
            if !overlap {
                map.add_room(&new_room);
                map.rooms.push(new_room);
                i += 1;
            }
        }
        map
    }

    /// Any tile containing impassable objects are occupied
    pub fn occupied(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.occupied[i] = *tile == TileType::HWall
                || *tile == TileType::VWall
                || *tile == TileType::Cur
                || *tile == TileType::Cul
                || *tile == TileType::Clr
                || *tile == TileType::Cll
        }
    }

    /// Checks if a move is inside the map or into an occupied tile
    fn valid_move(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = self.index(x, y);
        !self.occupied[idx]
    }

    /// Clears the entities from the content vector
    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

/// Draw all the tiles on the map
pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::HDoor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('─'),
                );
            }
            TileType::VDoor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('│'),
                );
            }
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437(' '),
                );
            }
            TileType::VWall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('║'),
                );
            }
            TileType::HWall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('═'),
                );
            }
            TileType::Cur => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('╗'),
                );
            }
            TileType::Cul => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('╔'),
                );
            }
            TileType::Clr => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('╝'),
                );
            }
            TileType::Cll => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('╚'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    /// Stop blobs from seeing the player when in rooms
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
    }

    /// Get distance for blobs when they move
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    /// Check which directions are movable
    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut moves = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        // Cardinal directions
        if self.valid_move(x - 1, y) {
            moves.push((idx - 1, 1.0))
        };
        if self.valid_move(x + 1, y) {
            moves.push((idx + 1, 1.0))
        };
        if self.valid_move(x, y - 1) {
            moves.push((idx - w, 1.0))
        };
        if self.valid_move(x, y + 1) {
            moves.push((idx + w, 1.0))
        };
        moves
    }
}
