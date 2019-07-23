#![deny(clippy::pedantic)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
extern crate clap;
extern crate rand;

mod components;
mod entities;
mod entity_store;
mod game;
mod utils;

use clap::{App, Arg};
use entities::*;
use entity_store::*;
use game::*;

fn args() -> clap::ArgMatches<'static> {
    App::new("ACO simulator")
        .version("1.0")
        .author("Joren Van Onder <joren@jvo.sh>")
        .arg(
            Arg::with_name("walls")
                .short("w")
                .long("walls")
                .help("Add Deneubourg walls"),
        )
        .get_matches()
}

fn main() {
    let args = args();
    let mut game = Game::init(EntityStore::default(), 5.0, 5.0);

    for _ in 0..1 {
        game.entity_store.create_entity(&EntityType::Ant);
    }

    game.entity_store.create_entity(&EntityType::Base);
    game.entity_store.create_entity(&EntityType::Sugar);

    if args.is_present("walls") {
        game.add_deneubourg_walls();
    }

    for i in 0..300 {
        println!("Tick #{}\n{}", i, game);
        game.tick();
    }
}
