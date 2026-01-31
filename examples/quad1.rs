// QuadTree
//
// Left mouse button to produce points to fill the QT.
// SPACE toggles point visibility
// ESC or Q to quit

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

      if self.points.len() <= self.cap {
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

   fn show(&self, show_points: bool) {
      draw_rectangle_lines(
         self.boundary.x,
         self.boundary.y,
         self.boundary.w,
         self.boundary.h,
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
   let boundary: Rect = Rect::new(0., 0., WIDTH as f32, HEIGHT as f32);
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
            qt.insert(Vec2::new(
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
