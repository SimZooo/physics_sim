use macroquad::{color::WHITE, input::MouseButton};
use physics_sim::app::{App, AppContext, WindowParameters};
use physics_sim::renderer::entity::EntityId;

fn spawn_ball_onclick(app_context: &mut AppContext, dt: f32, state: &mut AppState) {
    if state.new_timer <= 0. {
        state.clicked = false;
        state.new_timer = 0.2
    } else if state.clicked {
        state.new_timer -= dt;
        return;
    }
    if app_context.get_button_press(MouseButton::Left) {
        state.clicked = true;
        let mouse_pos = app_context.get_mouse_position();
        app_context.new_entity(mouse_pos, 10., WHITE);
    }
}

pub struct AppState {
    pub balls: Vec<EntityId>,
    pub new_timer: f32,
    pub clicked: bool,
}

#[macroquad::main("Physics Simulator")]
async fn main() {
    env_logger::init();

    let mut app = App::new(
        AppState {
            balls: vec![],
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
