use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, js_sys, window};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct GameState {
    apple_coords: Vector2,
    head: Head,
    tail: Vec<TailPiece>,
    board_size: Vector2,
    len: u8,
}

struct Head {
    point: Vector2,
    direction: Direction,
}

struct TailPiece {
    point: Vector2,
    lifetime: u16,
}

#[derive(Clone, PartialEq, Eq)]
struct Vector2 {
    x: u32,
    y: u32,
}

#[derive(Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn get_random_int(max: f64) -> u32 {
    js_sys::Math::floor(js_sys::Math::random() * max) as u32
}

impl GameState {
    fn random_empty_pos(&self) -> Vector2 {
        let temp = Vector2 {
            x: get_random_int(self.board_size.x.into()),
            y: get_random_int(self.board_size.y.into()),
        };
        loop {
            let temp = Vector2 {
                x: get_random_int(self.board_size.x.into()),
                y: get_random_int(self.board_size.y.into()),
            };
            if temp == self.apple_coords {
                continue;
            }
            if temp == self.head.point {
                continue;
            }
            if self.tail.iter().any(|tailpiece| tailpiece.point == temp) {
                // if the point of any temp piece is == to temp
                continue;
            }
            break;
        }
        temp
    }
}

impl GameState {
    fn new(w: u32, h: u32) -> Self {
        let mut temp = Self {
            len: 3,
            apple_coords: Vector2 { x: 0, y: 0 },
            head: Head {
                point: Vector2 { x: 0, y: 0 },
                direction: Direction::Down,
            },
            tail: Vec::new(),
            board_size: Vector2 {
                y: (h / BLOCK_SIZE).try_into().unwrap(),
                x: (w / BLOCK_SIZE).try_into().unwrap(),
            },
        };
        temp.apple_coords = temp.random_empty_pos();
        temp.head.point = temp.random_empty_pos();
        temp
    }
}

impl Vector2 {
    fn to_screenspace(&self) -> (u32, u32) {
        ((self.x * BLOCK_SIZE).into(), (self.y * BLOCK_SIZE).into())
    }
}

const BLOCK_SIZE: u32 = 50; // _ by _ px blocks

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("initiated");
    let window = window().expect("the window doesnt exist?");
    let document = window.document().expect("window doesnt have a document");
    let canvas = document
        .get_element_by_id("snake_canvas")
        .expect("the script needs a canvas with the id 'snake_canvas', please create one");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let height = canvas.height();
    let width = canvas.width();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap(); // ungodly type conversion mess, like 3 ways to panic here :3
    // game loop
    let mut state = GameState::new(width, height);
    let a = Closure::<dyn FnMut()>::new(move || game_update(&ctx, width, height, &mut state));
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 500)
        .unwrap();
    fn game_update(
        ctx: &CanvasRenderingContext2d,
        width: u32,
        height: u32,
        mut state: &mut GameState,
    ) {
        ctx.set_fill_style_str("black");
        ctx.fill_rect(0., 0., width as f64, height as f64);
        console_log!("filled_screen");

        let apple = state.apple_coords.clone();
        let (appl_x, appl_y) = apple.to_screenspace();
        ctx.set_fill_style_str("red");
        ctx.fill_rect(
            appl_x as f64,
            appl_y as f64,
            BLOCK_SIZE as f64,
            BLOCK_SIZE as f64,
        );

        match state.head.direction.clone() {
            // movement
            Direction::Left => {
                let mut pointclone = state.head.point.clone();
                if pointclone.x == 0 {
                    pointclone.x = state.board_size.x;
                }
                pointclone.x -= 1;
                state.head.point = pointclone
            }
            Direction::Right => {
                let mut pointclone = state.head.point.clone();
                if pointclone.x == state.board_size.x - 1 {
                    pointclone.x = 0;
                } else {
                    pointclone.x += 1;
                }
                state.head.point = pointclone
            }
            Direction::Up => {
                let mut pointclone = state.head.point.clone();
                if pointclone.y == 0 {
                    pointclone.y = state.board_size.y;
                }
                pointclone.y -= 1;
                state.head.point = pointclone
            }
            Direction::Down => {
                let mut pointclone = state.head.point.clone();
                if pointclone.y == state.board_size.y - 1 {
                    pointclone.y = 0;
                    //state = &mut GameState::new(width.clone(), height.clone());
                } else {
                    pointclone.y += 1;
                }
                state.head.point = pointclone
            }
        };

        let head = state.head.point.clone();
        let (head_x, head_y) = head.to_screenspace();
        ctx.set_fill_style_str("rgb(33, 15, 55)");
        ctx.fill_rect(
            head_x as f64,
            head_y as f64,
            BLOCK_SIZE as f64,
            BLOCK_SIZE as f64,
        );

        ctx.stroke();
    }
    a.forget();
}
