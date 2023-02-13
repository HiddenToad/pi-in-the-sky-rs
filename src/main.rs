use std::{collections::HashMap};
use nannou::prelude::*;

mod consts;
use consts::*;

#[derive(PartialEq)]
enum Scene{
    Start,
    Game,
    GameOver
}

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
    scene: Scene
}
impl Model {
    fn new(_window: window::Id) -> Self{
        let mut model = Model {
            _window,
            pies: vec![],
            plate: PiePlate { x: 0. },
            digit_idx: 0,
            abs_digit_idx: 0,
            digits: vec![],
            scene: Scene::Start
        };
        model.load_digits();
        model
    }

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

    Model::new(_window)
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.scene != Scene::Game{
        return;
    }
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
                    model.scene = Scene::GameOver;
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

fn event(_: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MouseMoved(pos) => {
            model.plate.x = pos.x;
        }
        KeyPressed(key) => {
            match key{
                Key::Return => {
                    match model.scene{
                        Scene::Start | Scene::GameOver => {
                           *model = Model::new(model._window);
                            model.scene = Scene::Game;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn start_view(app: &App, _: &Model, frame: Frame){
    let draw = app.draw();
    frame.clear(WHITE);
    draw.text("Press ENTER to begin!").center_justify().x_y(0., 0.).font_size(30).color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn game_view(app: &App, model: &Model, frame: Frame){
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

    //current digit
    draw.text(&model.digits[model.digit_idx].to_string())
        .font_size(50)
        .x(-(SCREEN_HALF as f32) + 50.)
        .y(SCREEN_HALF as f32 - 50.)
        .color(BLACK);
    draw.to_frame(app, &frame).unwrap();

}
fn game_over_view(app: &App, _: &Model, frame: Frame){
    let draw = app.draw();
    frame.clear(WHITE);
    draw.text("Game over! Press ENTER to retry, or ESC to quit!").center_justify().x_y(0., 0.).font_size(30).color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    (match model.scene{
        Scene::Start => {
            start_view
        },
        Scene::Game => {
            game_view
        },
        Scene::GameOver => {
            game_over_view
        }
    })(app, model, frame)
}
