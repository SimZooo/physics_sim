use egui_macroquad::egui::{self, Key, Pos2, Rect};
use macroquad::input::mouse_position;
use std::{collections::HashMap, sync::Arc};

use crate::{app::App, math::math::Vec2f, renderer::entity::Shape};

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
                        egui::CollapsingHeader::new(format!(
                            "Physics Entities {}",
                            app.app_context.entity_manager.entities.len()
                        ))
                        .show(ui, |ui| {
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
                        ui.checkbox(&mut app.app_context.debug_outlines, "Debug Outlines");
                        ui.checkbox(&mut app.app_context.show_forces, "Forces");
                        ui.checkbox(&mut app.app_context.show_com, "Center of Mass");
                        ui.horizontal(|ui| {
                            ui.label("Current shape:");
                            if ui
                                .add(egui::RadioButton::new(
                                    app.app_context.current_shape == Shape::Circle,
                                    "Circle",
                                ))
                                .clicked()
                            {
                                app.app_context.current_shape = Shape::Circle;
                            };
                            if ui
                                .add(egui::RadioButton::new(
                                    app.app_context.current_shape == Shape::Rectangle,
                                    "Rectangle",
                                ))
                                .clicked()
                            {
                                app.app_context.current_shape = Shape::Rectangle;
                            };
                        });
                        let mouse_pos_sim = app.app_context.get_mouse_position();
                        let mouse_pos_real = mouse_position();
                        ui.label(format!(
                            "Mouse Position: {:.2}, {:.2}. Pixels: {:.2} {:.2}",
                            mouse_pos_sim.x, mouse_pos_sim.y, mouse_pos_real.0, mouse_pos_real.1
                        ));
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
