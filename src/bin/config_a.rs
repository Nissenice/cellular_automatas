use cell_automata::automata::{CellAutomata, Config};
use cell_automata::window::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let config = Config {
        num_states: 3,
        win_condition_change_enabled: false,
        perturbation_enabled: false,
        ..Default::default()
    };

    let mut automata = CellAutomata::new(config);
    automata.run().await;
}
