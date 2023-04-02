use super::components::Position;

pub struct SnakeProperties {
    pub length: usize,
    pub snake_locations: Vec<Position>
}

impl Default for SnakeProperties {
    fn default() -> Self {
        SnakeProperties { length: 0, snake_locations: Vec::new() }
    }
}

impl SnakeProperties {
    pub fn add_part(self: &mut Self, position: Position) {
        self.length += 1;
        self.snake_locations.push(position);
    }
}

pub struct StaleSnakeProperties {
    pub length: usize,
    pub snake_locations: Vec<Position>
}


impl Default for StaleSnakeProperties {
    fn default() -> Self {
        StaleSnakeProperties { length: 0, snake_locations: Vec::new() }
    }
}

impl StaleSnakeProperties {
    pub fn add_part(self: &mut Self, position: Position) {
        self.length += 1;
        self.snake_locations.push(position);
    }
}
