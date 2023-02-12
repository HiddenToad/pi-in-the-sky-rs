use nannou::prelude::*;

type Color = Srgb<u8>;

const SCREEN_SIZE: u32 = 1000;
const SCREEN_HALF: u32 = SCREEN_SIZE / 2;
const PIE_RADIUS: f32 = 50.;
const PIE_BACKGROUND: Color = GRAY;
const PIE_ACCEL: f32 = 0.1;
const PIE_SPAWN_RATE: u64 = 45;

fn main() {
    nannou::app(model).update(update).run();
}

fn random_signum() -> f32{
    if random::<i32>() % 2 == 0{
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
            pos: pt2(random_f32() * random_signum() * SCREEN_HALF as f32, SCREEN_HALF as f32),
        }
    }
}


struct Model {
    _window: window::Id,
    pies: Vec<Pie>
}
impl Model{
    fn spawn_pie(&mut self){
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
    Model { _window, pies: vec![] }
}

fn update(app: &App, model: &mut Model, update: Update) {
    for pie in model.pies.iter_mut(){
        pie.velocity -= PIE_ACCEL;
        pie.pos.y += pie.velocity;
    }
    model.pies.retain(|pie|{ pie.pos.y - PIE_RADIUS > -(SCREEN_HALF as f32)});
    if app.elapsed_frames() % PIE_SPAWN_RATE == 0{
        model.spawn_pie();
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    for pie in &model.pies{
        draw.ellipse()
            .radius(PIE_RADIUS)
            .color(PIE_BACKGROUND)
            .x_y(pie.pos.x, pie.pos.y);
    }
    draw.to_frame(app, &frame).unwrap();
}
