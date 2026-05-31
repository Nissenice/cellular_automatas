use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use macroquad::prelude::*;
use std::time::Duration;

const FPS: f64 = 30.0;

pub struct Config {
    pub world_width: usize,
    pub world_height: usize,
    pub num_states: usize,
    pub num_enemies: usize,
    pub perturbation_rate: f64,
    pub perturbation_enabled: bool,
    pub win_condition_change_rate: f64,
    pub win_condition_change_enabled: bool,
    pub repopulate_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            world_width: 300,
            world_height: 200,
            num_states: 5,
            num_enemies: 0,
            perturbation_rate: 0.2,
            perturbation_enabled: true,
            win_condition_change_rate: 0.02,
            win_condition_change_enabled: true,
            repopulate_enabled: true,
        }
    }
}

struct World {
    cells: Vec<usize>,
    random_permutation: Vec<usize>,
    state_count: Vec<usize>,
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
        let mut config = config;
        if config.num_enemies >= config.num_states || config.num_enemies == 0 {
            config.num_enemies = (config.num_states - 1) / 2;
        }

        CellAutomata {
            world: World {
                cells: vec![0; config.world_width * config.world_height],
                random_permutation: random_permutation,
                state_count: vec![0; config.num_states],
            },
            rng,
            config,
        }
    }

    fn step(&mut self) {
        let mut next = World {
            cells: vec![0; self.config.world_width * self.config.world_height],
            random_permutation: if self.config.win_condition_change_enabled
                && self.rng.gen_bool(self.config.win_condition_change_rate)
            {
                Self::nudge_random_permutation(
                    &self.world.random_permutation,
                    &mut self.rng,
                    self.config.num_states,
                )
            } else {
                self.world.random_permutation.clone()
            },
            state_count: vec![0; self.config.num_states],
        };

        let win_condition = Self::calc_win_condition(&self.world.random_permutation);

        let s = self.config.world_width * self.config.world_height;
        let w = self.config.world_width;

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

            let perturbation = if self.config.perturbation_enabled
                && self.rng.gen_bool(self.config.perturbation_rate)
            {
                self.rng.gen_range(1..3)
            } else {
                0
            };
            if best_enemy_count >= 3 + perturbation {
                next.cells[i] = best_enemy;
            } else {
                next.cells[i] = self.world.cells[i];
            }
            next.state_count[next.cells[i]] += 1;
        }

        self.world = next;
        if self.config.repopulate_enabled {
            self.repopulate();
        }
    }

    fn draw_world(&self) {
        let width_scale = screen_width() / self.config.world_width as f32;
        let height_scale = screen_height() / self.config.world_height as f32;

        let (scale, offset_x, offset_y) = if height_scale < width_scale {
            (
                height_scale,
                (screen_width() - self.config.world_width as f32 * height_scale) / 2.0,
                0.0,
            )
        } else {
            (
                width_scale,
                0.0,
                (screen_height() - self.config.world_height as f32 * width_scale) / 2.0,
            )
        };

        clear_background(BLACK);
        for y in 0..self.config.world_height {
            for x in 0..self.config.world_width {
                draw_rectangle(
                    offset_x + (x as f32 * scale),
                    offset_y + (y as f32 * scale),
                    scale,
                    scale,
                    match self.world.cells[y * self.config.world_width + x] {
                        0 => PINK,
                        1 => BLUE,
                        2 => DARKBLUE,
                        3 => PURPLE,
                        4 => YELLOW,
                        5 => SKYBLUE,
                        6 => GOLD,
                        7 => ORANGE,
                        8 => DARKPURPLE,
                        9 => VIOLET,
                        10 => MAGENTA,
                        _ => BLACK,
                    },
                );
            }
        }
    }

    pub async fn run(&mut self) {
        for _ in 0..100 {
            let idx = self
                .rng
                .gen_range(0..self.config.world_width * self.config.world_height);
            let state = self.rng.gen_range(0..self.config.num_states);
            self.add_ball(idx, state, 20);
        }
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

    fn calc_win_condition(win_condition_raw: &Vec<usize>) -> Vec<usize> {
        let mut wincon = vec![0; win_condition_raw.len()];
        for i in 0..win_condition_raw.len() {
            wincon[win_condition_raw[i]] = win_condition_raw[(i + 1) % win_condition_raw.len()];
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

    fn repopulate(&mut self) {
        for state in 0..self.config.num_states {
            let count = self.world.state_count[state];
            if count < 20 {
                let i = self
                    .rng
                    .gen_range(0..self.config.world_width * self.config.world_height);
                self.add_ball(i, state, 10);
            }
        }
    }

    fn add_ball(&mut self, i: usize, state: usize, rad: usize) {
        let i = i as i32;
        let w = self.config.world_width as i32;
        let h = self.config.world_height as i32;

        let r = rad as i32;
        let x0 = i % w;
        let y0 = i / w;

        for dy in 0..=(2 * r + 1) {
            for dx in 0..=(2 * r + 1) {
                if (dx - r) * (dx - r) + (dy - r) * (dy - r) > r * r {
                    continue;
                }
                let x = (x0 + dx) % w;
                let y = (y0 + dy) % h;

                let idx = (y * w + x) as usize;

                self.world.cells[idx] = state;
            }
        }
    }
}
