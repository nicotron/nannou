// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com

// Seeking "vehicle" follows the mouse position

// Implements Craig Reynold's autonomous steering behaviors
// One vehicle "seeks"
// See: http://www.red3d.com/cwr/
extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model, event, view).run();
}

struct Model {
    vehicle: Vehicle,
}

struct Vehicle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    r: f32,
    // Maximum steering force
    max_force: f32,
    // Maximum speed
    max_speed: f32,
}

impl Vehicle {
    fn new(x: f32, y: f32) -> Self {
        let position = Vector2::new(x, y);
        let velocity = Vector2::new(0.0, -2.0);
        let acceleration = Vector2::new(0.0, 0.0);
        let r = 6.0;
        let max_force = 0.1;
        let max_speed = 4.0;

        Vehicle {
            position,
            velocity,
            acceleration,
            r,
            max_force,
            max_speed,
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        // We could add mass here if we want A = F / M
        self.acceleration += force;
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().with_dimensions(640, 360).build().unwrap();
    let middle = app.window_rect().xy();
    let vehicle = Vehicle::new(middle.x, middle.y);
    Model { vehicle }
}

fn event(app: &App, mut m: Model, event: Event) -> Model {
    {
        let Model {
            ref mut vehicle, ..
        } = m;
        let mouse = Vector2::new(app.mouse.x, app.mouse.y);
        // update gets called just before view every frame
        if let Event::Update(_update) = event {
            seek(vehicle, mouse);
        }
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    let mouse = Vector2::new(app.mouse.x, app.mouse.y);

    draw.ellipse()
        // Missing Stroke
        .x_y(mouse.x, mouse.y)
        .radius(48.0)
        .color(Rgb::new(0.78, 0.78, 0.78));

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}

// A method that calculates a steering force towards a target
// STEER = DESIRED MINUS VELOCITY
fn seek(vehicle: &mut Vehicle, target: Vector2) {
    let steer = {
        let Vehicle {
            ref position,
            ref velocity,
            ref max_speed,
            ref max_force,
            ..
        } = vehicle;
        // A vector pointing from the position to the target
        // Scale to maximum speed
        let desired = (target - *position).with_magnitude(*max_speed);

        // Steering = Desired minus velocity
        // Limit to maximum steering force
        (desired - *velocity).limit_magnitude(*max_force)
    };

    vehicle.apply_force(steer);
}
