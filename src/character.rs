use bevy::{
    input::mouse::MouseButtonInput,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[path = "common.rs"]
mod common;

pub struct MainDude {
    speed: f32,
}
const GUYSPEED: f32 = 20.0;
const SCALE_FACTOR: f32 = 3.0;
const GUYHEIGHT: f32 = 14.0;
const GUYWIDTH: f32 = 10.0;

// pub struct CharacterPlugin;

// impl Plugin for CharacterPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_startup_system(char_setup.system())
//             .add_system(animate_sprite_system.system())
//             .add_system(char_collision_system.system())
//             .add_system(position_mouse_click_system.system());
//     }
// }

pub fn char_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/GIMP FIGURES/char_1-Sheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();

    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Camera and Main Character
    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            translation: Translation(Vec3::new(-300.0, -300.0, 0.1)),
            scale: Scale(SCALE_FACTOR),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.05, true))
        .with(MainDude { speed: GUYSPEED });
}

pub fn animate_sprite_system(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &mut Translation,
        &MainDude,
    )>,
) {
    for (timer, mut _sprite, mut translation, maindude) in &mut query.iter() {
        if timer.finished {
            let _window = match windows.get_primary() {
                Some(a) => a,
                None => panic!("No Window!!! How is this Possible!!!"),
            };

            if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
                *translation.x_mut() -= maindude.speed;
                print!("x: {}", translation.x_mut());
                println!(" y: {}", &translation.y_mut());
            }
            if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
                *translation.x_mut() += maindude.speed;
                print!("x: {}", translation.x_mut());
                println!(" y: {}", &translation.y_mut());
            }
            if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
                *translation.y_mut() += maindude.speed;
                print!("x: {}", translation.x_mut());
                println!(" y: {}", &translation.y_mut());
            }
            if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
                *translation.y_mut() -= maindude.speed;
                print!("x: {}", translation.x_mut());
                println!(" y: {}", &translation.y_mut());
            }

            // boundaries
            // *translation.0.x_mut() = f32::max(
            //     -1.0 * ((window.width as f32 + 16.0 * SCALE_FACTOR) / 2.0),
            //     f32::min(
            //         (window.width as f32 - 16.0 * SCALE_FACTOR) / 2.0,
            //         translation.0.x(),
            //     ),
            // );
            // *translation.y_mut() = f32::max(
            //     -1.0 * (window.height as f32 + 16.0 * SCALE_FACTOR) / 2.0,
            //     f32::min(
            //         (window.height as f32 - 16.0 * SCALE_FACTOR) / 2.0,
            //         translation.0.y(),
            //     ),
            // );

            //timer.reset();
        }
    }
}

pub fn char_collision_system(
    mut query_player: Query<(&MainDude, &mut TextureAtlasSprite, &mut Translation)>,
    mut query_walls: Query<(&common::Solid, &Sprite, &mut Translation)>,
) {
    for (_, _char_sprite, mut char_loc) in &mut query_player.iter() {
        for (_, wall_sprite, wall_loc) in &mut query_walls.iter() {
            let collision = collide(
                char_loc.0,
                Vec2::new(GUYWIDTH * SCALE_FACTOR, GUYHEIGHT * SCALE_FACTOR),
                wall_loc.0,
                wall_sprite.size,
            );

            if let Some(collision) = collision {
                println!("{:?}", collision);
                match collision {
                    Collision::Left => {
                        *char_loc.x_mut() = wall_loc.0.x() - wall_sprite.size.x() / 2.0 - GUYWIDTH;
                    }
                    Collision::Right => {
                        *char_loc.x_mut() = wall_loc.0.x() + wall_sprite.size.x() / 2.0 + GUYWIDTH;
                    }
                    Collision::Top => {
                        *char_loc.y_mut() = wall_loc.0.y() + GUYHEIGHT + wall_sprite.size.y() / 2.0;
                    }
                    Collision::Bottom => {
                        *char_loc.y_mut() = wall_loc.0.y() - wall_sprite.size.y() / 2.0 - GUYHEIGHT
                    }
                }
            }
        }
    }
}

pub fn position_mouse_click_system(
    mut state: ResMut<common::State>,
    mouse_pos: ResMut<common::MouseLoc>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    //cursor_moved_events: Res<Events<CursorMoved>>,
    // cursor_movement: Res<Events<CursorMoved>>,
    // mouse_button: Res<Events<MouseButtonInput>>,
    // mut query_player: Query<(&MainDude, &mut TextureAtlasSprite, &mut Translation)>,
) {
    // println!("{:?}", cursor_movement);
    // println!("{:?}", mouse_button);
    //println!("Hello");
    // for (cursor_event, mouse_event) in state
    //     .cursor_moved_event_reader
    //     .iter(&cursor_moved_events)
    //     .zip(
    //         state
    //             .mouse_button_event_reader
    //             .iter(&mouse_button_input_events),
    //     )
    // {
    //     println!("{:?}", cursor_event);
    //     println!("{:?}", mouse_event);
    // }
    // for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
    //     mouse_pos.0 = event.position;
    // }

    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!("event: {:?} position: {:?}", event, mouse_pos.0);
    }
}
