use crate::entities::*;

#[derive(PartialEq, PartialOrd, Clone, Ord, Eq, Debug, Default)]
pub struct PositionComponent {
    pub x: u32,
    pub y: u32,
}

#[derive(PartialEq, Debug, Default)]
pub struct EdibleComponent {}

#[derive(PartialEq, Debug, Default)]
pub struct IntensityComponent {
    pub strength: u8,
}

#[derive(PartialEq, Debug)]
pub struct ReleasingPheromoneComponent {
    pub ticks_left: u32,
    pub ph_type: EntityType,
}
