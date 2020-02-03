mod utils;

use lazy_static::lazy_static;
use std::cell::RefCell;
use std::cmp;
use std::f64;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

lazy_static! {
    static ref CONFIG: Config = Config::new();
    static ref PROJECTILE_POINT: Mutex<Point> = Mutex::new(Point::new(0.0, 0.0));
}

static mut TURN: Option<Turn> = None;
const AUDIO_BUFFER_SIZE: usize = 8192;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    unsafe { TURN = Some(Turn::new()) };
    draw_terrain(0, CONFIG.width as usize);

    let window = web_sys::window().unwrap();

    // FIXME: Hack for requestAnimationFrame loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |i| {
        on_animation_frame(i);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(i32)>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    // FIXME: Hacky key event handler binding
    let onkeyup_handler = Closure::wrap(Box::new(|e: web_sys::KeyboardEvent| {
        on_key(e.key_code(), false);
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    window.set_onkeyup(Some(onkeyup_handler.as_ref().unchecked_ref()));
    onkeyup_handler.forget();

    Ok(())
}

use rand::Rng;
pub fn draw_terrain(min_index: usize, max_index: usize) {
    let turn = unsafe { TURN.as_mut().unwrap() };
    let context = canvas_context();
    let terrain_color = JsValue::from(turn.terrain.color_hex());
    let sky_color = JsValue::from(turn.terrain.sky_color_hex());

    let heights = &turn.terrain.heights;

    let left_tank_height = heights[CONFIG.tank_left_pos() as usize];
    let right_tank_height = heights[CONFIG.tank_right_pos() as usize];
    for i in min_index..max_index {
        let x = i as f64;
        // make the terrain flat where the tanks sit
        let mut height = heights[i];
        if x >= CONFIG.tank_left_pos() && x < CONFIG.tank_left_pos() + CONFIG.tank_width() {
            height = left_tank_height;
        }
        if x >= CONFIG.tank_right_pos() && x < CONFIG.tank_right_pos() + CONFIG.tank_width() {
            height = right_tank_height;
        }

        // draw the line twice to get brighter coloring
        for i in 0..4 {
            // draw the vertical line for the terrain
            context.set_stroke_style(&terrain_color);
            context.begin_path();
            context.move_to(x as f64, 500.0);
            context.line_to(x as f64, height);
            context.stroke();

            // draw the vertical line for the sky
            context.set_stroke_style(&sky_color);
            context.begin_path();
            context.move_to(x as f64, height);
            context.line_to(x as f64, 0.0);
            context.stroke();
        }
    }
    // make sure that tank y-coords are up to date (in case terrain was damaged where tank sits)
    turn.terrain.left_tank.location.y = turn.terrain.heights[CONFIG.tank_left_pos() as usize];
    turn.terrain.right_tank.location.y = turn.terrain.heights[CONFIG.tank_right_pos() as usize];
    // re-draw the tanks at the new locations
    turn.terrain.left_tank.draw();
    turn.terrain.right_tank.draw();
}

pub fn draw_tank(context: &web_sys::CanvasRenderingContext2d, point: Point) {
    context.set_fill_style(&JsValue::from("#FF0000"));
    context.begin_path();
    context.fill_rect(
        point.x(),
        point.y() - CONFIG.tank_height(),
        CONFIG.tank_width(),
        CONFIG.tank_height(),
    );
}

#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub struct Terrain {
    heights: Vec<f64>,
    color_hex: String,
    sky_color_hex: String,
    left_tank: Tank,
    right_tank: Tank,
}

use rand::seq::SliceRandom;
impl Terrain {
    pub fn new() -> Terrain {
        let mut heights: Vec<f64> = Vec::new();
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
        for x in 0..(CONFIG.width as usize) {
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

            heights.push(terrain_height as f64);
        }
        let left_tank: Tank = Tank::new(Point::new(
            CONFIG.tank_left_pos(),
            heights[CONFIG.tank_left_pos() as usize],
        ));
        let right_tank: Tank = Tank::new(Point::new(
            CONFIG.tank_right_pos(),
            heights[CONFIG.tank_right_pos() as usize],
        ));

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

        let sky_colors: Vec<String> = vec!["#000000"].iter().map(|&s| s.into()).collect();

        let sky_color: Vec<_> = sky_colors
            .choose_multiple(&mut rand::thread_rng(), 1)
            .collect();

        Terrain {
            heights,
            color_hex: color[0].clone(),
            sky_color_hex: sky_color[0].clone(),
            left_tank,
            right_tank,
        }
    }

    pub fn heights(&self) -> &Vec<f64> {
        &self.heights
    }

    pub fn color_hex(&self) -> String {
        String::from(self.color_hex.clone())
    }

    pub fn sky_color_hex(&self) -> String {
        String::from(self.sky_color_hex.clone())
    }
}

#[wasm_bindgen]
pub struct Tank {
    width: f64,
    height: f64,
    location: Point,
}

#[wasm_bindgen]
impl Tank {
    pub fn new(point: Point) -> Tank {
        let width = CONFIG.tank_width();
        let height = CONFIG.tank_height();
        let location = point;
        Tank {
            width,
            height,
            location,
        }
    }

    pub fn draw(&self) {
        let context = canvas_context();
        context.set_fill_style(&JsValue::from("#FF0000"));
        context.begin_path();
        context.fill_rect(
            self.location.x,
            self.location.y - CONFIG.tank_height(),
            CONFIG.tank_width(),
            CONFIG.tank_height(),
        );
    }
}

#[wasm_bindgen]
pub struct Turn {
    terrain: Terrain,
    active_tank: Side,
    projectile_in_flight: bool,
    init_point: Option<Point>,
    init_power: f64,
    init_angle: f64,
    // time at which player took a turn
    timestamp: f64,
    audio_context: web_sys::AudioContext,
    audio_buffer: web_sys::AudioBuffer,
    fire_sound: Vec<f32>,
    collision_sound: Vec<f32>,
}

enum Side {
    Left,
    Right,
}

#[wasm_bindgen]
impl Turn {
    pub fn new() -> Turn {
        let terrain = Terrain::new();
        // The left tank is the human, who fires first
        let active_tank = Side::Left;
        let projectile_in_flight = false;
        let init_point = None;
        let init_power = 0.0;
        let init_angle = 0.0;
        let timestamp = 0.0;
        let audio_context = web_sys::AudioContext::new().unwrap();
        let audio_buffer = audio_context
            .create_buffer(
                1,
                (audio_context.sample_rate() * 2.0) as u32,
                audio_context.sample_rate(),
            )
            .unwrap();

        let mut fire_sound: Vec<f32> = Vec::with_capacity(AUDIO_BUFFER_SIZE);
        let mut collision_sound: Vec<f32> = Vec::with_capacity(AUDIO_BUFFER_SIZE);
        for i in 0..AUDIO_BUFFER_SIZE {
            let fire_sound_data = ((i.pow(2) - i.pow(3)) as f32) / ((i.pow(3)) as f32);
            let collision_sound_data = if i / 400 % 5 == 0 { 0.9 } else { -0.9 };
            fire_sound.push(fire_sound_data);
            collision_sound.push(collision_sound_data);
        }

        Turn {
            terrain,
            active_tank,
            projectile_in_flight,
            init_point,
            init_power,
            init_angle,
            timestamp,
            audio_context,
            audio_buffer,
            fire_sound,
            collision_sound,
        }
    }

    pub fn take(&mut self) {
        self.projectile_in_flight = true;
    }

    pub fn end(&mut self) {
        match self.active_tank {
            Side::Left => self.active_tank = Side::Right,
            Side::Right => self.active_tank = Side::Left,
        }
        self.projectile_in_flight = false;
    }
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
    projectile_speed_modifier: f64,
    projectile_size: f64,
    power_normalizer: f64,
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
        let projectile_speed_modifier = 0.75;
        let projectile_size = 3.0;
        let power_normalizer = 200.0;

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
            projectile_speed_modifier,
            projectile_size,
            power_normalizer,
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

fn document() -> web_sys::Document {
    let document = web_sys::window().unwrap().document().unwrap();
    document
}

fn canvas_context() -> web_sys::CanvasRenderingContext2d {
    let canvas = document().get_element_by_id("warbots-canvas").unwrap();
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

pub fn on_animation_frame(timestamp: i32) {
    let mut tank: Option<&Tank> = None;
    let turn = unsafe { TURN.as_mut().unwrap() };

    if !turn.projectile_in_flight {
        return;
    }

    // set the timestamp to now if it not already set
    if turn.timestamp == 0.0 {
        turn.timestamp = timestamp as f64;
    }

    match turn.active_tank {
        Side::Left => tank = Some(&turn.terrain.left_tank),
        Side::Right => tank = Some(&turn.terrain.right_tank),
    }

    let context = canvas_context();
    let mut pp_lock = PROJECTILE_POINT.lock();
    let pp = unsafe { pp_lock.as_mut().unwrap() };

    // re-draw the sky where the projectile was previously
    context.set_fill_style(&JsValue::from(turn.terrain.sky_color_hex()));
    for i in 0..4 {
        context.fill_rect(pp.x, pp.y, CONFIG.projectile_size, CONFIG.projectile_size);
    }

    // draw the projectile where it is now
    let point = get_projectile_position(
        turn.init_point.as_ref().unwrap().x,
        turn.init_point.as_ref().unwrap().y,
        timestamp as f64,
    );

    // check once again that the projectile is in the game area
    if !turn.projectile_in_flight {
        return;
    }

    pp.x = point.x;
    pp.y = point.y;

    if collision(&turn.terrain, pp) {
        turn.projectile_in_flight = false;
        play_audio(&turn.collision_sound);
        mutate_terrain(point.x as usize);
        return;
    }
    context.set_fill_style(&JsValue::from("#FFFFFF"));
    context.fill_rect(pp.x, pp.y, CONFIG.projectile_size, CONFIG.projectile_size);
}

fn collision(terrain: &Terrain, point: &mut std::sync::MutexGuard<Point>) -> bool {
    let x = point.x as usize;
    let y = point.y;
    // terrain collision
    if terrain.heights[x] <= y {
        return true;
    }
    // TODO: tank collision
    false
}

fn mutate_terrain(x: usize) {
    let blast_radius: usize = 30;
    let min_index = cmp::max(0, x - blast_radius);
    let max_index = cmp::min(CONFIG.width as usize, x + blast_radius);
    let turn = unsafe { TURN.as_mut().unwrap() };
    for i in min_index..max_index {
        let x: f64 = i as f64 - min_index as f64 - (blast_radius as f64 / 2.0);
        turn.terrain.heights[i] = turn.terrain.heights[i] + 10.0 + 0.8 * x + (x / 40.0).powi(2);
    }
    draw_terrain(min_index, max_index);
}

fn get_projectile_position(x0: f64, y0: f64, timestamp: f64) -> Point {
    let turn = unsafe { TURN.as_mut().unwrap() };
    let t0 = turn.timestamp;
    let t = CONFIG.projectile_speed_modifier * (timestamp - t0);
    // TODO: take power into account
    let power = turn.init_power / CONFIG.power_normalizer;
    let vy = power * get_angle_rads().sin();
    // multiply by negative 1 to get the correction horizontal direction
    let vx = -1.0 * power * get_angle_rads().cos();
    let a: f64 = -0.001;

    let y = y0 - ((vy * t) + ((0.5) * a) * t.powi(2));
    let x = vx * t + x0;

    // stop processing if the bullet has gone below or beyond the screen
    if y > CONFIG.height || x <= 0.0 || x >= CONFIG.width {
        turn.projectile_in_flight = false;
    }

    Point::new(x, y)
}

fn request_animation_frame(f: &Closure<dyn FnMut(i32)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn on_key(key: u32, state: bool) {
    const KEY_SPACE: u32 = 32;
    const KEY_LEFT: u32 = 37;
    const KEY_UP: u32 = 38;
    const KEY_RIGHT: u32 = 39;
    const KEY_DOWN: u32 = 40;

    match key {
        KEY_SPACE => handle_player_fire_attempt(),
        _ => (),
    };
}

fn handle_player_fire_attempt() {
    // TODO: validate that it's the player's turn, and that he has NOT already fired
    set_ballistics_params(get_power() as f64, get_angle() as f64);
    let turn = unsafe { TURN.as_mut().unwrap() };
    play_audio(&turn.fire_sound);
    turn.take();
}

fn set_ballistics_params(init_power: f64, init_angle: f64) {
    let turn = unsafe { TURN.as_mut().unwrap() };
    turn.init_power = init_power;
    turn.init_angle = init_angle;
    let mut tank_option: Option<&Tank> = None;
    match turn.active_tank {
        Side::Left => tank_option = Some(&turn.terrain.left_tank),
        Side::Right => tank_option = Some(&turn.terrain.right_tank),
    }

    let tank = tank_option.unwrap();
    let x = tank.location.x + tank.width / 2.0;
    let y = tank.location.y - tank.height - 5.0;

    PROJECTILE_POINT.lock().unwrap().x = x;
    PROJECTILE_POINT.lock().unwrap().y = y;
    turn.init_point = Some(Point::new(x, y));
}

fn play_audio(sample: &[f32]) {
    let turn = unsafe { TURN.as_mut().unwrap() };
    let context = &turn.audio_context;
    let buffer = &turn.audio_buffer;

    let source = context.create_buffer_source().unwrap();

    // FIXME: copy_to_channel requires a mutable reference for some reason
    let mut_sample = unsafe { (sample as *const [f32] as *mut [f32]).as_mut().unwrap() };

    buffer.copy_to_channel(mut_sample, 0).unwrap();
    source.set_buffer(Some(&buffer));
    source
        .connect_with_audio_node(&context.destination())
        .unwrap();
    context.resume().unwrap();
    source.start().unwrap();
}

fn get_angle_rads() -> f64 {
    // convert degrees to radians
    (get_angle() as f64) * std::f64::consts::PI / 180.0
}

#[wasm_bindgen(module = "/www/rust-utils.js")]
extern "C" {
    fn get_power() -> u32;
    fn get_angle() -> u32;
}
