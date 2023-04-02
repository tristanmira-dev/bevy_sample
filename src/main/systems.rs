use bevy::{prelude::{Commands, Transform, Vec3, OrthographicCameraBundle, Query, Input, KeyCode, Res, Windows, ResMut}, sprite::{SpriteBundle, Sprite}};
use super::{super::init::common::constants::{SNAKE_HEAD_COLOR, ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR, SNAKE_TAIL_COLOR}, resources::{SnakeProperties, StaleSnakeProperties}, components::{SnakeHead, Direction, Position, Size, Food, SnakeTail, SnakePart, PartType}};

pub fn spawn_snake(mut snake_properties: ResMut<SnakeProperties>, mut stale_snake_properties: ResMut<StaleSnakeProperties>, mut commands: Commands) {
    
    snake_properties.add_part(Position { x: 10, y: 10 });
    stale_snake_properties.add_part(Position { x: 10, y: 10 });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..Sprite::default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..Transform::default()
        },
        ..SpriteBundle::default()
    }).insert(SnakeHead {
        current_direction: Direction::Forward
    }).insert(Size::square(0.5))
    .insert(SnakePart {
        part_type: PartType::Head
    });
}

pub fn spawn_tail(mut snake_properties: ResMut<SnakeProperties>, mut stale_snake_properties: ResMut<StaleSnakeProperties>, mut commands: Commands) {
    let position = Position {
        x: snake_properties.snake_locations[snake_properties.length - 1].x,
        y: snake_properties.snake_locations[snake_properties.length - 1].y - 1
    };

    let stale_position = Position {
        x: stale_snake_properties.snake_locations[stale_snake_properties.length - 1].x,
        y: stale_snake_properties.snake_locations[stale_snake_properties.length - 1].y - 1
    };

    snake_properties.add_part(position);
    stale_snake_properties.add_part(stale_position);

    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite { color: SNAKE_TAIL_COLOR, ..Sprite::default() },
            transform: Transform { translation: Vec3::new(0., 0., 0.), ..Transform::default()},
            ..SpriteBundle::default()
    })
    .insert(SnakeTail {
        current_direction: Direction::Forward,
        tail_index: snake_properties.length - 1
    })
    .insert(Size::square(0.3))
    .insert(SnakePart { part_type: PartType::Body});
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn snake_direction(snake_properties: Res<SnakeProperties>, keyboard_input: Res<Input<KeyCode>>, mut head_positions: Query<&mut SnakeHead>) {
    for mut head in head_positions.iter_mut() {
        
        if keyboard_input.pressed(KeyCode::W) {
            if snake_properties.snake_locations.len() > 1 {
                if snake_properties.snake_locations[1].x == snake_properties.snake_locations[0].x {
                    return
                }
            }
            head.current_direction = Direction::Forward;
        } if keyboard_input.pressed(KeyCode::A) {
            if snake_properties.snake_locations.len() > 1 {
                if snake_properties.snake_locations[1].y == snake_properties.snake_locations[0].y {
                    return
                }
            }
            head.current_direction = Direction::Left;
        } if keyboard_input.pressed(KeyCode::S) {
            if snake_properties.snake_locations.len() > 1 {
                if snake_properties.snake_locations[1].x == snake_properties.snake_locations[0].x {
                    return
                }
            }
            head.current_direction = Direction::Back;
        } if keyboard_input.pressed(KeyCode::D) {
            if snake_properties.snake_locations.len() > 1 {
                if snake_properties.snake_locations[1].y == snake_properties.snake_locations[0].y {
                    return
                }
            }
            head.current_direction = Direction::Right;
            
        }
    }
}

pub fn snake_movement(mut snake_properties: ResMut<SnakeProperties>, mut stale_snake_properties: ResMut<StaleSnakeProperties>, mut head_positions: Query<&SnakeHead>) {

    fn orientate_tail(snake_properties: &mut ResMut<SnakeProperties>, stale_snake_properties: &mut ResMut<StaleSnakeProperties>) {

        for i in 1..=snake_properties.length - 1 {
            
            snake_properties.snake_locations[i].y = stale_snake_properties.snake_locations[i-1].y;
            snake_properties.snake_locations[i].x =stale_snake_properties.snake_locations[i-1].x;
   
        }
    }

    fn update_tail(snake_properties: &mut ResMut<SnakeProperties>, stale_snake_properties: &mut ResMut<StaleSnakeProperties>) {

        for i in 0..=stale_snake_properties.length - 1 {
            stale_snake_properties.snake_locations[i].x = snake_properties.snake_locations[i].x;
            stale_snake_properties.snake_locations[i].y = snake_properties.snake_locations[i].y;
        }
    }

    for head in head_positions.iter_mut() { 
        match head.current_direction {
            Direction::Forward => snake_properties.snake_locations[0].y += 1,
            Direction::Back => snake_properties.snake_locations[0].y -= 1, 
            Direction::Left => snake_properties.snake_locations[0].x -= 1,
            Direction::Right => snake_properties.snake_locations[0].x += 1,
        }
        orientate_tail(&mut snake_properties, &mut stale_snake_properties);
        update_tail(&mut snake_properties, &mut stale_snake_properties);
        
    }
}

pub fn food_spawner(windows: Res<Windows>, mut commands: Commands) {
    let window = windows.get_primary().unwrap();

    commands.spawn_bundle(
        SpriteBundle {
            sprite: Sprite { color: FOOD_COLOR, ..Sprite::default()},
            transform: Transform { 
                translation: Vec3::new(
                    (rand::random::<f32>() * ARENA_WIDTH as f32) / ARENA_WIDTH as f32 * window.width() - (window.width() / 2.),
                    (rand::random::<f32>() * ARENA_HEIGHT as f32) / ARENA_HEIGHT as f32 * window.height() - (window.width() / 2.),
                    0.),
                    ..Transform::default()
                },
            ..SpriteBundle::default()
        }
    ).insert(Food).insert(Size::square(0.3));
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();


    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_HEIGHT as f32 * window.width() as f32,
            sprite_size.height / ARENA_WIDTH as f32 * window.height() as f32,
            1.0
        )
    }
}

pub fn head_translation(snake_properties: Res<SnakeProperties>, windows: Res<Windows>, mut q: Query<(&SnakeHead, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        pos / bound_game * bound_window - (bound_window / 2.)
    }


    for (head, mut transform) in q.iter_mut() {

        transform.translation = Vec3::new(
            convert(snake_properties.snake_locations[0].x as f32, window.width(), ARENA_WIDTH as f32),
            convert(snake_properties.snake_locations[0].y as f32, window.height(), ARENA_HEIGHT as f32),
            0.
        )
    }
}


pub fn body_translation(snake_properties: Res<SnakeProperties>, windows: Res<Windows>, mut q: Query<(&SnakeTail, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.)
    }


    for (body, mut transform) in q.iter_mut() {
        
        transform.translation = Vec3::new(
            convert(snake_properties.snake_locations[body.tail_index].x as f32, window.width(), ARENA_WIDTH as f32),
            convert(snake_properties.snake_locations[body.tail_index].y as f32, window.height(), ARENA_HEIGHT as f32),
            0.
        )
    }

}