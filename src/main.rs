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
use components::*;
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
    const WIDTH: f64 = 10.0;
    const HEIGHT: f64 = 10.0;

    let args = args();
    let mut game = Game::init(EntityStore::default(), WIDTH, HEIGHT);

    for i in 0..50 {
        let index = game.entity_store.create_entity(&EntityType::Ant);
        game.entity_store.update_position(
            &index,
            &PositionComponent {
                x: (0.5 + i as f64) % WIDTH,
                y: HEIGHT / 2.0,
            },
        );
    }

    let index = game.entity_store.create_entity(&EntityType::Base);
    game.entity_store.update_position(
        &index,
        &PositionComponent {
            x: 0.5,
            y: HEIGHT / 2.0,
        },
    );

    let index = game.entity_store.create_entity(&EntityType::Sugar);
    game.entity_store.update_position(
        &index,
        &PositionComponent {
            x: WIDTH - 0.5,
            y: HEIGHT / 2.0,
        },
    );

    if args.is_present("walls") {
        game.add_deneubourg_walls();
    }

    for i in 0..300 {
        println!("Tick #{}\n{}", i, game);
        game.tick();
    }
}
