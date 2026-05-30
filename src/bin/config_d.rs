use cell_automata::automata::{CellAutomata, Config};
use cell_automata::window::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let config = Config {
        num_states: 3,
        perturbation_rate: 0.8,
        win_condition_change_rate: 0.05,
        ..Default::default()
    };

    let mut automata = CellAutomata::new(config);
    automata.run().await;
}
