use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use bat_ui::{MouseState, TerminalUiPlugin, TermUi};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(TerminalUiPlugin)
    .add_startup_system(setup)
    .add_system(ui)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    let term = Terminal::with_size([20,20]);

    commands.spawn((
        TerminalBundle::from(term),
        AutoCamera,
        TermUi::default(),
        ClearAfterRender
    ));
}

fn ui(
    mut q_ui: Query<&mut TermUi>,
    //state: Res<MouseState>,
) {
    for mut ui in &mut q_ui {
        if ui.button("Hello") {
            println!("Button 1");
        }

        if ui.button("How are you?") {
            println!("Button 2");
        }

        if ui.button("Today?") {
            println!("Button 3");
        }
    }
    //let mut term = q_term.single_mut();

    // term.clear();

    // term.put_string([0,0], format!("Pos: {:?}", state.pos));
    // term.put_string([0,1], format!("LMB: {:?}", state.lmb));
}