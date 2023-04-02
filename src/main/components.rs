use bevy::prelude::Component;
use bevy::ecs::component::TableStorage;

pub enum PartType {
    Head,
    Body
}


#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    type Storage = TableStorage;
}

pub enum Direction {
    Forward,
    Left,
    Right,
    Back
}

pub struct SnakeHead {
    pub current_direction: Direction 
}

impl Component for SnakeHead {
    type Storage = TableStorage;
}

pub struct SnakeTail {
    pub current_direction: Direction,
    pub tail_index: usize
}

impl Component for SnakeTail {
    type Storage = TableStorage;
}

pub struct Size {
    pub width: f32,
    pub height: f32
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x
        }
    }
}

impl Component for Size {
    type Storage = TableStorage;
}


pub struct SnakePart {
    pub part_type: PartType
}

impl Component for SnakePart {
    type Storage = TableStorage;
}

pub struct Food;

impl Component for Food {
    type Storage = TableStorage;
}




