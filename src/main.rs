#![deny(clippy::pedantic)]
#![allow(dead_code)]
use std::fmt;

#[derive(Default)]
struct Tile<'a> {
    entities: Vec<&'a Placeable>,
}

impl<'a> fmt::Display for Tile<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.entities.len())
    }
}

/* Things that can be placed on a tile */
trait Placeable {
    fn get_pos(&self) -> (u32, u32);
    fn set_pos(&mut self, x: u32, y: u32);
}

trait AutoMove {
    fn get_new_pos(&self) -> (u32, u32);
}

#[derive(Debug, Default)]
struct Ant {
    x: u32,
    y: u32,
}

impl Placeable for Ant {
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

impl AutoMove for Ant {
    fn get_new_pos(&self) -> (u32, u32) {
        (0, 0)
    }
}

struct TileSet<'a> {
    columns: usize,
    tiles: Vec<Tile<'a>>,
}

impl<'a> TileSet<'a> {
    fn init(rows: usize, columns: usize) -> Self {
        let nr_tiles = rows * columns;
        let mut tiles: Vec<Tile> = Vec::with_capacity(nr_tiles);
        for _ in 0..(nr_tiles) {
            tiles.push(Tile::default());
        }

        Self {
            columns: columns,
            tiles: tiles,
        }
    }

    fn convert_xy(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    fn add_placeable(&mut self, x: usize, y: usize, p: &'a Placeable) {
        let index = self.convert_xy(x, y);
        self.tiles[index].entities.push(p);
    }
}

impl<'a> fmt::Display for TileSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, tile) in self.tiles.iter().enumerate() {
            if index > 0 && index % self.columns == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{} ", tile)?;
        }

        Ok(())
    }
}

// fn move_placeable(tile_set: &mut TileSet, placeables: Vec<&Placeable>) {
//     // clear entities
//     for tile in &mut tile_set.tiles {
//         tile.entities.clear();
//     }

//     for p in placeables {
//         let (x, y) = p.get_pos();
//         println!("{} {}", x, y);
//         tile_set.add_placeable(x as usize, y as usize, p);
//     }
// }

fn main() {
    const WORLD_SIZE: usize = 2;
    let mut tile_set = TileSet::init(WORLD_SIZE, WORLD_SIZE);

    let mut ants: Vec<Ant> = Vec::new();
    for _ in 0..8 {
        ants.push(Ant::default());
    }

    let some_tile = &mut tile_set.tiles[0];
    some_tile.entities.push(&ants[0]);
    let my_ant = some_tile.entities.pop();

    // move_placeable(
    //     &mut tile_set,
    //     ants.iter().map(|a| a as &Placeable).collect(),
    // );

    // println!("{}", tile_set);
}
