use model::{Star, Universe};
use nannou::prelude::*;
use rand::{thread_rng, Rng};
use utils::window;

pub mod model;
pub mod utils;

fn main() {
    nannou::app(model).update(update).event(event).run();
}

fn model(app: &App) -> Universe {
    let window_id = app
        .new_window()
        .view(view)
        .title("Universim")
        .build()
        .unwrap();

    let (width, height) = window(app, window_id).inner_size_points();

    let stars = (1..500)
        .map(|_| {
            Star::new(
                thread_rng().gen_range(-width..width),
                thread_rng().gen_range(-height..height),
                thread_rng().gen_range(0f32..width),
            )
        })
        .collect();

    Universe::new(window_id, stars, 10.)
}

fn update(app: &App, universe: &mut Universe, update: Update) {
    universe.update(app, update)
}

fn event(app: &App, universe: &mut Universe, event: Event) {
    universe.event(app, event)
}

fn view(app: &App, universe: &Universe, frame: Frame) {
    universe.draw(app, frame);
}
