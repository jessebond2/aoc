#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum DirectionHeading {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}
