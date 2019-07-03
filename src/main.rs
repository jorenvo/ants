#![deny(clippy::pedantic)]
#![allow(clippy::cast_sign_loss,clippy::cast_possible_wrap)]
extern crate rand;

mod components;
mod entity_store;
mod entities;
mod game;

use entity_store::*;
use entities::*;
use game::*;

fn main() {
    let mut game = Game::init(EntityStore::default(), 10, 10);
    
    for _ in 0..4 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    game.entity_store.create_entity(&EntityType::Sugar);

    for _ in 0..140 {
        game.tick();
        println!("{}", game);
    }
}
