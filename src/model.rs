use nannou::prelude::*;
use rand::{thread_rng, Rng};

use crate::utils::window;

pub struct Universe {
    window_id: window::Id,

    stars: Vec<Star>,
    move_speed: f32,
}

impl Universe {
    pub fn new(window_id: window::Id, stars: Vec<Star>, move_speed: f32) -> Self {
        Self {
            window_id,
            stars,
            move_speed,
        }
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = window_id;
    }

    pub fn stars(&self) -> &[Star] {
        self.stars.as_ref()
    }

    pub fn stars_mut(&mut self) -> &mut Vec<Star> {
        &mut self.stars
    }

    pub fn set_stars(&mut self, stars: Vec<Star>) {
        self.stars = stars;
    }

    pub fn update(&mut self, app: &App, update: Update) {
        let window_id = self.window_id();
        let move_speed = self.move_speed();
        self.stars_mut().into_iter().for_each(|s| {
            s.update(app, update, window_id, move_speed);
        });
    }

    pub fn event(&mut self, app: &App, event: Event) {
        let (width, _) = window(app, self.window_id()).inner_size_points();
        if let Event::WindowEvent {
            id: _,
            simple: Some(window_event),
        } = event
        {
            if let WindowEvent::MouseMoved(position) = window_event {
                let speed = map_range(
                    position.x + (width as f32 / 2.) as f32,
                    0.,
                    width as f32,
                    0.,
                    100.,
                );
                self.set_move_speed(speed);
            }
        }
    }

    pub fn draw(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);

        self.stars().iter().for_each(|s| {
            s.draw(app, &draw, &frame, &self);
        });
        draw.to_frame(app, &frame).unwrap();
    }

    pub fn move_speed(&self) -> f32 {
        self.move_speed
    }

    pub fn set_move_speed(&mut self, move_speed: f32) {
        self.move_speed = move_speed;
    }
}

#[derive(Debug)]
pub struct Star {
    x: f32,
    y: f32,
    z: f32,
    pz: f32,
}

impl Star {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, pz: z }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn pz(&self) -> f32 {
        self.pz
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_pz(&mut self, pz: f32) {
        self.pz = pz;
    }

    pub fn update(&mut self, app: &App, _update: Update, window_id: window::Id, move_speed: f32) {
        let (width, height) = window(app, window_id).inner_size_points();
        self.set_z(self.z() - move_speed);
        if self.z() < 1f32 {
            self.set_x(thread_rng().gen_range(-width..width));
            self.set_y(thread_rng().gen_range(-height..height));
            self.set_z(width);
        }
        self.set_pz(self.z() + move_speed);
    }

    pub fn draw(&self, app: &App, draw: &Draw, _frame: &Frame, universe: &Universe) {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        let pz = self.pz();
        let (width, height) = window(app, universe.window_id()).inner_size_points();

        let sx = map_range(x / z, 0., 1., 0., width);
        let sy = map_range(y / z, 0., 1., 0., height);

        let r = map_range(z, 0., width, 8., 0.);

        // Draw point
        draw.ellipse().x(sx).y(sy).radius(r).color(WHITE);

        let px = map_range(x / pz, 0., 1., 0., width);
        let py = map_range(y / pz, 0., 1., 0., height);

        // Draw trail
        draw.line()
            .start(vec2(px, py))
            .end(vec2(sx, sy))
            .color(WHITESMOKE);
    }
}
