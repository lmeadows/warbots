mod utils;

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn start() {
    let heights: [f64; 900] = [0f64; 900];
    let terrain: Terrain = Terrain::new(heights);
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

    draw_terrain(&context, terrain);
}

use rand::Rng;
pub fn draw_terrain(context: &web_sys::CanvasRenderingContext2d, terrain: Terrain) {
    let color = JsValue::from(terrain.color());
    context.set_stroke_style(&color);

    let heights = terrain.heights();

    for x in 0..900 {
        context.begin_path();
        context.move_to(x as f64, 500.0);
        context.line_to(x as f64, heights[x]);
        context.stroke();
    }
}

pub struct Terrain {
    heights: [f64; 900],
    color: String,
}

use rand::seq::SliceRandom;
impl Terrain {
    fn new(mut heights: [f64; 900]) -> Terrain {
        const STEP_MAX: f64 = 2.5;
        const STEP_CHANGE: f64 = 1.0;
        // minimum distance from the top of canvas to a mountain peak
        const HEIGHT_MIN: f64 = 30.0;
        // max distance from the top of canvas to a mountain peak
        const HEIGHT_MAX: f64 = 470.0;

        let mut rng = rand::thread_rng();

        // starting conditions
        let y1: f64 = rng.gen();
        let mut terrain_height: f64 = y1 * HEIGHT_MAX;
        let y2: f64 = rng.gen();
        let mut slope: f64 = (y2 * STEP_MAX) * 2.0 - STEP_MAX;

        // create the landscape
        for x in 0..heights.len() {
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

            if terrain_height < HEIGHT_MIN {
                terrain_height = HEIGHT_MIN;
                slope *= -1.0;
            }

            heights[x] = terrain_height;
        }

        let terrain_colors: Vec<String> = vec![
            "#27FF00", "#43AB08", "#9D5109", "#EABC00", "#00960E", "#CCCCCC", "#FFFFFF", "#F7CAA6",
            "#BAEFFF", "#8E4103", "#A50000",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        let color: Vec<_> = terrain_colors
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();

        Terrain {
            heights,
            color: color[0].clone(),
        }
    }

    fn heights(&self) -> [f64; 900] {
        self.heights
    }

    fn color(&self) -> &String {
        &self.color
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
