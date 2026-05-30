use cell_automata::automata::{CellAutomata, Config};
use cell_automata::window::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut automata = CellAutomata::new(Config::default());
    automata.run().await;
}
