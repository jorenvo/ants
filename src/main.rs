#![deny(clippy::pedantic)]
extern crate rand;

mod components;
mod entity_store;

use entity_store::*;
use rand::Rng;

struct Game {
    entity_store: EntityStore,
    rng: rand::rngs::ThreadRng,
}

impl Game {
    fn move_ants(&mut self) {
        let ants = &self.entity_store.ants;
        for (id, _) in ants {
            if let Some(pos) = self.entity_store.positions.get_mut(id) {
                pos.x += self.rng.gen_range(0, 10);
                pos.y += self.rng.gen_range(0, 10);
            }
        }
    }

    fn move_entities(&mut self) {
        self.move_ants();
    }
}

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
