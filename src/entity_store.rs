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
    pub bases: BTreeMap<EntityIndex, BaseEntity>,

    // Components
    positions: BTreeMap<EntityIndex, PositionComponent>,
    positions_lookup: BTreeMap<CoarsePositionComponent, HashSet<EntityIndex>>,
    directions: BTreeMap<EntityIndex, DirectionComponent>,
    pub edibles: BTreeMap<EntityIndex, EdibleComponent>,
    pub releasing_pheromones: BTreeMap<EntityIndex, ReleasingPheromoneComponent>,
    pub intensities: BTreeMap<EntityIndex, IntensityComponent>,
    pub pheromone_types: BTreeMap<EntityIndex, PheromoneType>,
}

impl EntityStore {
    fn get_new_index(&mut self) -> EntityIndex {
        self.new_index += 1;
        self.new_index - 1
    }

    pub fn get_position(&self, id: &EntityIndex) -> Option<&PositionComponent> {
        self.positions.get(id)
    }

    pub fn get_direction(&self, id: &EntityIndex) -> Option<&DirectionComponent> {
        self.directions.get(id)
    }

    pub fn get_entities_at(&self, search_pos: &PositionComponent) -> Option<&HashSet<EntityIndex>> {
        self.positions_lookup
            .get(&CoarsePositionComponent::from(search_pos))
    }

    pub fn get_entity_type_at(
        &self,
        search_pos: &PositionComponent,
        entity_type: &EntityType,
    ) -> Option<EntityIndex> {
        if let Some(ids) = self.get_entities_at(search_pos) {
            for id in ids {
                if self.entity_types.get(&id).unwrap() == entity_type {
                    return Some(*id);
                }
            }
        }

        None
    }

    pub fn update_position(&mut self, id: &EntityIndex, new_pos: &PositionComponent) {
        let old_pos = self.positions.get(&id);
        if let Some(old_pos) = old_pos {
            self.directions.insert(
                *id,
                DirectionComponent {
                    x: new_pos.x - old_pos.x,
                    y: new_pos.y - old_pos.y,
                },
            );

            if let Some(entities) = self
                .positions_lookup
                .get_mut(&CoarsePositionComponent::from(old_pos))
            {
                entities.remove(&id);

                if entities.is_empty() {
                    self.positions_lookup
                        .remove(&CoarsePositionComponent::from(old_pos));
                }
            }
        }

        self.positions.insert(*id, new_pos.clone());

        if self
            .positions_lookup
            .get(&CoarsePositionComponent::from(new_pos))
            .is_none()
        {
            self.positions_lookup
                .insert(CoarsePositionComponent::from(new_pos), HashSet::new());
        }

        self.positions_lookup
            .get_mut(&CoarsePositionComponent::from(new_pos))
            .unwrap()
            .insert(*id);
    }

    pub fn remove_position(&mut self, id: &EntityIndex) {
        if let Some(pos) = self.get_position(&id) {
            let cloned_pos = pos.clone();
            let entities = self
                .positions_lookup
                .get_mut(&CoarsePositionComponent::from(&cloned_pos))
                .unwrap();
            entities.remove(&id);

            if entities.is_empty() {
                self.positions_lookup
                    .remove(&CoarsePositionComponent::from(&cloned_pos));
            }
        }

        self.positions.remove(&id);
        self.directions.remove(&id);
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
                self.update_position(&index, &PositionComponent { x: 8.0, y: 8.0 });
                self.edibles.insert(index, EdibleComponent::default());
                self.sugars.insert(index, SugarEntity {});
            }
            EntityType::Base => {
                self.update_position(&index, &PositionComponent { x: 0.0, y: 0.0 });
                self.bases.insert(index, BaseEntity {});
            }
        }
        self.entity_types.insert(index, entity_type.clone());

        index
    }
}
