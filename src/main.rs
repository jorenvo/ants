use std::fmt;

#[derive(Default)]
struct Tile {
    entities: Vec<Box<Placeable>>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.entities.len())
    }
}

/* Things that can be placed on a tile */
trait Placeable {}

#[derive(Debug, Default)]
struct Ant {
    x: u64,
    y: u64,
}

impl Placeable for Ant {}

fn main() {
    const WORLD_SIZE: usize = 2;
    let mut tiles: Vec<Tile> = Vec::with_capacity(WORLD_SIZE * WORLD_SIZE);
    for _ in 0..(WORLD_SIZE * WORLD_SIZE) {
        tiles.push(Default::default());
    }

    let mut ants: Vec<Ant> = Vec::new();
    for _ in 0..8 {
        ants.push(Default::default());
        tiles[0].entities.push(Box::new(Ant { x: 0, y: 0 }));
    }

    for tile in tiles {
        println!("{}", tile);
    }
}
