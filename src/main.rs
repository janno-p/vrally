extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics,
    rotation: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.draw_width / 2) as f64, (args.draw_height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let transform = c.transform.trans(x, y).rot_rad(rotation).trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V2_1;
    let window = Window::new(WindowSettings::new("vrally", [200, 200])
                                            .opengl(opengl)
                                            .exit_on_esc(true))
                        .unwrap();
    let mut app = App { gl: GlGraphics::new(opengl), rotation: 0.0 };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
