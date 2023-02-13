use std::{collections::HashMap, process::exit};

use nannou::{color::white_point::E, prelude::*};

mod consts;
use consts::*;

fn main() {
    nannou::app(model).update(update).run();
}

fn random_signum() -> f32 {
    if random::<i32>() % 2 == 0 {
        -1.
    } else {
        1.
    }
}

#[derive(Debug)]
struct Pie {
    slices: u8,
    velocity: f32,
    pos: Point2,
}

impl Pie {
    fn new() -> Self {
        Self {
            slices: random_range(0, 10),
            velocity: 0.,
            pos: pt2(
                random_f32() * random_signum() * SCREEN_HALF as f32,
                SCREEN_HALF as f32,
            ),
        }
    }
    fn collides(&self, plate: &PiePlate) -> bool {
        let dist = pt2((self.pos.x - plate.x).abs(), (self.pos.y - PLATE_Y).abs());

        //if distance is greater than both dimensions
        if dist.x > (PLATE_W / 2. + PIE_RADIUS) && dist.x > (PLATE_H / 2. + PIE_RADIUS) {
            false
        //if distance is less than plate dimensions
        } else if dist.x <= PLATE_W / 2. && dist.y <= PLATE_H / 2. {
            true
        //handle possibility of corners
        } else {
            let corner_dist_sq_sum =
                (dist.x - PLATE_W / 2.).powf(2.) + (dist.y - PLATE_H / 2.).powf(2.);
            corner_dist_sq_sum <= PIE_RADIUS.powf(2.)
        }
    }
}

struct PiePlate {
    x: f32,
}

struct Model {
    _window: window::Id,
    pies: Vec<Pie>,
    plate: PiePlate,
    digit_idx: usize,
    abs_digit_idx: usize,
    digits: Vec<u8>,
}
impl Model {
    fn load_digits(&mut self) {
        self.digits = reqwest::blocking::get(&format!(
            "https://api.pi.delivery/v1/pi?start={}&numberOfDigits={DIGITS_LOADED_AT_ONCE}",
            self.abs_digit_idx
        ))
        .unwrap()
        .json::<HashMap<String, String>>()
        .unwrap()
        .get("content")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    }

    fn spawn_pie(&mut self) {
        self.pies.push(Pie::new())
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .event(event)
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .build()
        .unwrap();

    let mut model = Model {
        _window,
        pies: vec![],
        plate: PiePlate { x: 0. },
        digit_idx: 0,
        abs_digit_idx: 0,
        digits: vec![],
    };
    model.load_digits();
    model
}

fn update(app: &App, model: &mut Model, update: Update) {
    for pie in model.pies.iter_mut() {
        pie.velocity -= PIE_ACCEL;
        pie.pos.y += pie.velocity;
    }
    model.pies.retain(|pie| {
        let collides = pie.collides(&model.plate);
        if pie.pos.y + PIE_RADIUS > -(SCREEN_HALF as f32) && !collides {
            true
        } else if collides {
            if let Some(digit) = model.digits.get(model.digit_idx) {
                if pie.slices != *digit {
                    //TODO: load game over
                    println!("game over!!");
                    exit(0);
                }

                model.digit_idx += 1;
            }
            false
        } else {
            false
        }
    });
    if app.elapsed_frames() % PIE_SPAWN_RATE == 0 {
        model.spawn_pie();
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            model.plate.x = pos.x;
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    for pie in &model.pies {
        //pie background
        draw.ellipse()
            .radius(PIE_RADIUS)
            .color(PIE_BACKGROUND)
            .x_y(pie.pos.x, pie.pos.y);

        //pie text
        draw.text(&pie.slices.to_string())
            .x_y(pie.pos.x, pie.pos.y)
            .font_size(SLICES_FONT_SIZE)
            .color(WHITE);
    }

    //plate
    draw.rect()
        .w_h(PLATE_W, PLATE_H)
        .x_y(model.plate.x, PLATE_Y)
        .color(PLATE_COLOR);

    draw.text(&model.digits[model.digit_idx].to_string())
        .font_size(50)
        .x(-(SCREEN_HALF as f32) + 50.)
        .y(SCREEN_HALF as f32 - 50.)
        .color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}
