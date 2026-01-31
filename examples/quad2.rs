// QuadTree
//
// Quadtree is filled with random points.
// Move mouse to collect points in rectangular window around mouse pointer.
// Use mouse wheel to increase/decrease size of window.
// ESC to quit

use macroquad::prelude::*;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;

#[derive(Default)]
struct QTree {
   boundary: Rect,
   cap: usize,
   points: Vec<Vec2>,
   divided: bool,
   children: Option<Box<[QTree; 4]>>,
}

impl QTree {
   fn new(boundary: Rect, cap: usize) -> Self {
      Self {
         boundary,
         cap,
         ..Default::default()
      }
   }

   fn subdivide(&mut self) {
      let x = self.boundary.x;
      let y = self.boundary.y;
      let w = self.boundary.w;
      let h = self.boundary.h;
      self.children = Some(Box::new([
         QTree::new(Rect::new(x, y, w / 2., h / 2.), self.cap),
         QTree::new(Rect::new(x + w / 2., y, w / 2., h / 2.), self.cap),
         QTree::new(Rect::new(x, y + h / 2., w / 2., h / 2.), self.cap),
         QTree::new(Rect::new(x + w / 2., y + h / 2., w / 2., h / 2.), self.cap),
      ]));
      self.divided = true;
   }

   fn insert(&mut self, p: Vec2) -> bool {
      if !self.boundary.contains(p) {
         return false;
      }

      if self.points.len() < self.cap {
         self.points.push(p);
         return true;
      }

      if !self.divided {
         self.subdivide();
      }

      for c in self.children.as_mut().unwrap().iter_mut() {
         if c.insert(p) {
            return true;
         }
      }

      return false;
   }

   fn query(&self, r: &Rect, found: &mut Vec<Vec2>, queries: &mut usize) {
      *queries += 1;
      if self.boundary.intersect(*r).is_none() {
         return;
      }

      for p in &self.points {
         if r.contains(*p) {
            found.push(*p)
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
         self.boundary.x,
         self.boundary.y,
         self.boundary.w,
         self.boundary.h,
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
   let boundary: Rect = Rect::new(0., 0., WIDTH as f32, HEIGHT as f32);
   let mut qt = QTree::new(boundary, 4);
   let show_points = true;
   let mut collected: Vec<Vec2> = vec![];
   let mut window_size = 50.;
   let mut queries: usize = 0;

   // fill qt with points
   for _ in 0..500 {
      qt.insert(Vec2::new(
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
      // put mouse pointer in center of region
      let window = Rect {
         x: x - window_size / 2.,
         y: y - window_size / 2.,
         w: window_size as f32,
         h: window_size as f32,
      };

      qt.query(&window, &mut collected, &mut queries);

      clear_background(BLUE);
      draw_text(format!("Queries: {queries}").as_str(), 20., 20., 20., WHITE);
      qt.show(show_points);
      draw_rectangle_lines(
         x - window_size / 2.,
         y - window_size / 2.,
         window_size,
         window_size,
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
