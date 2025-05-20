use std::{thread, time};

use macroquad::prelude::*;

struct App {
    default_points: Vec<Vec2>,
    chaikin_points: Vec<Vec2>,
    start_animation: bool,
    steps: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            default_points: Vec::new(),
            chaikin_points: Vec::new(),
            start_animation: false,
            steps: 0,
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.default_points.push(vec2(x, y))
    }

    pub fn clear(&mut self) {
        *self = App::new();
    }

    fn chaikin(&mut self) {
        let points = &self.chaikin_points;

        let length = points.len();
        let start = points[0];
        let end = points[length - 1];

        let mut new_points = vec![start];

        for i in 0..length - 1 {
            let current = points[i];
            let next = points[i + 1];
            let dx = next.x - current.x;
            let dy = next.y - current.y;

            let new_1 = Vec2 {
                x: current.x + dx * 0.25,
                y: current.y + dy * 0.25,
            };
            let new_2 = Vec2 {
                x: current.x + dx * 0.75,
                y: current.y + dy * 0.75,
            };

            new_points.push(new_1);
            new_points.push(new_2);
        }

        new_points.push(end);

        self.chaikin_points = new_points;
    }

    fn animate(&mut self) {
        if self.chaikin_points.len() >= 2 {
            for i in 0..self.chaikin_points.len() - 1 {
                let start = self.chaikin_points[i];
                let end = self.chaikin_points[i + 1];
                draw_line(start.x, start.y, end.x, end.y, 2.0, WHITE);
            }

            thread::sleep(time::Duration::from_millis(500));

            self.chaikin();
            
            self.steps += 1;

            if self.steps == 7 {
                self.chaikin_points = self.default_points.clone();
                self.steps = 0;
            }
        } else {
            self.start_animation = false;
        }
    }
}

#[macroquad::main("Interactive Points Example")]
async fn main() {
    let mut app: App = App::new();

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::C) {
            app.clear();
        }

        if is_key_pressed(KeyCode::Enter) {
            if !app.start_animation {
                app.chaikin_points = app.default_points.clone();
            }

            app.start_animation = true;
        }

        if app.start_animation {
            app.animate();
        } else {
            if is_mouse_button_pressed(MouseButton::Left) {
                let (x, y) = mouse_position();
                app.add_point(x, y);
            }
        }

        for point in &app.default_points {
            draw_circle(point.x, point.y, 3.0, WHITE);
            draw_circle(point.x, point.y, 2.0, BLACK);
        }

        // ui instructions
        draw_text(
            "Left click to add points | C to clear | ESC to exit",
            20.0,
            20.0,
            25.0,
            RED,
        );

        draw_text(
            &format!("init Points: {}", app.default_points.len()),
            20.0,
            50.0,
            20.0,
            WHITE,
        );

        draw_text(&format!("steps: {}", app.steps), 20.0, 80.0, 20.0, WHITE);

        next_frame().await;
    }
}
