use macroquad::{
    color::WHITE,
    input::MouseButton,
    text::{Font, load_ttf_font},
};
use physics_sim::{
    app::{App, AppContext, WindowParameters},
    renderer::entity::Entity,
};
use physics_sim::{
    physics::entities::physics_body::PhysicsBody,
    renderer::{component::Position, entity::EntityId},
};

fn spawn_ball_onclick(app_context: &mut AppContext, dt: f32, state: &mut AppState) {
    if state.new_timer <= 0. {
        state.clicked = false;
        state.new_timer = 0.2
    } else if state.clicked {
        state.new_timer -= dt;
        return;
    }
    if AppContext::get_button_press(MouseButton::Left) {
        state.clicked = true;
        let mouse_pos = AppContext::get_mouse_position();
        let physics_body = PhysicsBody::new(mouse_pos, 10.);
        let circle = Entity::new(10., WHITE, Some(physics_body));
        app_context.entity_manager.add(circle);
    }
}

pub struct AppState {
    pub balls: Vec<EntityId>,
    pub font: Font,
    pub new_timer: f32,
    pub clicked: bool,
}

#[macroquad::main("MyGame")]
async fn main() {
    let font = load_ttf_font("assets/DiodrumCyrillic-Regular.ttf")
        .await
        .expect("Failed to load font");

    let mut app = App::new(
        AppState {
            balls: vec![],
            font,
            new_timer: 0.2,
            clicked: false,
        },
        WindowParameters {
            width: 1920,
            height: 1080,
        },
    );

    app.add_system_function(spawn_ball_onclick);
    app.run().await;
}
