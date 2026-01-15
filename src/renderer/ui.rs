use std::collections::HashMap;

use macroquad::{
    color::WHITE,
    text::{Font, TextParams, draw_text, draw_text_ex},
};

use crate::math::math::Vec2f;

pub struct TextMetadata {
    pub text: String,
    pub position: Vec2f,
}

pub enum UiElementType {
    Text(TextMetadata),
}

pub struct UiElement {
    pub element_type: UiElementType,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct UiId(usize);

pub struct UiManager {
    pub font: Option<Font>,
    curr_id: usize,
    pub elements: HashMap<UiId, UiElement>,
}

impl UiManager {
    pub fn init() -> Self {
        Self {
            font: None,
            curr_id: 0,
            elements: HashMap::new(),
        }
    }

    pub fn add(&mut self, element: UiElement) -> UiId {
        let ui_id = self.new_ui_id();
        self.elements.insert(ui_id, element);
        ui_id
    }

    fn new_ui_id(&mut self) -> UiId {
        let ui_id = UiId(self.curr_id);
        self.curr_id += 1;
        ui_id
    }

    pub fn render(&self) {
        for el in self.elements.values() {
            match &el.element_type {
                UiElementType::Text(s) => match &self.font {
                    Some(f) => {
                        draw_text_ex(
                            s.text.as_str(),
                            s.position.x,
                            s.position.y,
                            TextParams {
                                font: self.font.as_ref(),
                                ..Default::default()
                            },
                        );
                    }
                    None => {
                        draw_text(s.text.as_str(), s.position.x, s.position.y, 10., WHITE);
                    }
                },
            }
        }
    }
}
