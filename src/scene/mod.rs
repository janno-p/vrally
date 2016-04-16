use sfml::graphics::Drawable;

/*
pub enum GameState {
    Exit,
    Menu,
    PreGame,
    InGame,
    Credits,
    Load,
    Save
}
*/

pub trait Scene : Drawable {
    fn process_input(&self);
    fn device_lost();
    fn device_reset();
}

pub mod start_scene;
