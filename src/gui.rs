use std::sync::Arc;

use bevy::prelude::{IVec2, Vec2, Query, Component, Color, Res};
use bevy_ascii_terminal::{Terminal, Tile};
use sark_grids::geometry::GridRect;
use bitflags::bitflags;

use crate::mouse_state::{MouseState, ButtonState};

pub type Id = i32;

#[derive(Component, Default)]
pub struct TermUi {
    pub state: UiState,
    pub mouse: MouseState,
    pub(crate) size: IVec2,
    writes: Vec<Tile>,
    lines: Vec<usize>,
    id: Id,
}

impl TermUi {
    pub fn button(&mut self, label: impl AsRef<str>) -> bool {
        let label = label.as_ref();
        let id = self.get_id();

        let p = self.state.cursor_pos;
        let len = label.chars().count() + 2;

        let rect = GridRect::from_bl(p, [len,1]);
        if let Some(mpos) = self.mouse.pos && rect.contains_point(mpos) {
            self.state.hovered = id;
            if self.state.active == 0 && self.mouse.lmb.contains(ButtonState::HELD) {
                self.state.active = id;
            }
        }

        let (fg,bg) = 
        if self.state.hovered == id {
            if self.state.active == id {
                (Color::ANTIQUE_WHITE,Color::WHITE)
            } else {
                (Color::WHITE, Color::DARK_GRAY)
            }
        } else {
            (Color::BLACK, Color::WHITE)
        };
        for c in label.chars() {
            self.write(c,fg, bg);
        }
        self.lines.push(self.state.cursor_pos.x as usize);

        self.newline();

        let released = self.mouse.lmb.contains(ButtonState::RELEASED);
        let hovered = self.state.hovered == id;
        let active = self.state.active == id;

        self.state.hovered = 0;

        hovered && active && released 
    }

    fn write(&mut self, glyph: char, fg: Color, bg: Color) {
        let tile = Tile {
            glyph,
            fg_color: fg,
            bg_color: bg
        };
        self.writes.push(tile);
        self.state.cursor_pos.x += 1;
    }

    fn get_id(&mut self) -> Id {
        let id = self.id + 1;
        self.id += 1;
        id
    }

    fn on_finish(&mut self) {
        if !self.mouse.lmb.contains(ButtonState::HELD) {
            self.state.active = 0;
        } else if self.state.active == 0 {
            self.state.active = -1;
        }
        self.writes.clear();
        self.lines.clear();
        self.id = 0;
        self.state.cursor_pos = [0,self.size.y - 1].into()
    }

    fn newline(&mut self) -> bool {
        self.state.cursor_pos.y -= 1;
        self.state.cursor_pos.y >= 0
    }
}

#[derive(Default)]
pub struct UiState {
    cursor_pos: IVec2,
    active: Id,
    hovered: Id,
}

pub(crate) fn draw(
    mut q_term: Query<(&mut Terminal, &mut TermUi)>
) {
    for (mut term, mut ui) in &mut q_term {
        let mut writes = ui.writes.iter();
        let mut y = term.height() - 1;
        for line in ui.lines.iter() {
            for x in 0..*line {
                if let Some(next) = writes.next() {
                    term.put_tile([x,y], *next);
                }
            }
            y -= 1;
        }

        if let Some(pos) = ui.mouse.pos {
            term.put_string([0,0], format!("Cursor pos {}", pos));
        }

        ui.on_finish();

    } 
}