use macroquad::prelude::*;

struct App {
    start_animation: bool,
    chaikin: bool,
    steps: u32, // from 0 to 7 for animation
    points: Vec<Vec2>,
}

impl App {
    pub fn new() -> Self {
        Self {
            start_animation: false,
            chaikin: true,
            steps: 0,
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        self.points.push(vec2(x, y));
    }

    pub fn clear(&mut self) {
        *self = App::new();
    }

    fn chaikin(&mut self) {
        let length = self.points.len();
        let start = self.points[0];
        let end = self.points[length - 1];
        let mut new_points = vec![start];

        for i in 0..length - 1 {
            let current = self.points[i];
            let next = self.points[i + 1];
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

        self.points = new_points;
    }
}

#[macroquad::main("Interactive Points Example")]
async fn main() {
    let mut app: App = App::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Enter) {
            app.chaikin = true;
            app.start_animation = true;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if !app.start_animation{
            let (x, y) = mouse_position();
            app.add_point(x, y);
            }
        }

        if is_key_pressed(KeyCode::C) {
            app.clear();
        }

        clear_background(BLACK);

        if app.start_animation {
            if app.points.len() >= 2 {
                if app.chaikin {
                    app.chaikin();
                }
                for i in 0..app.points.len() - 1 {
                    let start = app.points[i];
                    let end = app.points[i + 1];
                    draw_line(start.x, start.y, end.x, end.y, 2.0, WHITE);
                }
                app.chaikin = false;
            }
        }

        for point in &app.points {
            draw_circle(point.x, point.y, 2.0, WHITE);
        }

        // Draw UI instructions
        draw_text(
            "Left click to add points | C to clear | ESC to exit",
            20.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Points: {}", app.points.len()),
            20.0,
            50.0,
            20.0,
            WHITE,
        );

        // Wait for next frame
        next_frame().await;
    }
}
