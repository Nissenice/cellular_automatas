use macroquad::window::Conf;

use crate::automata::Config;

const INIT_SCALE: f32 = 8.0;

pub fn window_conf() -> Conf {
    let config = Config::default();
    let window_width = (config.world_width as f32 * INIT_SCALE) as i32;
    let window_height = (config.world_height as f32 * INIT_SCALE) as i32;

    Conf {
        window_title: "Cell Automata".to_owned(),
        window_width,
        window_height,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}
