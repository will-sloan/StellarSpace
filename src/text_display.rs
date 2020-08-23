use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

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
