#![allow(dead_code)]

// QuadTree
//
// Quadtree is filled with random points.
// Move mouse to collect points in rectangular window around mouse pointer.
// Use mouse wheel to increase/decrease size of window.
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

struct Circle {
   x: f32,
   y: f32,
   r: f32,
}

impl Circle {
   fn new(x: f32, y: f32, r: f32) -> Self {
      Self { x, y, r }
   }

   fn contains(&self, p: &Point) -> bool {
      let d = (p.x - self.x).powf(2.) + (p.y - self.y).powf(2.);
      d <= self.r * self.r
   }

   fn intersects(&self, region: &Rect) -> bool {
      let xdist = (region.x - self.x).abs();
      let ydist = (region.y - self.y).abs();
      let r = self.r;
      let w = region.w / 2.;
      let h = region.h / 2.;
      let edges = (xdist - w).powf(2.) + (ydist - h).powf(2.);

      if xdist > (r + w) || ydist > (r + h) {
         return false;
      }

      if xdist <= w || ydist <= h {
         return true;
      }

      return edges <= r * r;
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

   fn query(&self, r: &Rect, found: &mut Vec<Point>, queries: &mut usize) {
      *queries += 1;
      if !self.boundary.intersects(r) {
         return;
      }

      for p in &self.points {
         if r.contains(p) {
            found.push(p.clone())
         }
      }

      if self.divided {
         for c in self.children.as_ref().unwrap().iter() {
            c.query(r, found, queries);
         }
      }
   }

   fn show(&self, show_points: bool) {
      draw_rectangle_lines(
         self.boundary.x - self.boundary.w,
         self.boundary.y - self.boundary.h,
         self.boundary.w * 2.,
         self.boundary.h * 2.,
         1.,
         GRAY,
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
   let show_points = true;
   let mut collected: Vec<Point> = vec![];
   let mut window_size = 50.;
   let mut queries: usize = 0;

   // fill qt with points
   for _ in 0..500 {
      qt.insert(Point::new(
         rand::gen_range(50., WIDTH as f32 - 50.),
         rand::gen_range(50., HEIGHT as f32 - 50.),
      ));
   }

   loop {
      if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
         return;
      }
      let (_, zoom) = mouse_wheel();
      if zoom != 0. {
         window_size += zoom * 5.;
      }

      let (x, y) = mouse_position();
      let region = Rect {
         x,
         y,
         w: window_size as f32,
         h: window_size as f32,
      };

      qt.query(&region, &mut collected, &mut queries);

      clear_background(BLUE);
      draw_text(format!("Queries: {queries}").as_str(), 20., 20., 20., WHITE);
      qt.show(show_points);
      draw_rectangle_lines(
         x - window_size,
         y - window_size,
         window_size * 2.,
         window_size * 2.,
         2.,
         YELLOW,
      );
      // points in window
      for p in collected.iter() {
         draw_circle(p.x, p.y, 2., YELLOW);
      }

      collected.clear();
      queries = 0;

      next_frame().await
   }
}
