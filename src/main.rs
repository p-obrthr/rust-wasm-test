use raylib::prelude::*;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 450;

struct GameState {
    score: u32,
}

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("test").build();

    rl.set_target_fps(60);

    let mut state = GameState { score: 0 };

    while !rl.window_should_close() {
        update(&mut state, &rl);

        let mut d = rl.begin_drawing(&thread);
        draw(&state, &mut d);
    }
}

fn update(state: &mut GameState, rl: &RaylibHandle) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        state.score += 1;
    }
}

fn draw(state: &GameState, d: &mut RaylibDrawHandle) {
    d.clear_background(Color::BLACK);

    d.draw_text("wasm", 140, 40, 30, Color::RED);

    let score = format!("Score: {}", state.score);
    d.draw_text(&score, 140, 80, 30, Color::GREEN);

    d.draw_circle(WIDTH / 2, HEIGHT / 2, 10.0, Color::WHITE);
}
