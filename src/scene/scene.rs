pub enum GameState {
    Exit,
    Menu,
    PreGame,
    InGame,
    Credits,
    Load,
    Save
}

pub trait Scene {
    fn process_input();
    fn do_frame(delta: f64);
    fn do_idle_frame(delta: f64);
    fn device_lost();
    fn device_reset();
}
