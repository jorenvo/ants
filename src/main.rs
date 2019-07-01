#![deny(clippy::pedantic)]
extern crate rand;

mod components;
mod entity_store;
mod entities;
mod game;

use entity_store::*;
use entities::*;
use game::*;

fn main() {
    let mut game = Game {
        entity_store: EntityStore::init(),
        rng: rand::thread_rng(),
    };
    for _ in 0..4 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    game.move_entities();

    println!("{}", game.entity_store);
}
