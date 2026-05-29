use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use macroquad::prelude::*;
use std::time::Duration;

const FPS: f64 = 60.0;

pub struct Config {
    pub world_width: usize,
    pub world_height: usize,
    pub num_states: usize,
    pub num_enemies: usize,
    pub perturbation_rate: f64,
    pub win_condition_change_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            world_width: 300,
            world_height: 200,
            num_states: 5,
            num_enemies: 2,
            perturbation_rate: 0.1,
            win_condition_change_rate: 0.05,
        }
    }
}

struct World {
    width: usize,
    height: usize,
    cells: Vec<usize>,
    random_permutation: Vec<usize>,
}

pub struct CellAutomata {
    world: World,
    rng: ThreadRng,
    config: Config,
}

impl CellAutomata {
    pub fn new(config: Config) -> Self {
        let mut rng = ::rand::thread_rng();
        let random_permutation = Self::random_permutation(&mut rng, config.num_states);

        CellAutomata {
            world: World {
                width: config.world_width,
                height: config.world_height,
                cells: (0..config.world_width * config.world_height)
                    .map(|_| rng.gen_range(0..config.num_states))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap_or_else(|_| panic!("Failed to convert cells to array")),
                random_permutation: random_permutation,
            },
            rng,
            config,
        }
    }

    fn step(&mut self) {
        let s = self.world.width * self.world.height;
        let w = self.world.width;

        let mut next = World {
            width: self.world.width,
            height: self.world.height,
            cells: vec![0; self.config.world_width * self.config.world_height],
            random_permutation: if self.rng.gen_bool(self.config.win_condition_change_rate) {
                Self::nudge_random_permutation(
                    &self.world.random_permutation,
                    &mut self.rng,
                    self.config.num_states,
                )
            } else {
                self.world.random_permutation.clone()
            },
        };

        let win_condition =
            Self::calc_win_condition(&self.world.random_permutation, self.config.num_states);

        for i in 0..s {
            let mut neighbor_count = vec![0; self.config.num_states];

            neighbor_count[self.world.cells[(s + i - w - 1) % s]] += 1; // up-left
            neighbor_count[self.world.cells[(s + i - w) % s]] += 1; // up
            neighbor_count[self.world.cells[(s + i - w + 1) % s]] += 1; // up-right
            neighbor_count[self.world.cells[(s + i - 1) % s]] += 1; // left
            neighbor_count[self.world.cells[(i + 1) % s]] += 1; // right
            neighbor_count[self.world.cells[(i + w - 1) % s]] += 1; // down-left
            neighbor_count[self.world.cells[(i + w) % s]] += 1; // down
            neighbor_count[self.world.cells[(i + w + 1) % s]] += 1; // down-right

            let mut best_enemy = 0;
            let mut best_enemy_count = 0;
            let mut enemy = win_condition[self.world.cells[i]];
            for _ in 0..self.config.num_enemies {
                let enemy_count = neighbor_count[enemy];
                if enemy_count > best_enemy_count {
                    best_enemy = enemy;
                    best_enemy_count = enemy_count;
                }
                enemy = win_condition[enemy];
            }

            let perturbation = if self.rng.gen_bool(self.config.perturbation_rate) {
                self.rng.gen_range(0..2)
            } else {
                0
            };
            if best_enemy_count >= 3 + perturbation {
                next.cells[i] = best_enemy;
            } else {
                next.cells[i] = self.world.cells[i];
            }
        }

        self.world = next;
    }

    fn draw_world(&self) {
        let width_scale = screen_width() / self.world.width as f32;
        let height_scale = screen_height() / self.world.height as f32;

        let (scale, offset_x, offset_y) = if height_scale < width_scale {
            (
                height_scale,
                (screen_width() - self.world.width as f32 * height_scale) / 2.0,
                0.0,
            )
        } else {
            (
                width_scale,
                0.0,
                (screen_height() - self.world.height as f32 * width_scale) / 2.0,
            )
        };

        clear_background(BLACK);
        for y in 0..self.world.height {
            for x in 0..self.world.width {
                draw_rectangle(
                    offset_x + (x as f32 * scale),
                    offset_y + (y as f32 * scale),
                    scale,
                    scale,
                    match self.world.cells[y * self.world.width + x] {
                        0 => PINK,
                        1 => BLUE,
                        2 => DARKBLUE,
                        3 => PURPLE,
                        4 => YELLOW,
                        5 => ORANGE,
                        6 => DARKBLUE,
                        7 => GREEN,
                        8 => DARKPURPLE,
                        9 => BROWN,
                        10 => BEIGE,
                        _ => RED,
                    },
                );
            }
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLACK);
            self.draw_world();
            next_frame().await;
            self.step();
            Self::wait();
        }
    }

    fn wait() {
        std::thread::sleep(Duration::from_secs_f64(1.0 / FPS));
    }

    fn calc_win_condition(win_condition_raw: &Vec<usize>, num_states: usize) -> Vec<usize> {
        let mut wincon = vec![0; num_states];
        for i in 0..num_states {
            wincon[win_condition_raw[i]] = win_condition_raw[(i + 1) % num_states];
        }
        wincon
    }

    fn nudge_random_permutation(
        permutation: &Vec<usize>,
        rng: &mut ThreadRng,
        num_states: usize,
    ) -> Vec<usize> {
        let mut permutation = permutation.clone();
        let a = rng.gen_range(0..num_states);
        permutation.swap(a, (a + 1) % num_states);
        permutation
    }

    fn random_permutation(rng: &mut ThreadRng, num_states: usize) -> Vec<usize> {
        let mut permutation = vec![0; num_states];
        for i in 0..num_states {
            permutation[i] = i;
        }
        for i in (1..num_states).rev() {
            let j = rng.gen_range(0..=i);
            permutation.swap(i, j);
        }
        permutation
    }
}
