use bevy::{prelude::{Plugin, IntoSystemDescriptor, CoreStage}, input::InputSystem};

mod gui;
mod mouse_state;

pub use mouse_state::MouseState;
pub use gui::TermUi;

pub struct TerminalUiPlugin;

impl Plugin for TerminalUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<MouseState>()
        .add_system_to_stage(
            CoreStage::PreUpdate, 
            mouse_state::update.after(InputSystem)
        )
        .add_system_to_stage(
            CoreStage::PostUpdate, 
            gui::draw
        );
    }
}