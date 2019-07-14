#![deny(clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
extern crate rand;

mod components;
mod entities;
mod entity_store;
mod game;
mod utils;

use components::*;
use entities::*;
use entity_store::*;
use game::*;

fn main() {
    let mut game = Game::init(EntityStore::default(), 10.0, 10.0);

    for _ in 0..10 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    // TODO this isn't great
    game.entity_store.builders.insert(0, BuilderComponent {});

    game.entity_store.create_entity(&EntityType::Base);
    game.entity_store.create_entity(&EntityType::Sugar);

    for i in 0..300 {
        println!("Tick #{}\n{}", i, game);
        game.tick();
    }
}
