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
    let mut game = Game::init(EntityStore::init());
    
    for _ in 0..4 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    for _ in 0..1 {
        game.entity_store.create_entity(&EntityType::Pheromone);
    }

    for _ in 0..10 {
        game.move_entities();
        println!("{}", game.entity_store);
    }
}
