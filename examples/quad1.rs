#![allow(dead_code)]

// QuadTree
//
// Left mouse button to produce points to fill the QT.
// SPACE toggles point visibility
// ESC to quit

use raylib::prelude::*;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;
const N: usize = 10;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

//     +---------+
//     |         |
//     |    *    | |
//     |  (x,y)  | | h
//     +---------+ |
//          ^^^^^^
//            w
struct Rect {
    x: f32, // x of center point
    y: f32, // y of center point
    w: f32, // center to left/right side
    h: f32, // center to top/bottom
}

impl Rect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= self.x - self.w
            && p.x < self.x + self.w
            && p.y >= self.y - self.h
            && p.y < self.y + self.h
    }

    fn intersects(&self, rhs: &Self) -> bool {
        !(rhs.x - rhs.w > self.x + self.w
            || rhs.x + rhs.w < self.x - self.w
            || rhs.y - rhs.h > self.y + self.h
            || rhs.y + rhs.h < self.y - self.h)
    }
}

struct QTree {
    boundary: Rect,
    cap: usize,
    points: Vec<Point>,
    divided: bool,
    children: Option<Box<[QTree; 4]>>,
}

impl QTree {
    fn new(boundary: Rect, cap: usize) -> Self {
        Self {
            boundary,
            cap,
            points: vec![],
            divided: false,
            children: None,
        }
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;
        self.children = Some(Box::new([
            // TODO rectangles are wrong!!!
            QTree::new(Rect::new(x + w / 2., y - h / 2., w / 2., h / 2.), self.cap),
            QTree::new(Rect::new(x - w / 2., y - h / 2., w / 2., h / 2.), self.cap),
            QTree::new(Rect::new(x + w / 2., y + h / 2., w / 2., h / 2.), self.cap),
            QTree::new(Rect::new(x - w / 2., y + h / 2., w / 2., h / 2.), self.cap),
        ]));
        self.divided = true;
    }

    fn insert(&mut self, p: Point) -> bool {
        if !self.boundary.contains(&p) {
            return false;
        }

        if self.points.len() < self.cap {
            self.points.push(p.clone());
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        for c in self
            .children
            .as_mut()
            .unwrap()
            .iter_mut()
        {
            if c.insert(p.clone()) {
                return true;
            }
        }

        return false;
    }

    fn show(&self, d: &mut RaylibDrawHandle, show_points: bool) {
        d.draw_rectangle_lines(
            (self.boundary.x - self.boundary.w) as i32,
            (self.boundary.y - self.boundary.h) as i32,
            self.boundary.w as i32 * 2,
            self.boundary.h as i32 * 2,
            Color::RAYWHITE,
        );

        if show_points {
            for p in &self.points {
                d.draw_circle(p.x as i32, p.y as i32, 2., Color::RED);
            }
        }

        if self.divided {
            for c in self
                .children
                .as_ref()
                .unwrap()
                .iter()
            {
                c.show(d, show_points);
            }
        }
    }
}

fn main() {
    let boundary: Rect = Rect::new(300., 200., 300., 200.);
    let mut qt = QTree::new(boundary, 4);
    let mut show_points = true;

    let (mut rl, thd) = raylib::init()
        .width(WIDTH)
        .height(HEIGHT)
        .title("")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            show_points = !show_points;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let pos = rl.get_mouse_position();
            for _ in 0..4 {
                qt.insert(Point::new(
                    pos.x + rand::random_range(-10..10) as f32,
                    pos.y + rand::random_range(-10..10) as f32,
                ));
            }
        }

        rl.draw(&thd, |mut d| {
            d.clear_background(Color::MIDNIGHTBLUE);
            d.draw_fps(20, 20);
            qt.show(&mut d, show_points);
        });
    }
}
