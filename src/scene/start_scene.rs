use scene::*;

struct StartSceneResources {
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
    background_texture: Texture,
    driver_texture: Texture
}

struct StartScene {
    resources: StartSceneResources,
    panel1: Panel,
    panel2: Panel,
    panel3: Panel,
    panel4: Panel
}

impl Scene for StartScene {
    fn process_input() {
        panel1.process_input();
        panel2.process_input();
        panel4.process_input();
    }

    fn do_frame(&self, delta: f64) {
        Sprite::begin();
        Sprite::set_transform(0., 0., Graphics::get_width() / 512., Graphics::get_height() / 256.);
        Sprite::draw(self.resources.background_texture);
        Sprite::reset_transform();
        Sprite::end();

        self.panel1.do_frame(delta);
        self.panel2.do_frame(delta);
        self.panel3.do_frame(delta);
        self.panel4.do_frame(delta);
    }

    fn do_idle_frame(delta: f64) {

    }

    fn device_lost() {

    }

    fn device_reset() {

    }
}
