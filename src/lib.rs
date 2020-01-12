mod utils;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("warbots-canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    draw_terrain(&context);
}

use rand::Rng;
pub fn draw_terrain(context: &web_sys::CanvasRenderingContext2d) {
    const STEP_MAX: f64 = 2.5;
    const STEP_CHANGE: f64 = 1.0;
    const HEIGHT_MAX: f64 = 500.0;

    let mut rng = rand::thread_rng();

    // starting conditions
    let y1: f64 = rng.gen();
    let mut terrain_height: f64 = y1 * HEIGHT_MAX;
    let y2: f64 = rng.gen();
    let mut slope: f64 = (y2 * STEP_MAX) * 2.0 - STEP_MAX;

    let c = JsValue::from(String::from("#0000FF"));
    context.set_stroke_style(&c);

    // create the landscape
    for x in 0..900 {
        // change height and slope
        terrain_height += slope;
        let y3: f64 = rng.gen();
        slope += (y3 * STEP_CHANGE) * 2.0 - STEP_CHANGE;

        // clip height and slope to maximum
        if slope > STEP_MAX {
            slope = STEP_MAX;
        }

        if slope < -1.0 * STEP_MAX {
            slope = -1.0 * STEP_MAX;
        }

        if terrain_height > HEIGHT_MAX {
            terrain_height = HEIGHT_MAX;
            slope *= -1.0;
        }

        if terrain_height < 0.0 {
            terrain_height = 0.0;
            slope *= -1.0;
        }
        // draw column
        context.begin_path();
        context.move_to(x as f64, HEIGHT_MAX);
        context.line_to(x as f64, terrain_height);
        context.stroke();
    }
}

#[wasm_bindgen]
pub struct Config {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Config {
    pub fn new() -> Config {
        let width = 900;
        let height = 500;

        Config { width, height }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}
