use std::path::PathBuf;

use chrono::prelude::*;
use nannou::prelude::*;


fn main() {
    nannou::app(model).update(update).run();
}

struct Point {
    pub x: f32,
    pub y: f32,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl Point {
    fn new(x: f32, y:f32, boundary: Rect) -> Self {
        let min_x = boundary.left();
        let max_x = boundary.right();
        let min_y = boundary.bottom();
        let max_y = boundary.top();

        Point {
            x, 
            y, 
            min_x, 
            max_x, 
            min_y, 
            max_y,
        }
    }

    fn update(&mut self, app: &App) {
        let sin1 = (app.time / 1.4).sin();
        let sin2 = (app.time / 2.0).sin();
        self.x = map_range(sin1, -1.0, 1.0, self.min_x, self.max_x);
        self.y = map_range(sin2, -1.0, 1.0, self.min_y, self.max_y); 
    }
}

struct Node {
    center: Vec2,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    color: Srgba,
}

impl Node {
    fn new(center: Vec2, x: f32, y: f32, radius: f32) -> Self {
        Node {
            center,
            x,
            y,
            radius,
            color: srgba(1.0, 1.0, 1.0, 1.0),
        }
    }

    fn update(&mut self, target: &Point) {
        let rad = vec2(self.center.x, self.center.y).angle_between(vec2(target.x, target.y));
        self.x = self.center.x + rad.sin() * 30.0;
        self.y = self.center.y + rad.cos() * 30.0;

        // update radius
        let pos = vec2(self.x, self.y);
        let pos_target = vec2(target.x, target.y);
        let dist = pos.distance(pos_target);
        self.radius = 2.0 + (1000.0 / dist).min(50.0);

        // update color
        if dist < 100.0 {
            self.color = srgba(
                random_f32(),
                random_f32(),
                random_f32(),
                1.0
            );
        } else {
            self.color = srgba(1.0, 1.0, 1.0, 1.0);
        }
    }
}

struct Model {
    _window: window::Id,
    nodes: Vec<Node>,
    point: Point,
    out_path: PathBuf,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(1200, 1200).view(view).build().unwrap();

    // create point
    let point = Point::new(-50.0, 0.0, app.window_rect().pad(300.0));

    // create nodes
    let cols = 30;
    let rows = 30;
    let radius = 20.0;
    let nodes = create_nodes(rows, cols, app.window_rect(), &point, radius);

    // output path to save frames
    let out_path = create_output_path(app);

    Model {
        _window, 
        nodes, 
        point, 
        out_path
    }
}

fn create_output_path(app: &App) -> PathBuf {
    let time_str = Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("out")
        .join(time_str)
}

fn create_nodes(rows: usize, cols: usize, win: Rect, target: &Point, radius: f32) -> Vec<Node> {
    let win_p = win.pad(200.0);
    let x_gap = (win_p.right() - win_p.left()) / (cols as f32 - 1.0);
    let y_gap = (win_p.top() - win_p.bottom()) / (rows as f32 - 1.0);
    let mut nodes = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            let x_center = win_p.left() + row as f32 * x_gap;
            let y_center = win_p.bottom() + col as f32 * y_gap;
            let rad = vec2(x_center, y_center).angle_between(vec2(target.x, target.y));
            let x = x_center + rad.sin() * radius;
            let y = y_center + rad.cos() * radius;
            let node = Node::new(
                Vec2::new(x_center, y_center),
                x,
                y,
                radius
            );
            nodes.push(node);
        }
    }
    nodes
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.point.update(&app);
    for i in 0..model.nodes.len() {
        model.nodes[i].update(&model.point);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // draw.ellipse().x_y(model.point.x, model.point.y).radius(30.0).color(RED);
    model.nodes.iter().for_each(|node| {
        draw.ellipse()
            .x_y(node.x, node.y)
            .radius(node.radius)
            .color(node.color);
    });

    draw.to_frame(app, &frame).unwrap();

    // capture frames (max 1000 saved)
    // deactivate to make output smoother
    if frame.nth() < 1000 {
        let file_path = captured_frame_path(&model, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn captured_frame_path(model: &Model, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    model.out_path
        .join(format!("{:03}", frame.nth()))
        .with_extension("png")
}