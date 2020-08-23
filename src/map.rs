use bevy::prelude::*;

#[path = "common.rs"]
mod common;

pub fn map_setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(1.8, 1.2, 1.2).into()),
            translation: Translation(Vec3::new(0.0, -50.0, 0.2)),
            sprite: Sprite {
                size: Vec2::new(300.0, 300.0),
            },
            ..Default::default()
        })
        .with(common::Solid {});

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

// pub struct MapPlugin;

// impl Plugin for MapPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_startup_system(map_setup.system());
//     }
// }
