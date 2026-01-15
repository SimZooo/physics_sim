use egui_macroquad::egui::{self, Key, Pos2, Rect};
use std::{collections::HashMap, sync::Arc};

use crate::{app::App, math::math::Vec2f};

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

pub struct UiManager {}

impl UiManager {
    pub fn render_ui<S>(app: &mut App<S>) {
        egui_macroquad::ui(|ctx| {
            let egui_rect = egui::SidePanel::new(egui::panel::Side::Left, "Scene")
                .resizable(true)
                .show(ctx, |ui| {
                    // Set fonts
                    let mut fonts = egui::FontDefinitions::default();
                    fonts.font_data.insert(
                        "diodrum".to_owned(),
                        Arc::new(egui::FontData::from_static(include_bytes!(
                            "../../assets/DiodrumCyrillic-Regular.ttf"
                        ))),
                    );
                    fonts
                        .families
                        .entry(egui::FontFamily::Monospace)
                        .or_default()
                        .insert(0, "diodrum".to_owned());
                    ctx.set_fonts(fonts);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Physics Simulation").size(20.));
                        if ui
                            .add(egui::ImageButton::new(egui_macroquad::egui::Image::new(
                                egui::include_image!("../../assets/play-button.png"),
                            )))
                            .clicked()
                        {
                            app.paused = !app.paused;
                        }
                    });
                    egui::ScrollArea::new([false, true]).show(ui, |ui| {
                        egui::CollapsingHeader::new("Physics Entities").show(ui, |ui| {
                            app.app_context
                                .entity_manager
                                .entities
                                .iter()
                                .enumerate()
                                .for_each(|(i, (_, e))| {
                                    egui::CollapsingHeader::new(format!("Entity {}", i)).show(
                                        ui,
                                        |ui| {
                                            let physics_body = &e.physics_body;
                                            let position = physics_body.position;
                                            let velocity = physics_body.velocity;
                                            let acceleration = physics_body.acceleration;
                                            ui.label(format!(
                                                "Position: {:.2}, {:.2}",
                                                position.x, position.y
                                            ));
                                            ui.label(format!(
                                                "Velocity: {:.2}, {:.2}",
                                                velocity.x, velocity.y
                                            ));
                                            ui.label(format!(
                                                "Acceleration: {:.2}, {:.2}",
                                                acceleration.x, acceleration.y
                                            ));
                                        },
                                    );
                                });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Pixels Per Unit:");
                            ui.add(egui::Slider::new(&mut app.app_context.ppu, 0.1..=1000.));
                        });
                        ui.checkbox(&mut app.app_context.debug_outlines, "Debug Outlines");
                    });
                    app.app_context.ui_wants_keyboard = ctx.wants_keyboard_input();
                    app.app_context.ui_wants_pointer = ctx.wants_pointer_input();
                })
                .response
                .rect;

            app.app_context.side_panel_left_rect = egui_rect;
        });
    }
}
