mod post_processing;
mod scene;

use crate::post_processing::PostProcessingPlugin;
use crate::scene::ScenePlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use std::env::args;

fn main() {
    let mut app = App::new();
    let print_graph = matches!(args().nth(1).as_deref(), Some("--graph"));

    let default_plugins = if print_graph {
        DefaultPlugins.build().disable::<LogPlugin>()
    } else {
        DefaultPlugins
            .set(AssetPlugin {
                watch_for_changes: true,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build()
    };
    app.add_plugins(default_plugins)
        .add_plugin(ScenePlugin)
        .add_plugin(PostProcessingPlugin);

    if print_graph {
        bevy_mod_debugdump::print_render_graph(&mut app);
    } else {
        app.run();
    }
}
