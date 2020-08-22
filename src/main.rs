use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(animate_sprite_system.system())
        .add_system(collision_system.system())
        .run();
}
struct MainDude {
    speed: f32,
}

struct Solid {}
const GUYSPEED: f32 = 20.0;
const SCALE_FACTOR: f32 = 3.0;
const SPRITEDIMENSIONS: f32 = 16.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/GIMP FIGURES/char_1-Sheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();

    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Camera and Main Character
    commands
        .spawn(Camera2dComponents::default())
        // .spawn(SpriteComponents {
        //     material: materials.add(Color::rgb(0.66, 0.206, 0.245).into()),
        //     sprite: Sprite {
        //         size: Vec2::new(1620.0, 720.0),
        //     },
        //     translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
        //     ..Default::default()
        // })
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            translation: Translation(Vec3::new(-400.0, -400.0, 0.1)),
            scale: Scale(SCALE_FACTOR),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.05))
        .with(MainDude { speed: GUYSPEED });

    // commands
    //     .spawn(Camera2dComponents::default())
    //     .spawn(SpriteComponents {
    //         material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
    //         translation: Translation(Vec3::new(0.0, 0.0, 0.1)),
    //         sprite: Sprite {
    //             size: Vec2::new(30.0, 30.0),
    //         },
    //         ..Default::default()
    //     })
    //     .with(Timer::from_seconds(0.05))
    //     .with(MainDude { speed: GUYSPEED });

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

    // let window = match windows.get_primary() {
    //     Some(a) => a,
    //     None => panic!("No Window!!! How is this Possible!!!"),
    // };

    // Platforms
    // commands.spawn(SpriteComponents {
    //     material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
    //     sprite: Sprite {
    //         size: Vec2::new(20.0, 20.0),
    //     },
    //     translation: Translation(Vec3::new(0.0, -160.0, 0.0)),
    //     ..Default::default()
    // });

    // // Background ?
    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.66, 0.206, 0.245).into()),
        sprite: Sprite {
            size: Vec2::new(1620.0, 720.0),
        },
        translation: Translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
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
    for (mut timer, mut _sprite, mut translation, maindude) in &mut query.iter() {
        //for (mut timer, mut translation, maindude) in &mut query.iter() {
        if timer.finished {
            let window = match windows.get_primary() {
                Some(a) => a,
                None => panic!("No Window!!! How is this Possible!!!"),
            };

            // println!("{}", sprite.draw.is_transparent);
            // Easy way to cover up old code
            // {
            /*
            // if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            //     sprite.index = match sprite.index {
            //         0 => 1,
            //         1 => 2,
            //         2 => 1,
            //         _ => 0,
            //     };
            //     if sprite.index == 1 || sprite.index == 2 {
            //         *translation.x_mut() -= maindude.speed;
            //     }
            // }
            // if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            //     sprite.index = match sprite.index {
            //         5 => 3,
            //         3 => 4,
            //         4 => 3,
            //         _ => 5,
            //     };
            //     if sprite.index == 3 || sprite.index == 4 {
            //         *translation.x_mut() += maindude.speed;
            //     }
            // }
            // if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            //     sprite.index = match sprite.index {
            //         5 => 3,
            //         3 => 4,
            //         4 => 3,
            //         _ => 5,
            //     };
            //     if sprite.index == 3 || sprite.index == 4 {
            //         *translation.y_mut() += maindude.speed;
            //     }
            // }
            // if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            //     sprite.index = match sprite.index {
            //         5 => 3,
            //         3 => 4,
            //         4 => 3,
            //         _ => 5,
            //     };
            //     if sprite.index == 3 || sprite.index == 4 {
            //         *translation.y_mut() -= maindude.speed;
            //     }
            // }
             */
            // }
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
            // let right_side =
            // let left_side =
            // let top =
            // let bottom =

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

            // print!("x: {}", translation.x_mut());
            // println!(" y: {}", &translation.y_mut());
            // else {
            //     sprite.index = match sprite.index {
            //         0..=2 => 0,
            //         _ => 5,
            //     }
            // }
            timer.reset();
        }
    }
}

