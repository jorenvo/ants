#![deny(clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
extern crate rand;

mod components;
mod entities;
mod entity_store;
mod game;
mod utils;

use entities::*;
use entity_store::*;
use game::*;

fn main() {
    let mut game = Game::init(EntityStore::default(), 10.0, 10.0);

    for _ in 0..1 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    game.entity_store.create_entity(&EntityType::Sugar);

    for _ in 0..200 {
        game.tick();
        println!("{}", game);
    }
}
