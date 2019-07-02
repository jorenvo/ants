#[derive(PartialEq, PartialOrd, Clone, Ord, Eq, Debug, Default)]
pub struct PositionComponent {
    pub x: u32,
    pub y: u32,
}

#[derive(PartialEq, Debug, Default)]
pub struct EdibleComponent {}