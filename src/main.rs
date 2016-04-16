extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ ContextSettings,
                    VideoMode,
                    event,
                    window_style };
use sfml::window::Key;
use sfml::graphics::{ RenderWindow,
                      RenderTarget,
                      CircleShape,
                      Color,
                      Shape,
                      Texture,
                      Transformable };

mod scene;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "SFML Example",
                                       window_style::CLOSE,
                                       &ContextSettings::default()).expect("Cannot create a new Render Window.");

    let mut circle = CircleShape::new().expect("Error, cannot create ball");
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));

    let resources = scene::start_scene::StartSceneResources {
        background_texture: Texture::new_from_file("data/Images/background.jpg").expect("Cannot create texture!")
    };

    let start_scene = scene::start_scene::StartScene::new(&resources, &window);

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => ()
            }
        }

        window.clear(&Color::new_rgb(0, 200, 200));
        window.draw(&circle);
        window.draw(&start_scene);
        window.display();
    }
}
