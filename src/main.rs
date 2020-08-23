use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseButtonInput,
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};

struct Solid {}

fn map_setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.8, 1.2, 1.2).into()),
            translation: Translation(Vec3::new(0.0, -50.0, 0.2)),
            sprite: Sprite {
                size: Vec2::new(300.0, 300.0),
            },
            ..Default::default()
        })
        .with(Solid {});

    // Future walls
    // commands
    //     .spawn(SpriteComponents {
    //         material: materials.add(Color::rgb(1.8, 1.2, 1.2).into()),
    //         translation: Translation(Vec3::new(0.0, -50.0, 0.2)),
    //         sprite: Sprite {
    //             size: Vec2::new(300.0, 300.0),
    //         },
    //         ..Default::default()
    //     })
    //     .with(Solid {});

    // commands
    //     .spawn(SpriteComponents {
    //         material: materials.add(Color::rgb(1.8, 1.2, 1.2).into()),
    //         translation: Translation(Vec3::new(0.0, 0.0, 0.2)),
    //         sprite: Sprite {
    //             size: Vec2::new(20.0, 300.0),
    //         },
    //         ..Default::default()
    //     })
    //     .with(Solid {});

    // let window = match windows.get_primary() {
    //     Some(a) => a,
    //     None => panic!("No Window!!! How is this Possible!!!"),
    // };
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(map_setup.system());
    }
}

fn text_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("FiraSans-Bold.ttf").unwrap();
    commands
        // texture
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: Text {
                value: "FPS:".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::BLUE,
                },
            },
            ..Default::default()
        });
}

fn text_display_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

pub struct FPSTextPlugin;

impl Plugin for FPSTextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(text_setup.system())
            .add_system(text_display_system.system());
    }
}

// fn text_update_system() {

// }

struct MainDude {
    speed: f32,
}
const GUYSPEED: f32 = 20.0;
const SCALE_FACTOR: f32 = 3.0;
const GUYHEIGHT: f32 = 14.0;
const GUYWIDTH: f32 = 10.0;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(char_setup.system())
            .add_system(animate_sprite_system.system())
            .add_system(char_collision_system.system())
            .add_system(position_mouse_click_system.system());
    }
}

fn char_setup(
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

fn animate_sprite_system(
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

fn char_collision_system(
    mut query_player: Query<(&MainDude, &mut TextureAtlasSprite, &mut Translation)>,
    mut query_walls: Query<(&Solid, &Sprite, &mut Translation)>,
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

fn position_mouse_click_system(
    mut state: ResMut<State>,
    mouse_pos: ResMut<MouseLoc>,
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
#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

struct MouseLoc(Vec2);

fn mouse_movement_updating_system(
    mut mouse_pos: ResMut<MouseLoc>,
    mut state: ResMut<State>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        //println!("Hello");
        mouse_pos.0 = event.position;
    }
}

fn general_setup(mut commands: Commands) {
    //commands.spawn(UiCameraComponents::default());
    commands.spawn(Camera2dComponents::default());
}

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    // The Camera, Background,and Mouse Position Stuff
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(general_setup.system())
            .add_resource(ClearColor(Color::rgb(0.5, 0.20, 0.80)))
            .init_resource::<State>()
            .add_resource(MouseLoc(Vec2::new(0.0, 0.0)))
            .add_system(mouse_movement_updating_system.system());
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(GeneralPlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(MapPlugin)
        .run();
}
