mod utils;

use lazy_static::lazy_static;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

lazy_static! {
    static ref TERRAIN: Terrain = Terrain::new([0f64; 900]);
}

#[wasm_bindgen]
pub fn start() {
    draw_terrain(&canvas_context(), &TERRAIN);

    // The Rust-WASM guide has some examples on how to handle interactivty in their preferred way
    // mio/tokio seem to have incompatibilities with wasm-bindgen, as does the std sleep method.
    // It looks like the wasm-bindgen folks suggest embracing js asynchronicity rather than doing
    // your own timing / event-looping anyway
    // https://rustwasm.github.io/wasm-bindgen/examples/closures.html
    // Will likely also need a way to have mutable, global state on the rust side with this
    // approach:
    // https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
}

use rand::Rng;
pub fn draw_terrain(context: &web_sys::CanvasRenderingContext2d, terrain: &Terrain) {
    let config: Config = Config::new();
    let color = JsValue::from(TERRAIN.color_hex());
    context.set_stroke_style(&color);

    let heights = terrain.heights();

    let left_tank_height = heights[config.tank_left_pos() as usize];
    let right_tank_height = heights[config.tank_right_pos() as usize];
    for i in 0..heights.len() {
        let x = i as f64;
        // make the terrain flat where the tanks sit
        let mut height = heights[i];
        if x >= config.tank_left_pos() && x < config.tank_left_pos() + config.tank_width() {
            height = left_tank_height;
        }
        if x >= config.tank_right_pos() && x < config.tank_right_pos() + config.tank_width() {
            height = right_tank_height;
        }
        if x == config.tank_right_pos() || x == config.tank_left_pos() {
            draw_tank(context, Point::new(x, height));
        }

        context.begin_path();
        context.move_to(x as f64, 500.0);
        context.line_to(x as f64, height);
        context.stroke();
    }
}

pub fn draw_tank(context: &web_sys::CanvasRenderingContext2d, point: Point) {
    let config: Config = Config::new();
    context.set_fill_style(&JsValue::from("#FF0000"));
    context.begin_path();
    context.fill_rect(
        point.x(),
        point.y() - config.tank_height(),
        config.tank_width(),
        config.tank_height(),
    );
}

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}

pub struct Terrain {
    heights: [f64; 900],
    color_hex: String,
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

            heights[x] = terrain_height as f64;
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
            color_hex: color[0].clone(),
        }
    }

    fn heights(&self) -> [f64; 900] {
        self.heights
    }

    fn color_hex(&self) -> &String {
        &self.color_hex
    }
}

pub struct Tank {
    width: f64,
    left_start_position: f64,
    right_start_postion: f64,
}

#[wasm_bindgen]
pub struct Config {
    width: f64,
    height: f64,
    tank_left_pos: f64,
    tank_right_pos: f64,
    tank_width: f64,
    tank_height: f64,
    max_power: u16,
    min_power: u16,
    max_angle: u8,
    min_angle: u8,
}

#[wasm_bindgen]
impl Config {
    pub fn new() -> Config {
        let width: f64 = 900.0;
        let height: f64 = 500.0;
        let tank_height: f64 = 10.0;
        let tank_width: f64 = 10.0;
        let tank_left_pos: f64 = 100.0;
        let tank_right_pos: f64 = 790.0;
        let max_power = 1000;
        let min_power = 0;
        let max_angle = 180;
        let min_angle = 0;

        Config {
            width,
            height,
            tank_height,
            tank_width,
            tank_left_pos,
            tank_right_pos,
            max_power,
            min_power,
            max_angle,
            min_angle,
        }
    }

    pub fn height(&self) -> f64 {
        self.height
    }
    pub fn width(&self) -> f64 {
        self.width
    }
    pub fn tank_width(&self) -> f64 {
        self.tank_width
    }
    pub fn tank_height(&self) -> f64 {
        self.tank_height
    }
    pub fn tank_left_pos(&self) -> f64 {
        self.tank_left_pos
    }
    pub fn tank_right_pos(&self) -> f64 {
        self.tank_right_pos
    }
    pub fn max_power(&self) -> u16 {
        self.max_power
    }
    pub fn min_power(&self) -> u16 {
        self.min_power
    }
    pub fn max_angle(&self) -> u8 {
        self.max_angle
    }
    pub fn min_angle(&self) -> u8 {
        self.min_angle
    }
}

pub struct Projectile {
    point: Point,
    color_hex: String,
}

impl Projectile {
    pub fn new() -> Projectile {
        // TODO: create a projectile from the tank's coordinates
        let point = Point::new(0.0, 0.0);
        let color_hex = String::from("#FF0000");
        Projectile { point, color_hex }
    }
    pub fn point(&self) -> &Point {
        &self.point
    }
    pub fn color_hex(&self) -> &String {
        &self.color_hex
    }
}

#[wasm_bindgen]
pub fn player_fire() {
    // TODO: animate the firing of the projectile here
    //let context = canvas_context();
}

fn canvas_context() -> web_sys::CanvasRenderingContext2d {
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

    context
}
