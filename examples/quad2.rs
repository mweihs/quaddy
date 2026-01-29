#![allow(dead_code)]

// QuadTree
//
// Quadtree is filled with random points.
// Move mouse to collect points in rectangular window around mouse pointer.
// Use mouse wheel to increase/decrease size of window.
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

   fn show(&self, d: &mut RaylibDrawHandle, show_points: bool) {
      d.draw_rectangle_lines(
         (self.boundary.x - self.boundary.w) as i32,
         (self.boundary.y - self.boundary.h) as i32,
         self.boundary.w as i32 * 2,
         self.boundary.h as i32 * 2,
         Color::GRAY,
      );

      if show_points {
         for p in &self.points {
            d.draw_circle(p.x as i32, p.y as i32, 2., Color::RED);
         }
      }

      if self.divided {
         for c in self.children.as_ref().unwrap().iter() {
            c.show(d, show_points);
         }
      }
   }
}

fn main() {
   let boundary: Rect = Rect::new(300., 200., 300., 200.);
   let mut qt = QTree::new(boundary, 4);
   let show_points = true;
   let mut collected: Vec<Point> = vec![];
   let mut window_size = 50;
   let mut queries: usize = 0;

   // fill qt with points
   for _ in 0..500 {
      qt.insert(Point::new(
         rand::random_range(50..WIDTH - 50) as f32,
         rand::random_range(50..HEIGHT - 50) as f32,
      ));
   }

   let (mut rl, thd) = raylib::init()
      .width(WIDTH)
      .height(HEIGHT)
      .title("")
      .log_level(TraceLogLevel::LOG_ERROR)
      .build();

   rl.set_target_fps(60);

   while !rl.window_should_close() {
      let zoom = rl.get_mouse_wheel_move();
      if zoom != 0. {
         window_size += zoom as i32 * 5;
      }

      let pos = rl.get_mouse_position();
      let rect: Rect = Rect {
         x: pos.x,
         y: pos.y,
         w: window_size as f32,
         h: window_size as f32,
      };

      qt.query(&rect, &mut collected, &mut queries);

      rl.draw(&thd, |mut d| {
         d.clear_background(Color::MIDNIGHTBLUE);
         d.hide_cursor();
         d.draw_text(format!("Queries: {queries}").as_str(), 20, 20, 20, Color::RAYWHITE);
         qt.show(&mut d, show_points);
         d.draw_rectangle_lines(
            pos.x as i32 - window_size,
            pos.y as i32 - window_size,
            window_size * 2,
            window_size * 2,
            Color::YELLOW,
         );
         for p in collected.iter() {
            d.draw_circle(p.x as i32, p.y as i32, 2., Color::LIGHTCYAN);
         }
      });

      collected.clear();
      queries = 0;
   }
}
