#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EntityType {
    Ant,
    Pheromone,
    Base,
    Sugar,
    Wall,
}

pub struct AntEntity {}
pub struct SugarEntity {}
pub struct BaseEntity {}
pub struct WallEntity {}
pub struct PheromoneEntity {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PheromoneType {
    Base,
    Food,
}
