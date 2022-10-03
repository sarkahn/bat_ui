use bevy::{prelude::{IVec2, Component, Query, MouseButton, Input, Res, Vec2, Camera, GlobalTransform, Mat4, Vec3, Image, Assets, Resource, ResMut}, window::Windows};
use bevy_ascii_terminal::Terminal;
use bitflags::bitflags;

use crate::gui::{UiState, TermUi};

bitflags! {
    #[derive(Default)]
    pub struct ButtonState: u32 {
        const NONE = 0;
        const CLICKED = 1 << 0;
        const HELD = 1 << 1;
        const RELEASED = 1 << 2;
    }
}

#[derive(Resource, Default, Clone)]
pub struct MouseState {
    pub lmb: ButtonState,
    pub rmb: ButtonState,
    pub mid: ButtonState,
    pub pos: Option<IVec2>,
}

impl MouseState {
    pub fn reset(&mut self) {
        self.lmb = ButtonState::NONE;
        self.rmb = ButtonState::NONE;
        self.mid = ButtonState::NONE;
        self.pos = None;
    }
}

pub(crate) fn update(
    mut state: ResMut<MouseState>,
    input: Res<Input<MouseButton>>,
    q_cam: Query<(&Camera, &GlobalTransform)>,
    windows: Res<Windows>,
    mut q_ui: Query<(&mut TermUi, &Terminal)>,
) {
    if q_cam.is_empty() {
        return;
    }

    state.reset();
    
    // First available camera we can find
    state.pos = 
    if let Some((cam,t)) = q_cam.iter().next() 
    && let Some(vp) = &cam.viewport
    && let Some(window) = windows.get_primary()
    && let Some(cpos) = window.cursor_position() {
        screen_to_world(
            cpos,
            &cam,
            &t,
            vp.physical_size.as_vec2(),
            vp.physical_position.as_vec2()
        ).map(|p|p.floor().as_ivec2())
    } else {
        None
    };

    state.lmb = update_button_state(&input, MouseButton::Left);
    state.mid = update_button_state(&input, MouseButton::Middle);
    state.rmb = update_button_state(&input, MouseButton::Right);

    for (mut ui,term) in &mut q_ui {
        ui.size = term.size().as_ivec2();
        ui.mouse = state.clone();
        // Convert mouse world position to local terminal position
        if let Some(pos) = ui.mouse.pos.as_mut() {
            *pos = term.from_world(*pos);
        }   
        //println!("State Mouse pos {:?}", ui.mouse.pos);
    }
}

fn update_button_state(
    input: &Input<MouseButton>, 
    button: MouseButton
) -> ButtonState {
    let mut state = ButtonState::NONE;

    if input.just_pressed(button) {
        state|= ButtonState::CLICKED;
    }
    
    if input.pressed(button) {
        state |= ButtonState::HELD;
    } 

    if input.just_released(button) {
        state |= ButtonState::RELEASED;
    }

    state
}


// MIT License
// Copyright (c) 2021 Aevyrie
// https://github.com/aevyrie/bevy_mod_raycast
/// Convert a screen position (IE: The mouse cursor position) to it's corresponding world position.
pub fn screen_to_world(
    screen_pos: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    viewport_size: Vec2,
    viewport_pos: Vec2,
) -> Option<Vec2> {
    let screen_size = viewport_size;
    let screen_pos = (screen_pos - viewport_pos).round();

    let view = camera_transform.compute_matrix();
    let projection = camera.projection_matrix();

    // 2D Normalized device coordinate cursor position from (-1, -1) to (1, 1)
    let cursor_ndc = (screen_pos / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let ndc_to_world: Mat4 = view * projection.inverse();
    let world_to_ndc = projection * view;

    // Calculate the camera's near plane using the projection matrix
    let projection = projection.to_cols_array_2d();
    let camera_near = (2.0 * projection[3][2]) / (2.0 * projection[2][2] - 2.0);

    // Compute the cursor position at the near plane. The bevy camera looks at -Z.
    let ndc_near = world_to_ndc.transform_point3(-Vec3::Z * camera_near).z;
    let cursor_pos_near = ndc_to_world.transform_point3(cursor_ndc.extend(ndc_near));
    let cursor_pos_near = cursor_pos_near.truncate();
    Some(cursor_pos_near)
}
