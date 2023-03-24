pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    pub fn to_string(&self) -> char {
        match self {
            Direction::UP => '‖',
            Direction::DOWN => '‖',
            Direction::LEFT => '═',
            Direction::RIGHT => '═',
        }
    }
}
