use crate::components::*;
use crate::entities::*;
use std::collections::{BTreeMap, HashSet};

pub type EntityIndex = usize;

#[derive(Default)]
pub struct EntityStore {
    pub new_index: EntityIndex,
    pub entity_types: BTreeMap<EntityIndex, EntityType>,

    // Entities
    pub ants: BTreeMap<EntityIndex, AntEntity>,
    pub pheromones: BTreeMap<EntityIndex, PheromoneEntity>,
    pub sugars: BTreeMap<EntityIndex, SugarEntity>,

    // Components
    pub positions: BTreeMap<EntityIndex, PositionComponent>,
    pub positions_lookup: BTreeMap<PositionComponent, HashSet<EntityIndex>>,
    pub edibles: BTreeMap<EntityIndex, EdibleComponent>,
    pub releasing_pheromones: BTreeMap<EntityIndex, ReleasingPheromoneComponent>,
    pub intensities: BTreeMap<EntityIndex, IntensityComponent>,
}

impl EntityStore {
    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    pub fn get_position(&self, id: &EntityIndex) -> Option<&PositionComponent> {
        self.positions.get(id)
    }

    pub fn get_entities_at(&self, search_pos: &PositionComponent) -> Option<&HashSet<EntityIndex>> {
        self.positions_lookup.get(search_pos)
    }

    pub fn update_position(&mut self, id: &EntityIndex, new_pos: &PositionComponent) {
        let old_pos = self.positions.get(&id);
        if let Some(old_pos) = old_pos {
            if let Some(entities) = self.positions_lookup.get_mut(&old_pos) {
                entities.remove(&id);

                if entities.is_empty() {
                    self.positions_lookup.remove(&old_pos);
                }
            }
        }

        self.positions.insert(*id, new_pos.clone());

        if self.positions_lookup.get(new_pos).is_none() {
            self.positions_lookup
                .insert(new_pos.clone(), HashSet::new());
        }

        self.positions_lookup.get_mut(new_pos).unwrap().insert(*id);
    }

    pub fn remove_position(&mut self, id: &EntityIndex) {
        if let Some(pos) = self.get_position(&id) {
            let cloned_pos = pos.clone();
            let entities = self.positions_lookup.get_mut(&cloned_pos).unwrap();
            entities.remove(&id);

            if entities.is_empty() {
                self.positions_lookup.remove(&cloned_pos);
            }
        }

        self.positions.remove(&id);
    }

    pub fn create_entity(&mut self, entity_type: &EntityType) -> EntityIndex {
        let index = self.get_new_index();
        match entity_type {
            EntityType::Ant => {
                self.update_position(&index, &PositionComponent::default());
                self.ants.insert(index, AntEntity {});
            }
            EntityType::Pheromone => {
                self.update_position(&index, &PositionComponent::default());
                self.pheromones.insert(index, PheromoneEntity {});
            }
            EntityType::Sugar => {
                self.update_position(&index, &PositionComponent { x: 5.0, y: 5.0 });
                self.edibles.insert(index, EdibleComponent::default());
                self.sugars.insert(index, SugarEntity {});
            }
        }
        self.entity_types.insert(index, entity_type.clone());

        index
    }
}
