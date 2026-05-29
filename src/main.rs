use macroquad::prelude::*;
use macroquad::window::Conf;

mod automata;
use automata::CellAutomata;

const INIT_SCALE: f32 = 8.0;

fn window_conf() -> Conf {
    let window_width = (automata::WORLD_WIDTH as f32 * INIT_SCALE) as i32;
    let window_height = (automata::WORLD_HEIGHT as f32 * INIT_SCALE) as i32;

    Conf {
        window_title: "Cell Automata".to_owned(),
        window_width,
        window_height,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut automata = CellAutomata::new();
    automata.run().await;
}
