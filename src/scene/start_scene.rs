use scene::Scene;
use sfml::graphics::{RenderTarget,RenderWindow,Sprite,Texture};
use sfml::system::Vector2f;
use sfml::traits::Drawable;

struct Panel;

impl Panel {
    fn process_input(&self) { }
}

impl Drawable for Panel {
    fn draw<RT: RenderTarget>(&self, _: &mut RT) {

    }
}

pub struct StartSceneResources {
    /*
    data_font: Font,
    panel_font: Font,
    button_font: Font,
    menu_button_font: Font,
    start_button_texture: Texture,
    config_button_texture: Texture,
    help_button_texture: Texture,
    exit_button_texture: Texture,
    single_button_texture: Texture,
    campaign_button_texture: Texture,
    save_button_texture: Texture,
    back_button_texture: Texture,
    previous_button_texture: Texture,
    next_button_texture: Texture,
    go_button_texture: Texture,
    */
    pub background_texture: Texture,
    //driver_texture: Texture,
}

pub struct StartScene<'x> {
    panel1: Panel,
    panel2: Panel,
    panel3: Panel,
    panel4: Panel,
    background_sprite: Sprite<'x>
}

impl<'x> StartScene<'x> {
    pub fn new(resources: &'x StartSceneResources, target: &RenderWindow) -> StartScene<'x> {
        let size = target.get_size();
        let mut background_sprite = Sprite::new_with_texture(&resources.background_texture).expect("Cannot create sprite!");
        background_sprite.set_position(&Vector2f::new(0., 0.));
        background_sprite.set_scale(&Vector2f::new((size.x as f32) / 512., (size.y as f32) / 256.));
        StartScene {
            panel1: Panel,
            panel2: Panel,
            panel3: Panel,
            panel4: Panel,
            background_sprite: background_sprite
        }
    }
}

impl<'x> Scene for StartScene<'x> {
    fn process_input(&self) {
        self.panel1.process_input();
        self.panel2.process_input();
        self.panel4.process_input();
    }

    fn device_lost() {

    }

    fn device_reset() {

    }
}

impl<'x> Drawable for StartScene<'x> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        target.draw(&self.background_sprite);
        target.draw(&self.panel1);
        target.draw(&self.panel2);
        target.draw(&self.panel3);
        target.draw(&self.panel4);
    }
}
