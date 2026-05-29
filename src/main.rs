use ::rand::Rng;
use ::rand::rngs::ThreadRng;
use macroquad::{
    input::KeyCode::{O, R},
    prelude::*,
    window::Conf,
};
use std::time::Duration;

const FPS: f64 = 60.0;
const WORLD_WIDTH: usize = 300;
const WORLD_HEIGHT: usize = 200;
const INIT_SCALE: f32 = 8.0;
const NUM_STATES: usize = 5;
const NUM_ENEMIES: usize = 2;

const PERTURBATION_RATE: f64 = 0.1;
const WIN_CONDITION_CHANGE_RATE: f64 = 0.05;

struct World {
    width: usize,
    height: usize,
    cells: [usize; WORLD_WIDTH * WORLD_HEIGHT],
    random_permutation: [usize; NUM_STATES],
}

fn window_conf() -> Conf {
    let window_width = (WORLD_WIDTH as f32 * INIT_SCALE) as i32;
    let window_height = (WORLD_HEIGHT as f32 * INIT_SCALE) as i32;

    Conf {
        window_title: "Cell Automata".to_owned(),
        window_width,
        window_height,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

fn step(world: &World, rng: &mut ThreadRng) -> World {
    let s = world.width * world.height;
    let w = world.width;

    let mut next = World {
        width: world.width,
        height: world.height,
        cells: [0; WORLD_WIDTH * WORLD_HEIGHT],
        random_permutation: if rng.gen_bool(WIN_CONDITION_CHANGE_RATE) {
            nudge_random_permutation(&world.random_permutation, rng)
        } else {
            world.random_permutation.clone()
        },
    };

    let win_condition = calc_win_condition(&world.random_permutation);

    for i in 0..s {
        let mut neighbor_count = [0; NUM_STATES];

        neighbor_count[world.cells[(s + i - w - 1) % s]] += 1; // up-left
        neighbor_count[world.cells[(s + i - w) % s]] += 1; // up
        neighbor_count[world.cells[(s + i - w + 1) % s]] += 1; // up-right
        neighbor_count[world.cells[(s + i - 1) % s]] += 1; // left
        neighbor_count[world.cells[(i + 1) % s]] += 1; // right
        neighbor_count[world.cells[(i + w - 1) % s]] += 1; // down-left
        neighbor_count[world.cells[(i + w) % s]] += 1; // down
        neighbor_count[world.cells[(i + w + 1) % s]] += 1; // down-right

        let mut best_enemy = 0;
        let mut best_enemy_count = 0;
        let mut enemy = win_condition[world.cells[i]];
        for _ in 0..NUM_ENEMIES {
            let enemy_count = neighbor_count[enemy];
            if enemy_count > best_enemy_count {
                best_enemy = enemy;
                best_enemy_count = enemy_count;
            }
            enemy = win_condition[enemy];
        }

        let perturbation = if rng.gen_bool(PERTURBATION_RATE) {
            rng.gen_range(0..2)
        } else {
            0
        };
        if best_enemy_count >= 3 + perturbation {
            next.cells[i] = best_enemy;
        } else {
            next.cells[i] = world.cells[i];
        }
    }

    next
}

fn new_world() -> World {
    let mut rng = ::rand::thread_rng();

    let random_permutation = random_permutation(&mut rng);

    World {
        width: WORLD_WIDTH,
        height: WORLD_HEIGHT,
        cells: (0..WORLD_WIDTH * WORLD_HEIGHT)
            .map(|_| rng.gen_range(0..NUM_STATES))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| panic!("Failed to convert cells to array")),
        random_permutation: random_permutation,
    }
}

fn draw_world(world: &World) {
    let width_scale = screen_width() / world.width as f32;
    let height_scale = screen_height() / world.height as f32;

    let (scale, offset_x, offset_y) = if height_scale < width_scale {
        (
            height_scale,
            (screen_width() - world.width as f32 * height_scale) / 2.0,
            0.0,
        )
    } else {
        (
            width_scale,
            0.0,
            (screen_height() - world.height as f32 * width_scale) / 2.0,
        )
    };

    clear_background(BLACK);
    for y in 0..world.height {
        for x in 0..world.width {
            draw_rectangle(
                offset_x + (x as f32 * scale),
                offset_y + (y as f32 * scale),
                scale,
                scale,
                match world.cells[y * world.width + x] {
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

fn wait() {
    std::thread::sleep(Duration::from_secs_f64(1.0 / FPS));
}

fn calc_win_condition(win_condition_raw: &[usize; NUM_STATES]) -> [usize; NUM_STATES] {
    let mut wincon = [0; NUM_STATES];
    for i in 0..NUM_STATES {
        wincon[win_condition_raw[i]] = win_condition_raw[(i + 1) % NUM_STATES];
    }
    wincon
}

fn nudge_random_permutation(
    permutation: &[usize; NUM_STATES],
    rng: &mut ThreadRng,
) -> [usize; NUM_STATES] {
    let mut permutation = permutation.clone();
    let a = rng.gen_range(0..NUM_STATES);
    permutation.swap(a, (a + 1) % NUM_STATES);
    permutation
}

fn random_permutation(rng: &mut ThreadRng) -> [usize; NUM_STATES] {
    let mut permutation = [0; NUM_STATES];
    for i in 0..NUM_STATES {
        permutation[i] = i;
    }
    for i in (1..NUM_STATES).rev() {
        let j = rng.gen_range(0..=i);
        permutation.swap(i, j);
    }
    permutation
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = new_world();
    let mut rng = ::rand::thread_rng();

    loop {
        clear_background(BLACK);
        draw_world(&world);
        next_frame().await;
        world = step(&world, &mut rng);
        wait();
    }
}
