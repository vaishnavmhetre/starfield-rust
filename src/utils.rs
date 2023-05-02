use nannou::{window, App};

pub fn window(app: &App, window_id: window::Id) -> std::cell::Ref<window::Window> {
    app.window(window_id)
        .expect(format!("Failed to find window for {window_id:?}").as_str())
}
