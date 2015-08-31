extern crate sfml;

use sfml::graphics::{ RenderWindow, RenderTarget, RectangleShape, Color };
use sfml::system::{ Vector2f, Clock, Time };
use sfml::traits::Drawable;
use sfml::window::{ ContextSettings, VideoMode, event, Close };
use sfml::window::keyboard::Key;

struct App {
    rotation: f32
}

impl App {
    fn update(&mut self, dt: &Time) {
        self.rotation += 120. * dt.as_seconds();
    }
}

impl Drawable for App {
    fn draw<RT:RenderTarget>(&self, target: &mut RT) {
        let mut square = RectangleShape::new().expect("Error, cannot create rectangle.");
        square.set_position(&Vector2f::new(75., 75.));
        square.set_size(&Vector2f::new(50., 50.));
        square.set_rotation(self.rotation);
        square.set_fill_color(&Color::red());
        square.set_origin(&Vector2f::new(25., 25.));
        target.draw(&square);
    }
}

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "vrally",
                                       Close,
                                       &ContextSettings::default())
                       .expect("Cannot create a new Render Window.");

    let mut app = App { rotation: 0. };
    let mut clock = Clock::new();

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => ()
            }
        }

        app.update(&clock.restart());

        window.clear(&Color::green());
        window.draw(&app);
        window.display();
    }
}