fn collision_system(
    mut query_player: Query<(&MainDude, &mut TextureAtlasSprite, &mut Translation)>,
    mut query_walls: Query<(&Solid, &Sprite, &mut Translation)>,
) {
    // println!("Hello");
    for (_, char_sprite, mut char_loc) in &mut query_player.iter() {
        //println!("1");
        for (_, wall_sprite, wall_loc) in &mut query_walls.iter() {
            // println!(
            //     "a_pos: {}, a_size: {}, b_pos: {}, b_size: {}",
            //     char_loc.0,
            //     Vec2::new(SPRITEDIMENSIONS, SPRITEDIMENSIONS),
            //     wall_loc.0,
            //     wall_sprite.size,
            // );
            let collision = collide(
                char_loc.0,
                Vec2::new(SPRITEDIMENSIONS, SPRITEDIMENSIONS),
                wall_loc.0,
                wall_sprite.size,
            );
            // println!(
            //     "{} {} {} {}",
            //     char_loc.0,
            //     Vec2::new(SPRITEDIMENSIONS, SPRITEDIMENSIONS),
            //     wall_loc.0,
            //     wall_sprite.size
            // );
            //println!("{}", char_loc.0.x());
            /*
                if (rect1.x < rect2.x + rect2.width &&

                rect1.x + rect1.width > rect2.x &&

                rect1.y < rect2.y + rect2.height &&

                rect1.y + rect1.height > rect2.y) {
                    // collision detected!
                }
            */
            /*
                rect1.x =  char_loc.0.x()
                rect1.y = char_loc.0.y()
                rect1.width && rect1.height = SPRITEDIMENSIONS

                rect2.x = wall_loc.0.x()
                rect2.y = wall_loc.0.y()
                rect2.width = wall_sprite.size.x()
                rect2.height = wall_sprite.size.y()

                if (char_loc.0.x() < wall_loc.0.x() + wall_sprite.size.x() &&

                char_loc.0.x() + SPRITEDIMENSIONS > wall_loc.0.x() &&

                char_loc.0.y() < wall_loc.0.y() + wall_sprite.size.y() &&

                char_loc.0.y() + SPRITEDIMENSIONS > wall_loc.0.y()) {
                    // collision detected!
                }

            */
            // if char_loc.0.x() < wall_loc.0.x() + wall_sprite.size.x()
            //     && char_loc.0.x() + SPRITEDIMENSIONS > wall_loc.0.x()
            //     && char_loc.0.y() < wall_loc.0.y() + wall_sprite.size.y()
            //     && char_loc.0.y() + SPRITEDIMENSIONS > wall_loc.0.y()
            // {
            //     println!("\n\n\nHIT\n\n\n");
            // }

            // if char_loc.0.x() < wall_loc.0.x() + wall_sprite.size.x()
            //     && char_loc.0.x() + SPRITEDIMENSIONS > wall_loc.0.x()
            //     && char_loc.0.y() < wall_loc.0.y() + wall_sprite.size.y()
            //     && char_loc.0.y() + SPRITEDIMENSIONS > wall_loc.0.y()
            // {
            //     println!("\n\n\nHIT\n\n\n");
            // }

            //if (char_loc.0)
            if let Some(collision) = collision {
                //println!("{:?}", collision);
                match collision {
                    Collision::Left => {
                        println!(
                            "LEFT {} {} {}",
                            char_loc.x_mut(),
                            wall_loc.0.x(),
                            SPRITEDIMENSIONS
                        );
                        //println!("Hit left");
                        *char_loc.x_mut() = wall_loc.0.x() - SPRITEDIMENSIONS; //f32::min(wall_loc.0.x() - SPRITEDIMENSIONS, )*char_loc.x_mut() - //SPRITEDIMENSIONS;
                                                                               //wall_sprite.size.x();
                    }
                    Collision::Right => {
                        println!(
                            "RIGHT {} {} {}",
                            char_loc.x_mut(),
                            wall_loc.0.x(),
                            SPRITEDIMENSIONS
                        );
                        //println!("Hit right");
                        *char_loc.x_mut() = wall_loc.0.x() + SPRITEDIMENSIONS; //*char_loc.x_mut() + //SPRITEDIMENSIONS;
                                                                               //wall_sprite.size.x()
                    }
                    Collision::Top => {
                        println!(
                            "TOP {} {} {}",
                            char_loc.y_mut(),
                            wall_loc.0.y(),
                            SPRITEDIMENSIONS
                        );
                        *char_loc.y_mut() = wall_loc.0.y() +//*char_loc.y_mut() + //wall_sprite.size.y();
                     SPRITEDIMENSIONS + wall_sprite.size.y()/2.0;
                    }
                    Collision::Bottom => {
                        println!(
                            "BOTTOM Char Y {} Wall Loc {:?} SPRITE {} CALC {}",
                            char_loc.y_mut(),
                            wall_loc.0,
                            SPRITEDIMENSIONS,
                            wall_loc.0.y() - wall_sprite.size.y() / 2.0 - SPRITEDIMENSIONS
                        );
                        *char_loc.y_mut() =
                            wall_loc.0.y() - wall_sprite.size.y() / 2.0 - SPRITEDIMENSIONS
                        //*char_loc.y_mut() - SPRITEDIMENSIONS;
                        //wall_sprite.size.y();
                    }
                }
            }
        }
    }
}
