use wasm_bindgen::prelude::*;

const WIDTH: usize = 40;
const HEIGHT: usize = 12;
const MAX_STEPS: usize = 55;

#[wasm_bindgen]
pub struct RogueEngine {
    width: usize,
    height: usize,
    player_x: usize,
    player_y: usize,
    map: Vec<char>,
    initial_map: Vec<char>,
    steps: usize,
    status: String,
    has_refactor: bool,
}

#[wasm_bindgen]
impl RogueEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RogueEngine {
        let mut map = vec!['.'; WIDTH * HEIGHT];

        // Границы проекта (стены)
        for x in 0..WIDTH {
            map[x] = '#';
            map[(HEIGHT - 1) * WIDTH + x] = '#';
        }
        for y in 0..HEIGHT {
            map[y * WIDTH] = '#';
            map[y * WIDTH + (WIDTH - 1)] = '#';
        }

        // Внутренние перегородки — чтобы карта не была пустой коробкой
        for y in 3..8 {
            map[y * WIDTH + 14] = '#';
        }
        for y in 5..10 {
            map[y * WIDTH + 26] = '#';
        }
        map[4 * WIDTH + 14] = '.';
        map[7 * WIDTH + 26] = '.';

        // Триггеры выгорания
        map[2 * WIDTH + 8]  = 'L';  // Legacy Code
        map[9 * WIDTH + 20] = 'P';  // Toxic PM
        map[5 * WIDTH + 33] = 'B';  // Burnout (босс)

        // Артефакт Refactor — единственный способ победить B
        map[2 * WIDTH + 36] = 'R';

        RogueEngine {
            width: WIDTH,
            height: HEIGHT,
            player_x: 5,
            player_y: 5,
            map: map.clone(),
            initial_map: map,
            steps: 0,
            status: "playing".to_string(),
            has_refactor: false,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        if self.status != "playing" {
            return;
        }

        let nx = self.player_x as i32 + dx;
        let ny = self.player_y as i32 + dy;
        if nx < 0 || ny < 0 { return; }
        let nx = nx as usize;
        let ny = ny as usize;
        if nx >= self.width || ny >= self.height { return; }

        let idx = ny * self.width + nx;
        let tile = self.map[idx];

        if tile == '#' {
            return;
        }

        self.steps += 1;
        self.player_x = nx;
        self.player_y = ny;

        match tile {
            'L' => { self.status = "legacy_crash".into(); }
            'P' => { self.status = "pm_overload".into(); }
            'B' => {
                if self.has_refactor {
                    self.status = "refactored".into();
                } else {
                    self.status = "burnout_boss".into();
                }
            }
            'R' => {
                self.has_refactor = true;
                self.map[idx] = '.';
            }
            _ => {}
        }

        if self.status == "playing" && self.steps >= MAX_STEPS {
            self.status = "exhaustion".into();
        }
    }

    pub fn get_status(&self) -> String {
        self.status.clone()
    }

    pub fn reset(&mut self) {
        self.map = self.initial_map.clone();
        self.player_x = 5;
        self.player_y = 5;
        self.steps = 0;
        self.status = "playing".into();
        self.has_refactor = false;
    }

    pub fn render_frame(&self) -> String {
        let mut view = self.map.clone();
        if self.status == "playing" {
            view[self.player_y * self.width + self.player_x] = '@';
        }

        let energy = if self.steps >= MAX_STEPS { 0 } else {
            100usize.saturating_sub(self.steps * 100 / MAX_STEPS)
        };
        let refactor_mark = if self.has_refactor { "[R]" } else { "[ ]" };

        let mut out = String::with_capacity(self.width * (self.height + 3) + 64);
        out.push_str(&format!(
            " STEP {:02}/{:02}  ·  CPU 99%  ·  ENERGY {:>3}%  ·  REFACTOR {}\n",
            self.steps, MAX_STEPS, energy, refactor_mark
        ));

        for y in 0..self.height {
            for x in 0..self.width {
                out.push(view[y * self.width + x]);
            }
            out.push('\n');
        }
        out.push_str(" [WASD / Arrows] move · [R] refactor · [B] burnout · [Esc] exit");
        out
    }
}