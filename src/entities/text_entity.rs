use std::any;

use macroquad::text::{Font, TextParams, draw_text_ex};

use crate::{component::Position, entity::Entity};

pub struct TextParameters {
    pub font: Option<Font>,
}

pub struct TextEntity {
    pub text: String,
    pub position: Position,
    pub params: TextParameters,
}

impl TextEntity {
    pub fn new(text: &str, position: Position, font: Option<Font>) -> Self {
        Self {
            text: text.to_string(),
            position,
            params: { TextParameters { font } },
        }
    }
}

impl Entity for TextEntity {
    fn render(&self) {
        // Create macroquad TextParams based on entity TextParameters
        let params = TextParams {
            font: self.params.font.as_ref(),
            ..Default::default()
        };
        draw_text_ex(
            &self.text.as_str(),
            self.position.x,
            self.position.y,
            params,
        );
    }

    fn as_any(&mut self) -> &mut dyn any::Any {
        self
    }
}
