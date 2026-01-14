use macroquad::text::load_ttf_font;
use physics_sim::{
    app::{App, AppContext},
    component::Position,
    entities::text_entity::TextEntity,
    entity::EntityId,
};

fn update_text(app: &mut AppContext, dt: f32, state: &mut AppState) {
    let mut text_entity = app
        .entity_manager
        .get_entity_mut(&state.text_id)
        .expect("Failed to get text_entity")
        .as_any()
        .downcast_mut::<TextEntity>()
        .expect("Not a TextEntity");

    text_entity.position.x += dt * 100.;
    text_entity.position.y += dt * 100.;
}

pub struct AppState {
    pub text_id: EntityId,
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut app = App::new(AppState {
        text_id: EntityId(0),
    });

    let font = load_ttf_font("assets/DiodrumCyrillic-Regular.ttf")
        .await
        .expect("Failed to load font");
    let text_entity = TextEntity::new("Hello, World!", Position { x: 400., y: 400. }, Some(font));
    let text_id = app.app_context.entity_manager.add(Box::new(text_entity));
    app.state.text_id = text_id;

    app.add_system_function(update_text);

    app.run().await;
}
