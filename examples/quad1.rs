#![allow(dead_code)]

// QuadTree
//
// Left mouse button to produce points to fill the QT.
// SPACE toggles point visibility
// ESC to quit

use macroquad::prelude::*;

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

      for c in self.children.as_mut().unwrap().iter_mut() {
         if c.insert(p.clone()) {
            return true;
         }
      }

      return false;
   }

   fn show(&self, show_points: bool) {
      draw_rectangle_lines(
         self.boundary.x - self.boundary.w,
         self.boundary.y - self.boundary.h,
         self.boundary.w * 2.,
         self.boundary.h * 2.,
         1.,
         WHITE,
      );

      if show_points {
         for p in &self.points {
            draw_circle(p.x, p.y, 2., RED);
         }
      }

      if self.divided {
         for c in self.children.as_ref().unwrap().iter() {
            c.show(show_points);
         }
      }
   }
}

fn window_conf() -> Conf {
   Conf {
      window_title: "QuadTree".to_owned(),
      window_width: WIDTH,
      window_height: HEIGHT,
      ..Default::default()
   }
}

#[macroquad::main(window_conf)]
async fn main() {
   let boundary: Rect = Rect::new(300., 200., 300., 200.);
   let mut qt = QTree::new(boundary, 4);
   let mut show_points = true;

   loop {
      if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
         return;
      }
      if is_key_pressed(KeyCode::Space) {
         show_points = !show_points;
      }

      if is_mouse_button_down(MouseButton::Left) {
         let (x, y) = mouse_position();
         for _ in 0..4 {
            qt.insert(Point::new(
               x + rand::gen_range::<f32>(-10., 10.),
               y + rand::gen_range::<f32>(-10., 10.),
            ));
         }
      }

      clear_background(BLUE);
      qt.show(show_points);

      next_frame().await
   }
}
